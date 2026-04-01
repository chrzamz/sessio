import { invoke } from "@tauri-apps/api/core";
import { useState, useCallback } from "react";
import type {
  SessionSummary,
  MessageDetail,
  SearchResult,
  ScanResult,
  Stats,
  ProjectInfo,
  SessionFilter,
} from "../types";

export function useSessions() {
  const [sessions, setSessions] = useState<SessionSummary[]>([]);
  const [loading, setLoading] = useState(false);

  const loadSessions = useCallback(async (filter: SessionFilter) => {
    setLoading(true);
    try {
      const result = await invoke<SessionSummary[]>("list_sessions", {
        filter,
      });
      setSessions(result);
    } finally {
      setLoading(false);
    }
  }, []);

  const getMessages = useCallback(async (sessionId: string) => {
    return invoke<MessageDetail[]>("get_session_messages", {
      sessionId,
    });
  }, []);

  const scanAndIndex = useCallback(async () => {
    return invoke<ScanResult>("scan_and_index");
  }, []);

  const search = useCallback(async (query: string) => {
    return invoke<SearchResult[]>("search", { query });
  }, []);

  const toggleStar = useCallback(async (sessionId: string) => {
    return invoke<boolean>("toggle_star", { sessionId });
  }, []);

  const getStats = useCallback(async () => {
    return invoke<Stats>("get_stats");
  }, []);

  const getProjects = useCallback(async () => {
    return invoke<ProjectInfo[]>("get_projects");
  }, []);

  return {
    sessions,
    loading,
    loadSessions,
    getMessages,
    scanAndIndex,
    search,
    toggleStar,
    getStats,
    getProjects,
  };
}
