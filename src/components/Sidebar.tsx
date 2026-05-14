import type { ProjectInfo } from "../types";

interface SidebarProps {
  stats: { total_sessions: number; claude_sessions: number; codex_sessions: number } | null;
  projects: ProjectInfo[];
  toolFilter: string | null;
  projectFilter: string | null;
  starredOnly: boolean;
  onToolFilter: (tool: string | null) => void;
  onProjectFilter: (project: string | null) => void;
  onStarredOnly: (v: boolean) => void;
  onScan: () => void;
  scanning: boolean;
  onMergeRequest: (source: ProjectInfo) => void;
}

export function Sidebar({
  stats,
  projects,
  toolFilter,
  projectFilter,
  starredOnly,
  onToolFilter,
  onProjectFilter,
  onStarredOnly,
  onScan,
  scanning,
  onMergeRequest,
}: SidebarProps) {
  return (
    <div className="w-56 shrink-0 border-r border-zinc-200 bg-zinc-50 flex flex-col h-full overflow-y-auto">
      <div className="p-3 border-b border-zinc-200">
        <button
          onClick={onScan}
          disabled={scanning}
          className="w-full px-3 py-1.5 text-sm bg-zinc-900 text-white rounded hover:bg-zinc-700 disabled:opacity-50 flex items-center justify-center gap-2"
        >
          {scanning && (
            <svg
              className="animate-spin h-3.5 w-3.5"
              viewBox="0 0 24 24"
              fill="none"
              aria-hidden="true"
            >
              <circle
                className="opacity-25"
                cx="12"
                cy="12"
                r="10"
                stroke="currentColor"
                strokeWidth="4"
              />
              <path
                className="opacity-75"
                fill="currentColor"
                d="M4 12a8 8 0 018-8v4a4 4 0 00-4 4H4z"
              />
            </svg>
          )}
          <span>{scanning ? "Scanning..." : "Scan & Index"}</span>
        </button>
        {stats && (
          <div className="mt-2 text-xs text-zinc-500 space-y-0.5">
            <div>{stats.total_sessions} sessions</div>
            <div>CC: {stats.claude_sessions} / Codex: {stats.codex_sessions}</div>
          </div>
        )}
      </div>

      <div className="p-3 border-b border-zinc-200">
        <div className="text-xs font-medium text-zinc-400 uppercase mb-2">Tool</div>
        <div className="space-y-1">
          {[
            { label: "All", value: null },
            { label: "Claude Code", value: "claude-code" },
            { label: "Codex", value: "codex" },
          ].map((item) => (
            <button
              key={item.label}
              onClick={() => onToolFilter(item.value)}
              className={`block w-full text-left px-2 py-1 text-sm rounded ${
                toolFilter === item.value
                  ? "bg-zinc-200 text-zinc-900 font-medium"
                  : "text-zinc-600 hover:bg-zinc-100"
              }`}
            >
              {item.label}
            </button>
          ))}
        </div>
      </div>

      <div className="p-3 border-b border-zinc-200">
        <label className="flex items-center gap-2 text-sm text-zinc-600 cursor-pointer">
          <input
            type="checkbox"
            checked={starredOnly}
            onChange={(e) => onStarredOnly(e.target.checked)}
            className="rounded"
          />
          Starred only
        </label>
      </div>

      <div className="p-3 flex-1 overflow-y-auto">
        <div className="text-xs font-medium text-zinc-400 uppercase mb-2">Projects</div>
        <div className="space-y-0.5">
          <button
            onClick={() => onProjectFilter(null)}
            className={`block w-full text-left px-2 py-1 text-sm rounded truncate ${
              !projectFilter
                ? "bg-zinc-200 text-zinc-900 font-medium"
                : "text-zinc-600 hover:bg-zinc-100"
            }`}
          >
            All projects
          </button>
          {projects.map((p) => (
            <div
              key={p.project_dir ?? ""}
              className={`group flex items-center rounded ${
                projectFilter === p.project_dir
                  ? "bg-zinc-200"
                  : "hover:bg-zinc-100"
              } ${!p.dir_exists ? "opacity-60" : ""}`}
            >
              <button
                onClick={() => onProjectFilter(p.project_dir || "")}
                className={`flex-1 min-w-0 text-left px-2 py-1 text-xs truncate ${
                  projectFilter === p.project_dir
                    ? "text-zinc-900 font-medium"
                    : "text-zinc-500"
                }`}
                title={
                  p.dir_exists === false
                    ? `${p.project_dir ?? ""} (directory no longer exists)`
                    : p.project_dir || ""
                }
              >
                {!p.dir_exists && <span className="mr-1" aria-label="missing directory">⊘</span>}
                <span className={!p.dir_exists ? "line-through" : ""}>
                  {p.project_name || "unknown"}
                </span>{" "}
                <span className="text-zinc-400">({p.session_count})</span>
              </button>
              {!p.dir_exists && (
                <button
                  onClick={() => onMergeRequest(p)}
                  className="px-1.5 py-1 text-xs text-zinc-400 hover:text-zinc-900 opacity-0 group-hover:opacity-100"
                  title="Merge into another project"
                  aria-label="Merge into another project"
                >
                  →
                </button>
              )}
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
