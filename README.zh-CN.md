<div align="center">

# Sessio

**统一搜索与浏览你的 Claude Code 与 Codex CLI 对话记录。**

*一个窗口，看完你和 AI 编程助手聊过的所有内容。*

[English](README.md) · [简体中文](README.zh-CN.md)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE-APACHE)
[![Built with Tauri](https://img.shields.io/badge/built%20with-Tauri%20v2-24C8DB?logo=tauri)](https://tauri.app/)
[![Rust](https://img.shields.io/badge/rust-stable-orange?logo=rust)](https://www.rust-lang.org/)
[![React 19](https://img.shields.io/badge/react-19-61DAFB?logo=react)](https://react.dev/)
[![TypeScript](https://img.shields.io/badge/typescript-5-3178C6?logo=typescript)](https://www.typescriptlang.org/)
[![Status: Pre-Alpha](https://img.shields.io/badge/status-pre--alpha-orange)](#状态)
[![GitHub stars](https://img.shields.io/github/stars/chrzamz/sessio?style=social)](https://github.com/chrzamz/sessio)

</div>

---

## 为什么需要 Sessio

如果你重度使用 **Claude Code** 或 **Codex CLI**，对话历史散落在多个目录的 JSONL 文件里：

- 项目目录改名后，历史就被切断
- 没有跨工具、跨项目、跨时间的搜索能力
- 想回顾"两周前那次调试讨论"几乎不可能

**Sessio 解决这个问题。** 它把所有对话索引进本地 SQLite，提供干净的三栏界面来搜索、过滤、浏览。

> 🔒 **本地优先。只读。你的数据从不离开你的电脑。**

## 功能

- 🔍 **全文搜索** — 跨所有对话，基于 SQLite FTS5，亚 100ms 响应
- 🗂️ **按工具与项目过滤** — Claude Code、Codex 或两者，可限定到任意目录
- 📜 **时间线视图** — user / assistant 消息的清晰对话线
- ⭐ **收藏标记** 重要会话，快速回访
- 🚫 **从不修改原始文件** — 只读索引，你的 JSONL 完全不动
- 💻 **本地优先** — 无网络调用、无遥测、无云端
- 🪶 **轻量** — Tauri 驱动，约 10MB 安装包，低内存占用

## 截图

> 即将上线。UI 为三栏布局：侧边栏（筛选）· 会话列表 · 会话详情。

## 技术栈

| 层级 | 技术 |
|---|---|
| 桌面运行时 | [Tauri v2](https://tauri.app/) |
| 前端 | React 19 · TypeScript · Vite · TailwindCSS |
| 后端 | Rust · rusqlite · walkdir · chrono |
| 存储 | SQLite (FTS5) — 仅索引层，原始 JSONL 不动 |

## 安装

> 预编译版本即将提供，目前请从源码构建。

### 环境要求

- Node 20+ 与 [pnpm](https://pnpm.io/)
- [Rust 工具链](https://rustup.rs/)
- 各平台 [Tauri 依赖](https://tauri.app/v2/start/prerequisites/)（macOS / Windows / Linux）

### 从源码构建

```bash
git clone https://github.com/chrzamz/sessio.git
cd sessio
pnpm install

# 开发模式
pnpm tauri dev

# 或构建生产版本
pnpm tauri build
```

## 使用

1. 启动 Sessio
2. 点击 **Scan & Index** — 自动发现并解析 `~/.claude/projects` 与 `~/.codex/sessions` 下所有 JSONL
3. 浏览、过滤、搜索

随时可以重新点击 **Scan & Index** 来收录新对话。（增量索引已在 roadmap 上。）

## 架构

Sessio 是建立在你已有对话文件之上的只读索引层：

```
~/.claude/projects/*/   ─┐
                         ├─→  Rust 扫描器  →  SQLite (sessions + messages + FTS5)  →  React UI
~/.codex/sessions/*/*    ─┘
```

- **扫描器** 遍历两个目录，识别所有 JSONL 文件
- **解析器**（按工具：`claude.rs`、`codex.rs`）提取消息、时间戳、项目元数据
- **索引器** upsert 到 SQLite — 原始文件从不被修改
- **前端** 通过 Tauri commands 查询；FTS5 驱动搜索

更多架构细节见 [CLAUDE.md](CLAUDE.md)。

## 路线图

完整计划见 [PROGRESS.md](PROGRESS.md)。

- [x] **Phase 1–3** — 项目骨架、扫描器、索引器、FTS5、基础三栏 UI
- [ ] **Phase 4** — 项目别名（修复目录改名后历史"丢失"）、增量索引、收藏、子代理折叠、性能优化
- [ ] **Phase 5** — 协作画像（规则版）：工具角色分工、项目深度、工作模式
- [ ] **Phase 6** — AI 驱动的会话洞察 & 长期人格画像（可选开启）

## 状态

🚧 **Pre-alpha。** 作者每天在用并持续迭代。预计会有破坏性变更、schema 迁移、各种粗糙之处。

试用时遇到问题，欢迎 [提 issue](https://github.com/chrzamz/sessio/issues) — 早期反馈最有价值。

## 贡献

Issue 与 PR 都欢迎。代码量不大，结构清晰：

- 先看 [CLAUDE.md](CLAUDE.md) 了解架构与设计决策
- 看 [PROGRESS.md](PROGRESS.md) 了解计划与待办
- 跑 `pnpm tauri dev` 试一下

## FAQ

**Sessio 会修改我的 Claude Code / Codex 文件吗？**
不会。Sessio 只读。原始 JSONL 文件从不被触碰。

**Sessio 会调用外部 API 吗？**
不会。所有事都在本地发生。无遥测、无统计、无云同步。

**支持其他 AI 编程工具吗（Cursor、Aider、Windsurf）？**
暂未支持。解析层被设计成可扩展的 — 欢迎 PR。

**索引存在哪里？**
macOS 下是 `~/Library/Application Support/sessio/index.db`（其他平台对应位置）。删除该文件即可重置。

## 许可证

双协议授权，任选其一：

- Apache License, Version 2.0 — [LICENSE-APACHE](LICENSE-APACHE)
- MIT License — [LICENSE-MIT](LICENSE-MIT)

除非你明确声明，任何提交到本项目的贡献都将以上述双协议授权，无需额外条款。

## 致谢

Sessio 读取以下工具产生的对话文件：

- [Claude Code](https://claude.com/claude-code) by Anthropic
- [Codex CLI](https://github.com/openai/codex) by OpenAI

Sessio 是独立项目，与上述任何公司无关联、无背书关系。
