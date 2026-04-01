use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

use super::{ParsedMessage, ParsedSession, SessionParser};

pub struct ClaudeParser;

impl SessionParser for ClaudeParser {
    fn can_parse(&self, path: &Path) -> bool {
        path.extension().map_or(false, |ext| ext == "jsonl")
            && path
                .to_string_lossy()
                .contains(".claude/projects/")
    }

    fn parse(&self, path: &Path) -> Result<ParsedSession> {
        let file = fs::File::open(path).context("Failed to open JSONL file")?;
        let reader = BufReader::new(file);
        let file_size = fs::metadata(path)?.len();

        let mut messages = Vec::new();
        let mut first_user_msg: Option<String> = None;
        let mut first_ts: Option<String> = None;
        let mut last_ts: Option<String> = None;
        let mut user_msg_count: u32 = 0;
        let mut msg_count: u32 = 0;

        let system_reminder_re = Regex::new(r"<system-reminder>[\s\S]*?</system-reminder>").unwrap();

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

            // Skip non-message types
            if msg_type == "file-history-snapshot" {
                continue;
            }

            let timestamp = data["timestamp"].as_str().map(String::from);
            if let Some(ref ts) = timestamp {
                if first_ts.is_none() {
                    first_ts = Some(ts.clone());
                }
                last_ts = Some(ts.clone());
            }

            if msg_type == "user" || msg_type == "assistant" {
                msg_count += 1;
                let content = extract_content(&data["message"], &system_reminder_re);

                if msg_type == "user" && !content.is_empty() {
                    user_msg_count += 1;
                    if first_user_msg.is_none() {
                        first_user_msg = Some(content.chars().take(200).collect());
                    }
                }

                if !content.is_empty() {
                    messages.push(ParsedMessage {
                        id: data["uuid"].as_str().map(String::from),
                        role: msg_type.to_string(),
                        content,
                        timestamp: timestamp.clone(),
                        line_number: line_num as u32,
                    });
                }
            }
        }

        // Extract project info from path
        let (project_dir, project_name, is_subagent) = extract_project_info(path);

        // Session ID = file stem
        let id = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();

        Ok(ParsedSession {
            id,
            tool: "claude-code".to_string(),
            source_path: path.to_string_lossy().to_string(),
            project_dir,
            project_name,
            first_message: first_user_msg,
            message_count: msg_count,
            user_message_count: user_msg_count,
            file_size,
            created_at: first_ts,
            updated_at: last_ts,
            is_subagent,
            messages,
        })
    }
}

fn extract_content(message: &serde_json::Value, re: &Regex) -> String {
    if message.is_null() {
        return String::new();
    }

    // message can be a dict with "content" field
    if let Some(obj) = message.as_object() {
        if let Some(content) = obj.get("content") {
            if let Some(s) = content.as_str() {
                let cleaned = re.replace_all(s, "").trim().to_string();
                return cleaned;
            }
            if let Some(arr) = content.as_array() {
                let texts: Vec<String> = arr
                    .iter()
                    .filter_map(|b| {
                        if b["type"].as_str() == Some("text") {
                            b["text"].as_str().map(String::from)
                        } else {
                            None
                        }
                    })
                    .collect();
                let joined = texts.join(" ");
                let cleaned = re.replace_all(&joined, "").trim().to_string();
                return cleaned;
            }
        }
    }

    String::new()
}

fn extract_project_info(path: &Path) -> (Option<String>, Option<String>, bool) {
    let path_str = path.to_string_lossy();

    // Check if subagent
    let is_subagent = path_str.contains("/subagents/");

    // Extract the project directory name from the path
    // e.g., ~/.claude/projects/-Users-alice-Workspace-Projects-XinLi/xxx.jsonl
    // → project folder = -Users-alice-Workspace-Projects-XinLi
    if let Some(parent) = path.parent() {
        let folder_name = parent
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();

        if folder_name == "subagents" {
            return (None, Some("subagents".to_string()), true);
        }

        // Decode folder name: -Users-alice-Workspace-Projects-XinLi → /Users/alice/Workspace/Projects/XinLi
        let decoded = decode_claude_path(&folder_name);
        let project_name = decoded
            .split('/')
            .filter(|s| !s.is_empty())
            .last()
            .map(String::from)
            .or_else(|| Some(folder_name.clone()));

        return (Some(decoded), project_name, is_subagent);
    }

    (None, None, is_subagent)
}

fn decode_claude_path(encoded: &str) -> String {
    // -Users-alice-Workspace → /Users/alice/Workspace
    if encoded.starts_with('-') {
        encoded.replacen('-', "/", 1).replace('-', "/")
    } else {
        encoded.replace('-', "/")
    }
}
