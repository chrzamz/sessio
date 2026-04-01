# Sessio - 协作文档

## 项目目标

统一管理 Claude Code 和 Codex CLI 的对话历史。扫描本地 JSONL 文件，建立 SQLite 索引，提供全文搜索、按项目过滤、收藏标记等功能。解决对话散落多目录、改名后丢失、无法回溯的痛点。开源发布，先自用，再吸引同好。

## 技术选型

- 框架：Tauri v2（桌面应用）
- 前端：React 19 + TypeScript + Vite + TailwindCSS
- 后端：Rust
- 存储：SQLite（rusqlite，索引层）+ 原始 JSONL（只读，不修改）
- 包管理：pnpm

## 架构决策记录

- **为什么用 SQLite 而非直接读 JSONL**：450+ 个 JSONL 全量解析太慢，SQLite FTS5 可以做到 <100ms 全文搜索
- **为什么不修改原始文件**：这是其他工具的数据，只做索引读取，不动原始数据
- **为什么不用 Electron**：Tauri 包体小（~10MB vs ~150MB），内存占用低，Rust 后端处理 JSONL 解析更快
- **为什么先做 Claude Code + Codex**：自己在用这两个，格式已摸清，后续通过 parser trait 扩展其他工具

## 数据源

### Claude Code (`~/.claude/projects/`)
- 目录名 = 路径编码（`/Users/foo/bar` → `-Users-foo-bar`）
- 消息格式：`type: "user"` / `"assistant"` / `"file-history-snapshot"`
- 消息内容在 `message.content`（string 或 content blocks list）
- 元数据：`uuid`, `timestamp`, `cwd`, `sessionId`
- 子代理在 `subagents/` 目录下

### Codex CLI (`~/.codex/sessions/`)
- 按日期分目录：`sessions/2026/03/31/rollout-{ts}-{uuid}.jsonl`
- 消息格式：`type: "session_meta"` / `"response_item"` / `"event_msg"`
- 用户消息：`response_item` → `payload.item.role == "user"` + `type == "message"`
- 需过滤：permissions 注入、AGENTS.md 注入等系统内容

## 范围边界

### MVP 做什么
- [ ] 扫描 ~/.claude 和 ~/.codex，解析 JSONL，写入 SQLite 索引
- [ ] 对话列表（时间排序，显示工具/项目/摘要/时间/大小）
- [ ] 对话详情（user/assistant 消息时间线）
- [ ] 全文搜索（SQLite FTS5）
- [ ] 按工具、按项目过滤
- [ ] 收藏标记

### 明确不做
- 不修改原始 JSONL 文件
- 不做对话编辑/删除功能
- 不做云同步
- 不做 AI 摘要（MVP 阶段）
- 不做实时监听（MVP 手动刷新）
- 不做其他工具支持（Cursor/Aider 等后续扩展）

### 后续可能做（但现在不做）
- 文件系统监听（notify crate），新对话自动入库
- AI 自动生成对话标题
- 目录迁移修复（检测路径变更后的孤立会话）
- Cursor / Aider / Windsurf 支持
- 系统托盘常驻 + 快捷搜索
- 导出对话为 Markdown
- 标签系统

## 协作约定

1. **改架构先讨论**：涉及新增依赖、拆分文件、改存储方案等，先说方案，确认再动手
2. **单步确认**：每完成一个功能模块，更新进度，再进行下一个
3. **最小实现**：不加"以防万一"的代码，不提前优化，不过度抽象
4. **超范围提醒**：如果请求超出 MVP 范围，提醒我
5. **VCS 用 jj**：所有版本控制操作使用 jj，不用 git

## 进度跟踪

进度维护在 [PROGRESS.md](PROGRESS.md) 中。

- 标记规范：`[ ]` 待做 / `[~]` 进行中 / `[x]` 已完成 / `[!]` 受阻
- 每完成一个任务立即更新，不攒批
- 每次工作结束在变更日志中追加当日摘要
