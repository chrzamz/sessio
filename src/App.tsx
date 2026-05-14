import { useState, useEffect, useCallback, useRef } from "react";
import { Sidebar } from "./components/Sidebar";
import { SessionList } from "./components/SessionList";
import { SessionDetail } from "./components/SessionDetail";
import { MergeDialog } from "./components/MergeDialog";
import { useSessions } from "./hooks/useSessions";
import type { SessionSummary, Stats, ProjectInfo } from "./types";

function App() {
  const {
    sessions,
    loadSessions,
    scanAndIndex,
    search,
    toggleStar,
    getStats,
    getProjects,
    mergeProject,
  } = useSessions();

  const [stats, setStats] = useState<Stats | null>(null);
  const [projects, setProjects] = useState<ProjectInfo[]>([]);
  const [selectedSession, setSelectedSession] = useState<SessionSummary | null>(null);
  const [toolFilter, setToolFilter] = useState<string | null>(null);
  const [projectFilter, setProjectFilter] = useState<string | null>(null);
  const [starredOnly, setStarredOnly] = useState(false);
  const [searchQuery, setSearchQuery] = useState("");
  const [scanning, setScanning] = useState(false);
  const [toast, setToast] = useState<{ msg: string; kind: "success" | "error" } | null>(null);
  const toastTimerRef = useRef<ReturnType<typeof window.setTimeout> | null>(null);
  const [mergeSource, setMergeSource] = useState<ProjectInfo | null>(null);

  const refreshData = useCallback(async () => {
    await loadSessions({
      tool: toolFilter,
      project: projectFilter,
      starred_only: starredOnly,
      hide_subagents: true,
    });
    const [s, p] = await Promise.all([getStats(), getProjects()]);
    setStats(s);
    setProjects(p);
  }, [toolFilter, projectFilter, starredOnly]);

  useEffect(() => {
    refreshData();
  }, [refreshData]);

  useEffect(() => {
    return () => {
      if (toastTimerRef.current !== null) {
        window.clearTimeout(toastTimerRef.current);
      }
    };
  }, []);

  const handleScan = async () => {
    setScanning(true);
    try {
      const result = await scanAndIndex();
      setToast({
        msg: `扫描完成：${result.parsed}/${result.total_files} 个会话，耗时 ${(result.duration_ms / 1000).toFixed(1)}s`,
        kind: "success",
      });
      await refreshData();
    } catch (e) {
      setToast({ msg: `扫描失败：${String(e)}`, kind: "error" });
    } finally {
      setScanning(false);
      if (toastTimerRef.current !== null) {
        window.clearTimeout(toastTimerRef.current);
      }
      toastTimerRef.current = window.setTimeout(() => {
        setToast(null);
        toastTimerRef.current = null;
      }, 3000);
    }
  };

  const handleSearch = async () => {
    if (!searchQuery.trim()) {
      await refreshData();
      return;
    }
    await search(searchQuery);
  };

  const handleToggleStar = async (id: string) => {
    await toggleStar(id);
    await refreshData();
  };

  const handleMergeConfirm = async (canonicalDir: string) => {
    if (!mergeSource?.project_dir) return;
    const aliasDir = mergeSource.project_dir;
    try {
      await mergeProject(aliasDir, canonicalDir);
      if (projectFilter === aliasDir) {
        setProjectFilter(canonicalDir);
      }
      setMergeSource(null);
      await refreshData();
      setToast({ msg: "项目已合并", kind: "success" });
    } catch (e) {
      setToast({ msg: `合并失败：${String(e)}`, kind: "error" });
    } finally {
      if (toastTimerRef.current !== null) {
        window.clearTimeout(toastTimerRef.current);
      }
      toastTimerRef.current = window.setTimeout(() => {
        setToast(null);
        toastTimerRef.current = null;
      }, 3000);
    }
  };

  return (
    <div className="flex h-screen bg-white text-zinc-900">
      <Sidebar
        stats={stats}
        projects={projects}
        toolFilter={toolFilter}
        projectFilter={projectFilter}
        starredOnly={starredOnly}
        onToolFilter={setToolFilter}
        onProjectFilter={setProjectFilter}
        onStarredOnly={setStarredOnly}
        onScan={handleScan}
        scanning={scanning}
        onMergeRequest={setMergeSource}
      />
      <SessionList
        sessions={sessions}
        selectedId={selectedSession?.id ?? null}
        onSelect={setSelectedSession}
        onToggleStar={handleToggleStar}
        searchQuery={searchQuery}
        onSearchChange={setSearchQuery}
        onSearch={handleSearch}
      />
      <SessionDetail session={selectedSession} />
      {mergeSource && (
        <MergeDialog
          source={mergeSource}
          candidates={projects.filter(
            (p) => p.dir_exists && p.project_dir && p.project_dir !== mergeSource.project_dir,
          )}
          onConfirm={handleMergeConfirm}
          onCancel={() => setMergeSource(null)}
        />
      )}
      {toast && (
        <div
          role="status"
          aria-live="polite"
          className={`fixed bottom-4 right-4 px-4 py-2 rounded shadow-lg text-sm text-white ${
            toast.kind === "success" ? "bg-zinc-900" : "bg-red-600"
          }`}
        >
          {toast.msg}
        </div>
      )}
    </div>
  );
}

export default App;
