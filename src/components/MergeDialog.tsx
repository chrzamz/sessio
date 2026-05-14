import { useEffect, useMemo, useState } from "react";
import type { ProjectInfo } from "../types";

interface MergeDialogProps {
  source: ProjectInfo;
  candidates: ProjectInfo[];
  onConfirm: (canonicalDir: string) => Promise<void> | void;
  onCancel: () => void;
}

export function MergeDialog({ source, candidates, onConfirm, onCancel }: MergeDialogProps) {
  const [query, setQuery] = useState("");
  const [selectedDir, setSelectedDir] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);

  const filtered = useMemo(() => {
    const q = query.trim().toLowerCase();
    if (!q) return candidates;
    return candidates.filter((c) => {
      const name = c.project_name?.toLowerCase() ?? "";
      const dir = c.project_dir?.toLowerCase() ?? "";
      return name.includes(q) || dir.includes(q);
    });
  }, [candidates, query]);

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") onCancel();
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [onCancel]);

  const handleConfirm = async () => {
    if (!selectedDir || submitting) return;
    setSubmitting(true);
    try {
      await onConfirm(selectedDir);
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/40"
      onClick={onCancel}
    >
      <div
        className="bg-white rounded-lg shadow-xl w-[480px] max-h-[80vh] flex flex-col"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="p-4 border-b border-zinc-200">
          <div className="text-sm font-medium text-zinc-900">Merge project into…</div>
          <div className="mt-1 text-xs text-zinc-500 truncate" title={source.project_dir ?? ""}>
            <span className="line-through">{source.project_name ?? "unknown"}</span>
            <span className="ml-1">({source.session_count} sessions)</span>
          </div>
        </div>

        <div className="p-3 border-b border-zinc-200">
          <input
            autoFocus
            type="text"
            placeholder="Filter active projects…"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            className="w-full px-2 py-1.5 text-sm border border-zinc-200 rounded focus:outline-none focus:border-zinc-400"
          />
        </div>

        <div className="flex-1 overflow-y-auto p-2">
          {filtered.length === 0 ? (
            <div className="text-xs text-zinc-400 text-center py-6">No active projects match.</div>
          ) : (
            filtered.map((c) => (
              <button
                key={c.project_dir ?? ""}
                onClick={() => setSelectedDir(c.project_dir)}
                className={`block w-full text-left px-2 py-1.5 text-sm rounded ${
                  selectedDir === c.project_dir
                    ? "bg-zinc-900 text-white"
                    : "hover:bg-zinc-100 text-zinc-700"
                }`}
                title={c.project_dir ?? ""}
              >
                <div className="truncate">{c.project_name ?? "unknown"}</div>
                <div
                  className={`truncate text-xs ${
                    selectedDir === c.project_dir ? "text-zinc-300" : "text-zinc-400"
                  }`}
                >
                  {c.project_dir} · {c.session_count} sessions
                </div>
              </button>
            ))
          )}
        </div>

        <div className="p-3 border-t border-zinc-200 flex justify-end gap-2">
          <button
            onClick={onCancel}
            className="px-3 py-1.5 text-sm text-zinc-600 hover:bg-zinc-100 rounded"
          >
            Cancel
          </button>
          <button
            onClick={handleConfirm}
            disabled={!selectedDir || submitting}
            className="px-3 py-1.5 text-sm bg-zinc-900 text-white rounded hover:bg-zinc-700 disabled:opacity-40"
          >
            {submitting ? "Merging…" : "Merge"}
          </button>
        </div>
      </div>
    </div>
  );
}
