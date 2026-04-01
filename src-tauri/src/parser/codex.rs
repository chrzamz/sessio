use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

use super::{ParsedMessage, ParsedSession, SessionParser};

pub struct CodexParser;

impl SessionParser for CodexParser {
    fn can_parse(&self, path: &Path) -> bool {
        path.extension().map_or(false, |ext| ext == "jsonl")
            && path.to_string_lossy().contains(".codex/")
            && !path
                .file_name()
                .map_or(false, |n| n.to_string_lossy().contains("index"))
    }

    fn parse(&self, path: &Path) -> Result<ParsedSession> {
        let file = fs::File::open(path).context("Failed to open JSONL file")?;
        let reader = BufReader::new(file);
        let file_size = fs::metadata(path)?.len();

        let mut messages = Vec::new();
        let mut cwd: Option<String> = None;
        let mut first_user_msg: Option<String> = None;
        let mut first_ts: Option<String> = None;
        let mut last_ts: Option<String> = None;
        let mut user_msg_count: u32 = 0;
        let mut msg_count: u32 = 0;

        let system_re = Regex::new(r"<system-reminder>[\s\S]*?</system-reminder>").unwrap();

        for (line_num, line) in reader.lines().enumerate() {
            let line = match line {
                Ok(l) => l,
                Err(_) => continue,
            };
            let line = line.trim().to_string();
            if line.is_empty() {
                continue;
            }

            let data: serde_json::Value = match serde_json::from_str(&line) {
                Ok(v) => v,
                Err(_) => continue,
            };

            let msg_type = data["type"].as_str().unwrap_or("");
            let payload = &data["payload"];

            let timestamp = data["timestamp"].as_str().map(String::from);
            if let Some(ref ts) = timestamp {
                if first_ts.is_none() {
                    first_ts = Some(ts.clone());
                }
                last_ts = Some(ts.clone());
            }

            match msg_type {
                "session_meta" => {
                    if cwd.is_none() {
                        cwd = payload["cwd"].as_str().map(String::from);
                    }
                }
                "response_item" => {
                    let item = if payload["item"].is_object() {
                        &payload["item"]
                    } else {
                        payload
                    };

                    let role = item["role"].as_str().unwrap_or("");
                    let item_type = item["type"].as_str().unwrap_or("");

                    if item_type != "message" {
                        continue;
                    }

                    let content = extract_codex_content(&item["content"], &system_re);

                    // Skip system injections
                    if role == "user" || role == "developer" {
                        if content.starts_with("<permissions")
                            || content.starts_with("# AGENTS.md")
                            || content.starts_with("<environment")
                            || content.starts_with("<user_instructions")
                            || content.starts_with("You create concise run metadata")
                        {
                            continue;
                        }
                    }

                    if (role == "user" || role == "assistant") && !content.is_empty() {
                        msg_count += 1;

                        if role == "user" {
                            user_msg_count += 1;
                            if first_user_msg.is_none() {
                                first_user_msg = Some(content.chars().take(200).collect());
                            }
                        }

                        messages.push(ParsedMessage {
                            id: None,
                            role: role.to_string(),
                            content,
                            timestamp: timestamp.clone(),
                            line_number: line_num as u32,
                        });
                    }
                }
                _ => {}
            }
        }

        // Extract project name from cwd
        let project_name = cwd.as_ref().and_then(|d| {
            d.split('/')
                .filter(|s| !s.is_empty())
                .last()
                .map(String::from)
        });

        let _is_archived = path.to_string_lossy().contains("archived_sessions");

        let id = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();

        Ok(ParsedSession {
            id,
            tool: "codex".to_string(),
            source_path: path.to_string_lossy().to_string(),
            project_dir: cwd,
            project_name,
            first_message: first_user_msg,
            message_count: msg_count,
            user_message_count: user_msg_count,
            file_size,
            created_at: first_ts,
            updated_at: last_ts,
            is_subagent: false,
            messages,
        })
    }
}

fn extract_codex_content(content: &serde_json::Value, re: &Regex) -> String {
    if let Some(s) = content.as_str() {
        let cleaned = re.replace_all(s, "").trim().to_string();
        return cleaned;
    }

    if let Some(arr) = content.as_array() {
        let texts: Vec<String> = arr
            .iter()
            .filter_map(|b| {
                if let Some(obj) = b.as_object() {
                    let btype = obj
                        .get("type")
                        .and_then(|t| t.as_str())
                        .unwrap_or("");
                    if btype == "input_text" || btype == "text" || btype == "output_text" {
                        return obj.get("text").and_then(|t| t.as_str()).map(String::from);
                    }
                }
                None
            })
            .collect();
        let joined = texts.join(" ");
        let cleaned = re.replace_all(&joined, "").trim().to_string();
        return cleaned;
    }

    String::new()
}
