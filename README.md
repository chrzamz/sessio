<div align="center">

# Sessio

**Unified search & browse for your Claude Code and Codex CLI conversations.**

*One window for every conversation you've ever had with your AI coding assistants.*

[English](README.md) · [简体中文](README.zh-CN.md)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE-APACHE)
[![Built with Tauri](https://img.shields.io/badge/built%20with-Tauri%20v2-24C8DB?logo=tauri)](https://tauri.app/)
[![Rust](https://img.shields.io/badge/rust-stable-orange?logo=rust)](https://www.rust-lang.org/)
[![React 19](https://img.shields.io/badge/react-19-61DAFB?logo=react)](https://react.dev/)
[![TypeScript](https://img.shields.io/badge/typescript-5-3178C6?logo=typescript)](https://www.typescriptlang.org/)
[![Status: Pre-Alpha](https://img.shields.io/badge/status-pre--alpha-orange)](#status)
[![GitHub stars](https://img.shields.io/github/stars/chrzamz/sessio?style=social)](https://github.com/chrzamz/sessio)

</div>

---

## Why Sessio

If you use **Claude Code** or **Codex CLI** heavily, your conversation history lives as scattered JSONL files across many directories:

- Renaming a project folder fragments your history
- There's no way to search across tools, projects, or time
- You can't easily revisit "that one debugging conversation from two weeks ago"

**Sessio fixes that.** It indexes everything into a local SQLite database and gives you a clean three-pane UI to search, filter, and browse.

> 🔒 **Local-first. Read-only. Your data never leaves your machine.**

## Features

- 🔍 **Full-text search** across every conversation — SQLite FTS5, sub-100ms
- 🗂️ **Filter by tool & project** — Claude Code, Codex, or both, scoped to any directory
- 📜 **Timeline view** — user/assistant messages in a clean conversation thread
- ⭐ **Star important sessions** for instant access
- 🚫 **Never modifies source files** — read-only index over your existing JSONL
- 💻 **Local-first** — no network calls, no telemetry, no cloud
- 🪶 **Lightweight** — Tauri-powered, ~10MB binary, low memory footprint

## Screenshots

> Coming soon. The UI is a three-pane layout: sidebar (filters) · session list · session detail.

## Tech Stack

| Layer | Tech |
|---|---|
| Desktop runtime | [Tauri v2](https://tauri.app/) |
| Frontend | React 19 · TypeScript · Vite · TailwindCSS |
| Backend | Rust · rusqlite · walkdir · chrono |
| Storage | SQLite (FTS5) — index layer only, original JSONL untouched |

## Install

> Pre-built binaries coming soon. For now, build from source.

### Prerequisites

- Node 20+ and [pnpm](https://pnpm.io/)
- [Rust toolchain](https://rustup.rs/)
- Platform-specific [Tauri requirements](https://tauri.app/v2/start/prerequisites/) (macOS / Windows / Linux)

### Build from source

```bash
git clone https://github.com/chrzamz/sessio.git
cd sessio
pnpm install

# Run in dev mode
pnpm tauri dev

# Or build a production binary
pnpm tauri build
```

## Usage

1. Launch Sessio
2. Click **Scan & Index** — it discovers and parses every JSONL under `~/.claude/projects` and `~/.codex/sessions`
3. Browse, filter, search

Re-run **Scan & Index** anytime to pick up new conversations. (Incremental indexing is on the roadmap.)

## Architecture

Sessio is structured as a read-only index over your existing conversation files:

```
~/.claude/projects/*/   ─┐
                         ├─→  Rust scanner  →  SQLite (sessions + messages + FTS5)  →  React UI
~/.codex/sessions/*/*    ─┘
```

- **Scanner** walks both directories and identifies JSONL files
- **Parser** (per-tool: `claude.rs`, `codex.rs`) extracts messages, timestamps, project metadata
- **Indexer** upserts into SQLite — original files are never modified
- **Frontend** queries via Tauri commands; FTS5 powers search

See [CLAUDE.md](CLAUDE.md) for deeper architectural notes.

## Roadmap

See [PROGRESS.md](PROGRESS.md) for the full plan.

- [x] **Phase 1–3** — Scaffold, scanner, indexer, FTS5, basic three-pane UI
- [ ] **Phase 4** — Project aliases (fix renamed-folder history loss), incremental indexing, starring, subagent collapse, perf
- [ ] **Phase 5** — Collaboration profile (rule-based): tool roles, project depth, work patterns
- [ ] **Phase 6** — AI-powered session insights & long-term persona analysis (opt-in)

## Status

🚧 **Pre-alpha.** Actively developed and used daily by the author. Expect breaking changes, schema migrations, and rough edges.

If you try it and something doesn't work, [open an issue](https://github.com/chrzamz/sessio/issues) — early feedback is gold.

## Contributing

Issues and PRs welcome. The codebase is small and approachable:

- Start with [CLAUDE.md](CLAUDE.md) for architecture and decisions
- Check [PROGRESS.md](PROGRESS.md) for what's planned and what's open
- Run `pnpm tauri dev` and tinker

## FAQ

**Does Sessio modify my Claude Code / Codex files?**
No. Sessio only reads. Your original JSONL files are never touched.

**Does Sessio call any external API?**
No. Everything happens locally. There is no telemetry, no analytics, no cloud sync.

**What about other AI coding tools (Cursor, Aider, Windsurf)?**
Not yet. The parser layer is designed to be extended — contributions welcome.

**Where is the index stored?**
`~/Library/Application Support/sessio/index.db` on macOS (and platform equivalents elsewhere). Delete this file to reset.

## License

Licensed under either of:

- Apache License, Version 2.0 — [LICENSE-APACHE](LICENSE-APACHE)
- MIT License — [LICENSE-MIT](LICENSE-MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this work shall be dual-licensed as above, without any additional terms or conditions.

## Acknowledgments

Sessio reads conversation files produced by:

- [Claude Code](https://claude.com/claude-code) by Anthropic
- [Codex CLI](https://github.com/openai/codex) by OpenAI

Sessio is an independent project and is not affiliated with or endorsed by either company.
