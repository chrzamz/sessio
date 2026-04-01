use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::parser::claude::ClaudeParser;
use crate::parser::codex::CodexParser;
use crate::parser::{ParsedSession, SessionParser};

pub struct Scanner {
    claude_parser: ClaudeParser,
    codex_parser: CodexParser,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            claude_parser: ClaudeParser,
            codex_parser: CodexParser,
        }
    }

    pub fn discover_sessions(&self) -> Vec<PathBuf> {
        let home = dirs::home_dir().unwrap_or_default();
        let mut files = Vec::new();

        // Scan Claude Code sessions
        let claude_dir = home.join(".claude").join("projects");
        if claude_dir.exists() {
            for entry in WalkDir::new(&claude_dir)
                .max_depth(2)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "jsonl") {
                    files.push(path.to_path_buf());
                }
            }
        }

        // Scan Codex sessions
        let codex_sessions_dir = home.join(".codex").join("sessions");
        if codex_sessions_dir.exists() {
            for entry in WalkDir::new(&codex_sessions_dir)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "jsonl") {
                    files.push(path.to_path_buf());
                }
            }
        }

        // Scan Codex archived sessions
        let codex_archived_dir = home.join(".codex").join("archived_sessions");
        if codex_archived_dir.exists() {
            for entry in WalkDir::new(&codex_archived_dir)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "jsonl") {
                    files.push(path.to_path_buf());
                }
            }
        }

        files
    }

    pub fn parse_session(&self, path: &Path) -> Result<ParsedSession> {
        if self.claude_parser.can_parse(path) {
            self.claude_parser.parse(path)
        } else if self.codex_parser.can_parse(path) {
            self.codex_parser.parse(path)
        } else {
            anyhow::bail!("No parser found for: {}", path.display())
        }
    }
}
