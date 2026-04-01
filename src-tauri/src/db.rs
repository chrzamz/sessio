use anyhow::{Context, Result};
use rusqlite::Connection;
use std::path::Path;
use std::sync::Mutex;

use crate::parser::ParsedSession;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new(db_path: &Path) -> Result<Self> {
        let conn = Connection::open(db_path).context("Failed to open SQLite database")?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                tool TEXT NOT NULL,
                source_path TEXT NOT NULL UNIQUE,
                project_dir TEXT,
                project_name TEXT,
                first_message TEXT,
                message_count INTEGER DEFAULT 0,
                user_message_count INTEGER DEFAULT 0,
                file_size INTEGER DEFAULT 0,
                created_at TEXT,
                updated_at TEXT,
                is_subagent BOOLEAN DEFAULT 0,
                tags TEXT DEFAULT '[]',
                starred BOOLEAN DEFAULT 0,
                indexed_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS messages (
                rowid INTEGER PRIMARY KEY AUTOINCREMENT,
                id TEXT,
                session_id TEXT NOT NULL REFERENCES sessions(id),
                role TEXT NOT NULL,
                content TEXT,
                timestamp TEXT,
                line_number INTEGER
            );

            CREATE VIRTUAL TABLE IF NOT EXISTS messages_fts USING fts5(
                content,
                content=messages,
                content_rowid=rowid
            );

            CREATE TRIGGER IF NOT EXISTS messages_ai AFTER INSERT ON messages BEGIN
                INSERT INTO messages_fts(rowid, content) VALUES (new.rowid, new.content);
            END;

            CREATE TRIGGER IF NOT EXISTS messages_ad AFTER DELETE ON messages BEGIN
                INSERT INTO messages_fts(messages_fts, rowid, content) VALUES('delete', old.rowid, old.content);
            END;

            CREATE INDEX IF NOT EXISTS idx_sessions_tool ON sessions(tool);
            CREATE INDEX IF NOT EXISTS idx_sessions_project ON sessions(project_dir);
            CREATE INDEX IF NOT EXISTS idx_sessions_updated ON sessions(updated_at DESC);
            CREATE INDEX IF NOT EXISTS idx_messages_session ON messages(session_id);
            ",
        )
        .context("Failed to initialize database schema")?;

        Ok(())
    }

    pub fn upsert_session(&self, session: &ParsedSession) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO sessions (id, tool, source_path, project_dir, project_name, first_message, message_count, user_message_count, file_size, created_at, updated_at, is_subagent, indexed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
             ON CONFLICT(source_path) DO UPDATE SET
                first_message = excluded.first_message,
                message_count = excluded.message_count,
                user_message_count = excluded.user_message_count,
                file_size = excluded.file_size,
                updated_at = excluded.updated_at,
                indexed_at = excluded.indexed_at",
            rusqlite::params![
                session.id,
                session.tool,
                session.source_path,
                session.project_dir,
                session.project_name,
                session.first_message,
                session.message_count,
                session.user_message_count,
                session.file_size,
                session.created_at,
                session.updated_at,
                session.is_subagent,
                now,
            ],
        )?;

        // Delete old messages for this session, then insert new ones
        conn.execute(
            "DELETE FROM messages WHERE session_id = ?1",
            rusqlite::params![session.id],
        )?;

        let mut stmt = conn.prepare(
            "INSERT INTO messages (id, session_id, role, content, timestamp, line_number) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        )?;

        for msg in &session.messages {
            stmt.execute(rusqlite::params![
                msg.id,
                session.id,
                msg.role,
                msg.content,
                msg.timestamp,
                msg.line_number,
            ])?;
        }

        Ok(())
    }

    pub fn list_sessions(
        &self,
        tool_filter: Option<&str>,
        project_filter: Option<&str>,
        starred_only: bool,
        hide_subagents: bool,
    ) -> Result<Vec<SessionSummary>> {
        let conn = self.conn.lock().unwrap();
        let mut sql = String::from(
            "SELECT id, tool, source_path, project_dir, project_name, first_message, message_count, user_message_count, file_size, created_at, updated_at, is_subagent, starred, tags
             FROM sessions WHERE 1=1",
        );
        let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        if let Some(tool) = tool_filter {
            sql.push_str(" AND tool = ?");
            params.push(Box::new(tool.to_string()));
        }
        if let Some(project) = project_filter {
            sql.push_str(" AND project_dir LIKE ?");
            params.push(Box::new(format!("%{}%", project)));
        }
        if starred_only {
            sql.push_str(" AND starred = 1");
        }
        if hide_subagents {
            sql.push_str(" AND is_subagent = 0");
        }

        sql.push_str(" ORDER BY updated_at DESC");

        let params_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params_refs.as_slice(), |row| {
            Ok(SessionSummary {
                id: row.get(0)?,
                tool: row.get(1)?,
                source_path: row.get(2)?,
                project_dir: row.get(3)?,
                project_name: row.get(4)?,
                first_message: row.get(5)?,
                message_count: row.get(6)?,
                user_message_count: row.get(7)?,
                file_size: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
                is_subagent: row.get(11)?,
                starred: row.get(12)?,
                tags: row.get(13)?,
            })
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn search(&self, query: &str) -> Result<Vec<SearchResult>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT m.session_id, m.role, m.content, m.timestamp, s.tool, s.project_name, s.first_message,
                    snippet(messages_fts, 0, '<mark>', '</mark>', '...', 48) as snippet
             FROM messages_fts
             JOIN messages m ON messages_fts.rowid = m.rowid
             JOIN sessions s ON m.session_id = s.id
             WHERE messages_fts MATCH ?1
             ORDER BY rank
             LIMIT 50",
        )?;

        let rows = stmt.query_map(rusqlite::params![query], |row| {
            Ok(SearchResult {
                session_id: row.get(0)?,
                role: row.get(1)?,
                content: row.get(2)?,
                timestamp: row.get(3)?,
                tool: row.get(4)?,
                project_name: row.get(5)?,
                session_first_message: row.get(6)?,
                snippet: row.get(7)?,
            })
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn get_session_messages(&self, session_id: &str) -> Result<Vec<MessageDetail>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, role, content, timestamp, line_number FROM messages WHERE session_id = ?1 ORDER BY line_number",
        )?;

        let rows = stmt.query_map(rusqlite::params![session_id], |row| {
            Ok(MessageDetail {
                id: row.get(0)?,
                role: row.get(1)?,
                content: row.get(2)?,
                timestamp: row.get(3)?,
                line_number: row.get(4)?,
            })
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn toggle_star(&self, session_id: &str) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE sessions SET starred = NOT starred WHERE id = ?1",
            rusqlite::params![session_id],
        )?;

        let starred: bool = conn.query_row(
            "SELECT starred FROM sessions WHERE id = ?1",
            rusqlite::params![session_id],
            |row| row.get(0),
        )?;

        Ok(starred)
    }

    pub fn get_stats(&self) -> Result<Stats> {
        let conn = self.conn.lock().unwrap();

        let total_sessions: u32 = conn.query_row("SELECT COUNT(*) FROM sessions", [], |row| row.get(0))?;
        let total_messages: u32 = conn.query_row("SELECT COUNT(*) FROM messages", [], |row| row.get(0))?;
        let claude_sessions: u32 = conn.query_row(
            "SELECT COUNT(*) FROM sessions WHERE tool = 'claude-code'",
            [],
            |row| row.get(0),
        )?;
        let codex_sessions: u32 = conn.query_row(
            "SELECT COUNT(*) FROM sessions WHERE tool = 'codex'",
            [],
            |row| row.get(0),
        )?;

        Ok(Stats {
            total_sessions,
            total_messages,
            claude_sessions,
            codex_sessions,
        })
    }

    pub fn get_projects(&self) -> Result<Vec<ProjectInfo>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT project_dir, MIN(project_name), COUNT(*) as count
             FROM sessions
             WHERE project_dir IS NOT NULL AND is_subagent = 0
             GROUP BY project_dir
             ORDER BY count DESC",
        )?;

        let rows = stmt.query_map([], |row| {
            let project_dir: Option<String> = row.get(0)?;
            let dir_exists = project_dir
                .as_deref()
                .map(|d| Path::new(d).exists())
                .unwrap_or(true);
            Ok(ProjectInfo {
                project_dir,
                project_name: row.get(1)?,
                session_count: row.get(2)?,
                dir_exists,
            })
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SessionSummary {
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
    pub starred: bool,
    pub tags: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchResult {
    pub session_id: String,
    pub role: String,
    pub content: String,
    pub timestamp: Option<String>,
    pub tool: String,
    pub project_name: Option<String>,
    pub session_first_message: Option<String>,
    pub snippet: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MessageDetail {
    pub id: Option<String>,
    pub role: String,
    pub content: String,
    pub timestamp: Option<String>,
    pub line_number: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Stats {
    pub total_sessions: u32,
    pub total_messages: u32,
    pub claude_sessions: u32,
    pub codex_sessions: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProjectInfo {
    pub project_dir: Option<String>,
    pub project_name: Option<String>,
    pub session_count: u32,
    pub dir_exists: bool,
}
