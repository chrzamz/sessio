use serde::{Deserialize, Serialize};
use tauri::{Manager, State};

use crate::db::{Database, MessageDetail, ProjectAlias, ProjectInfo, SearchResult, SessionSummary, Stats};

#[derive(Debug, Deserialize)]
pub struct SessionFilter {
    pub tool: Option<String>,
    pub project: Option<String>,
    pub starred_only: Option<bool>,
    pub hide_subagents: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ScanResult {
    pub total_files: u32,
    pub parsed: u32,
    pub failed: u32,
    pub duration_ms: u64,
}

#[tauri::command]
pub async fn scan_and_index(app: tauri::AppHandle) -> Result<ScanResult, String> {
    tauri::async_runtime::spawn_blocking(move || -> Result<ScanResult, String> {
        let db = app.state::<Database>();
        let scanner = crate::scanner::Scanner::new();
        let files = scanner.discover_sessions();
        let total_files = files.len() as u32;

        let start = std::time::Instant::now();
        let mut parsed: u32 = 0;
        let mut failed: u32 = 0;

        for path in &files {
            match scanner.parse_session(path) {
                Ok(session) => {
                    if let Err(e) = db.upsert_session(&session) {
                        eprintln!("Failed to index {}: {}", path.display(), e);
                        failed += 1;
                    } else {
                        parsed += 1;
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse {}: {}", path.display(), e);
                    failed += 1;
                }
            }
        }

        let duration_ms = start.elapsed().as_millis() as u64;

        Ok(ScanResult {
            total_files,
            parsed,
            failed,
            duration_ms,
        })
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub fn list_sessions(
    filter: SessionFilter,
    db: State<'_, Database>,
) -> Result<Vec<SessionSummary>, String> {
    db.list_sessions(
        filter.tool.as_deref(),
        filter.project.as_deref(),
        filter.starred_only.unwrap_or(false),
        filter.hide_subagents.unwrap_or(true),
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_session_messages(
    session_id: String,
    db: State<'_, Database>,
) -> Result<Vec<MessageDetail>, String> {
    db.get_session_messages(&session_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn search(query: String, db: State<'_, Database>) -> Result<Vec<SearchResult>, String> {
    db.search(&query).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_star(session_id: String, db: State<'_, Database>) -> Result<bool, String> {
    db.toggle_star(&session_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_stats(db: State<'_, Database>) -> Result<Stats, String> {
    db.get_stats().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_projects(db: State<'_, Database>) -> Result<Vec<ProjectInfo>, String> {
    db.get_projects().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn merge_project(
    alias_dir: String,
    canonical_dir: String,
    db: State<'_, Database>,
) -> Result<(), String> {
    db.merge_project(&alias_dir, &canonical_dir)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn unmerge_project(alias_dir: String, db: State<'_, Database>) -> Result<(), String> {
    db.unmerge_project(&alias_dir).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_aliases(db: State<'_, Database>) -> Result<Vec<ProjectAlias>, String> {
    db.get_aliases().map_err(|e| e.to_string())
}
