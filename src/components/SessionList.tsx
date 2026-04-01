import type { SessionSummary } from "../types";

interface SessionListProps {
  sessions: SessionSummary[];
  selectedId: string | null;
  onSelect: (session: SessionSummary) => void;
  onToggleStar: (id: string) => void;
  searchQuery: string;
  onSearchChange: (q: string) => void;
  onSearch: () => void;
}

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes}B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(0)}KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)}MB`;
}

function formatDate(ts: string | null): string {
  if (!ts) return "";
  const d = new Date(ts);
  const now = new Date();
  const diffDays = Math.floor((now.getTime() - d.getTime()) / (1000 * 60 * 60 * 24));
  if (diffDays === 0) return d.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
  if (diffDays < 7) return `${diffDays}d ago`;
  return d.toLocaleDateString([], { month: "short", day: "numeric" });
}

function toolIcon(tool: string): string {
  return tool === "claude-code" ? "CC" : "CX";
}

export function SessionList({
  sessions,
  selectedId,
  onSelect,
  onToggleStar,
  searchQuery,
  onSearchChange,
  onSearch,
}: SessionListProps) {
  return (
    <div className="w-80 shrink-0 border-r border-zinc-200 flex flex-col h-full">
      <div className="p-2 border-b border-zinc-200">
        <div className="flex gap-1">
          <input
            type="text"
            value={searchQuery}
            onChange={(e) => onSearchChange(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && onSearch()}
            placeholder="Search conversations..."
            className="flex-1 px-2 py-1.5 text-sm border border-zinc-200 rounded bg-white focus:outline-none focus:border-zinc-400"
          />
          <button
            onClick={onSearch}
            className="px-2 py-1.5 text-sm border border-zinc-200 rounded hover:bg-zinc-50"
          >
            Go
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto">
        {sessions.length === 0 && (
          <div className="p-4 text-sm text-zinc-400 text-center">
            No conversations. Click "Scan & Index" to start.
          </div>
        )}
        {sessions.map((s) => (
          <div
            key={s.id}
            onClick={() => onSelect(s)}
            className={`px-3 py-2.5 border-b border-zinc-100 cursor-pointer hover:bg-zinc-50 ${
              selectedId === s.id ? "bg-blue-50 border-l-2 border-l-blue-500" : ""
            }`}
          >
            <div className="flex items-center gap-1.5 mb-1">
              <span
                className={`text-[10px] font-mono px-1 rounded ${
                  s.tool === "claude-code"
                    ? "bg-orange-100 text-orange-700"
                    : "bg-green-100 text-green-700"
                }`}
              >
                {toolIcon(s.tool)}
              </span>
              <span className="text-xs text-zinc-400 truncate flex-1">
                {s.project_name || "home"}
              </span>
              <span className="text-xs text-zinc-400">{formatDate(s.updated_at)}</span>
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  onToggleStar(s.id);
                }}
                className="text-xs"
              >
                {s.starred ? "\u2605" : "\u2606"}
              </button>
            </div>
            <div className="text-sm text-zinc-800 line-clamp-2 leading-snug">
              {s.first_message || "(empty conversation)"}
            </div>
            <div className="text-xs text-zinc-400 mt-1">
              {s.user_message_count} msgs &middot; {formatSize(s.file_size)}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
