import { useEffect, useState } from "react";
import type { MessageDetail, SessionSummary } from "../types";
import { invoke } from "@tauri-apps/api/core";

interface SessionDetailProps {
  session: SessionSummary | null;
}

export function SessionDetail({ session }: SessionDetailProps) {
  const [messages, setMessages] = useState<MessageDetail[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (!session) {
      setMessages([]);
      return;
    }
    setLoading(true);
    invoke<MessageDetail[]>("get_session_messages", {
      sessionId: session.id,
    })
      .then(setMessages)
      .finally(() => setLoading(false));
  }, [session?.id]);

  if (!session) {
    return (
      <div className="flex-1 flex items-center justify-center text-zinc-400">
        Select a conversation
      </div>
    );
  }

  return (
    <div className="flex-1 flex flex-col h-full overflow-hidden">
      <div className="p-3 border-b border-zinc-200 bg-white shrink-0">
        <div className="text-sm font-medium text-zinc-800 line-clamp-1">
          {session.first_message || "(empty)"}
        </div>
        <div className="text-xs text-zinc-400 mt-1 flex gap-3">
          <span>
            {session.tool === "claude-code" ? "Claude Code" : "Codex"}
          </span>
          <span>{session.project_name || "home"}</span>
          <span>{session.message_count} messages</span>
          <span>{session.updated_at ? new Date(session.updated_at).toLocaleString() : ""}</span>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto p-4 space-y-3">
        {loading && <div className="text-sm text-zinc-400">Loading messages...</div>}
        {messages.map((msg, i) => (
          <div
            key={i}
            className={`flex ${msg.role === "user" ? "justify-end" : "justify-start"}`}
          >
            <div
              className={`max-w-[85%] px-3 py-2 rounded-lg text-sm whitespace-pre-wrap break-words ${
                msg.role === "user"
                  ? "bg-blue-500 text-white"
                  : "bg-zinc-100 text-zinc-800"
              }`}
            >
              <div className="text-xs opacity-60 mb-1">
                {msg.role === "user" ? "You" : "AI"}
                {msg.timestamp && (
                  <span className="ml-2">
                    {new Date(msg.timestamp).toLocaleTimeString([], {
                      hour: "2-digit",
                      minute: "2-digit",
                    })}
                  </span>
                )}
              </div>
              {msg.content.length > 2000
                ? msg.content.slice(0, 2000) + "\n\n... (truncated)"
                : msg.content}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
