export interface SessionSummary {
  id: string;
  tool: string;
  source_path: string;
  project_dir: string | null;
  project_name: string | null;
  first_message: string | null;
  message_count: number;
  user_message_count: number;
  file_size: number;
  created_at: string | null;
  updated_at: string | null;
  is_subagent: boolean;
  starred: boolean;
  tags: string | null;
}

export interface MessageDetail {
  id: string | null;
  role: string;
  content: string;
  timestamp: string | null;
  line_number: number;
}

export interface SearchResult {
  session_id: string;
  role: string;
  content: string;
  timestamp: string | null;
  tool: string;
  project_name: string | null;
  session_first_message: string | null;
  snippet: string;
}

export interface ScanResult {
  total_files: number;
  parsed: number;
  failed: number;
  duration_ms: number;
}

export interface Stats {
  total_sessions: number;
  total_messages: number;
  claude_sessions: number;
  codex_sessions: number;
}

export interface ProjectInfo {
  project_dir: string | null;
  project_name: string | null;
  session_count: number;
  dir_exists: boolean;
}

export interface ProjectAlias {
  alias_dir: string;
  canonical_dir: string;
  created_at: string;
}

export interface SessionFilter {
  tool: string | null;
  project: string | null;
  starred_only: boolean | null;
  hide_subagents: boolean | null;
}
