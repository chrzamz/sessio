use anyhow::Result;
use serde::{Deserialize, Serialize};

pub mod claude;
pub mod codex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedSession {
    pub id: String,
    pub tool: String,
    pub source_path: String,
    pub project_dir: Option<String>,
    pub project_name: Option<String>,
    pub first_message: Option<String>,
    pub message_count: u32,
    pub user_message_count: u32,
    pub file_size: u64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub is_subagent: bool,
    pub messages: Vec<ParsedMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedMessage {
    pub id: Option<String>,
    pub role: String,
    pub content: String,
    pub timestamp: Option<String>,
    pub line_number: u32,
}

pub trait SessionParser {
    fn can_parse(&self, path: &std::path::Path) -> bool;
    fn parse(&self, path: &std::path::Path) -> Result<ParsedSession>;
}
