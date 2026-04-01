# Sessio

> Unified search & browse for your **Claude Code** and **Codex CLI** conversations.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE-APACHE)
[![Built with Tauri](https://img.shields.io/badge/built%20with-Tauri%20v2-24C8DB)](https://tauri.app/)

Sessio is a local-first desktop app that scans your local `~/.claude` and `~/.codex` directories, indexes every conversation into SQLite, and gives you full-text search, project filtering, and a clean timeline view across every chat you've ever had with these tools.

**Local-only. Read-only. Your data never leaves your machine.**

---

## Why

If you use Claude Code or Codex CLI heavily, your conversation history lives as scattered JSONL files across many directories. Renaming a project folder fragments your history. There's no easy way to search across tools, projects, or time.

Sessio fixes that.

## Features

- 🔍 **Full-text search** across every conversation (SQLite FTS5)
- 🗂️ **Filter by tool** (Claude Code / Codex) and project
- 📜 **Timeline view** of user/assistant messages per session
- ⭐ **Star important sessions** to find them later
- 🚫 **Never modifies source files** — read-only index over your existing JSONL
- 💻 **Local-first** — no network calls, no telemetry

## Tech Stack

- **Tauri v2** — small, fast desktop runtime
- **React 19 + TypeScript + Vite + TailwindCSS** — frontend
- **Rust + rusqlite** — backend & indexing
- **SQLite FTS5** — search

## Install

> Pre-built binaries coming soon. For now, build from source.

### From source

```bash
# Prerequisites: Node 20+, pnpm, Rust toolchain
git clone https://github.com/chrzamz/sessio.git
cd sessio
pnpm install
pnpm tauri dev      # run in dev mode
pnpm tauri build    # produce .app / .dmg / .exe
```

## Usage

1. Launch Sessio
2. Click **Scan & Index** — it'll discover and parse every JSONL under `~/.claude/projects` and `~/.codex/sessions`
3. Browse, filter, search

Re-run **Scan & Index** anytime to pick up new conversations.

## Roadmap

See [PROGRESS.md](PROGRESS.md) for the full plan. Highlights:

- [x] Phase 1–3 — Scaffold, scanner, indexer, FTS5, basic UI
- [ ] Phase 4 — Project aliases (fix renamed-folder history loss), incremental indexing, stars, perf
- [ ] Phase 5 — **Collaboration profile** (rule-based): how you actually work with AI, tool roles, project depth, risk patterns
- [ ] Phase 6 — AI-powered session insights & long-term persona analysis (opt-in)

## Status

🚧 **Pre-alpha.** Actively developed, used daily by the author. Expect breaking changes.

## Contributing

Issues and PRs welcome. The codebase is small and approachable — start with [CLAUDE.md](CLAUDE.md) and [PROGRESS.md](PROGRESS.md) to understand the architecture and where things are heading.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this work shall be dual-licensed as above, without any additional terms or conditions.

## Acknowledgments

Sessio reads conversation files produced by:

- [Claude Code](https://claude.com/claude-code) by Anthropic
- [Codex CLI](https://github.com/openai/codex) by OpenAI

Sessio is not affiliated with or endorsed by either.
