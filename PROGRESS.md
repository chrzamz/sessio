# Sessio - 进度跟踪

## 当前阶段: Phase 4 - 完善

## Phase 1: 项目骨架
- [x] 项目初始化（Tauri v2 + React + TypeScript）
- [x] 项目文档（CLAUDE.md / PROGRESS.md / README.md）
- [x] Rust 后端模块结构（parser / scanner / db / commands）
- [x] Rust 依赖配置（rusqlite / walkdir / chrono）
- [x] 前端依赖配置（TailwindCSS）
- [x] 前端组件目录结构（Sidebar / SessionList / SessionDetail）
- [x] jj 版本控制初始化

## Phase 2: 后端 — 扫描 & 索引
- [x] SQLite schema 创建（sessions / messages / FTS5）
- [x] Claude Code JSONL 解析器
- [x] Codex JSONL 解析器
- [x] 目录扫描器（发现所有 JSONL 文件）
- [x] 索引器（扫描 + 解析 + 写入 DB）
- [x] Tauri commands 暴露给前端

## Phase 3: 前端 — 基础 UI
- [x] 三栏布局（侧边栏 / 列表 / 详情）
- [x] 对话列表组件
- [x] 对话详情组件（消息气泡）
- [x] 搜索栏
- [x] 工具/项目过滤

## Phase 4: 完善（按优先级排序）

> 实施顺序：项目别名 → 增量索引 → 收藏 → 子代理折叠 → 性能 → e2e。
> 前两项对日常自用收益最高，先做。

- [ ] **项目别名（解决目录改名后历史"丢失"）**
  - 场景：`project-kanban` 改名为 `push-log` 后，老 JSONL 还在原编码目录，新会话写入新编码目录，UI 上变成两个独立的"项目"，老的还会被标 ⊘
  - 方案 A（手动合并）：新表 `project_aliases (alias_dir, canonical_dir)`；右键 ⊘ 项目 → "合并到..." → 选活项目；侧栏隐藏 alias，session 数滚到 canonical；筛选时 alias = canonical
- [ ] 增量索引（按 mtime + size 跳过未变化的 JSONL）
- [ ] 收藏标记
- [ ] 子代理折叠
- [ ] 性能优化（大文件分页加载）
- [ ] 端到端测试

## Phase 5: 协作画像 — 规则驱动版（无 LLM 依赖）

> 参考 `_docs/ai-collaboration-persona-direction.md` Phase A。
> 目标：基于现有 SQLite 数据 + 关键词规则做第一版"工作行为画像"，
> 验证用户是否觉得有趣，再决定是否进 Phase 6。
> 严格边界：不做心理诊断，所有结论可解释、可点回会话。

- [ ] 工具协作画像（Claude / Codex 使用分布、平均深度、长会话比例）
- [ ] 项目热度 vs 深度图（高频浅层 vs 低频高浓度）
- [ ] 任务类型分布（产品 / 技术 / 写作 / 调试 / 自动化，关键词聚类）
- [ ] 风险卡片（多线程切换、结构延迟收尾倾向，规则版）
- [ ] 优势卡片（规则版）
- [ ] Persona Tab 页面 + 可解释性（每条结论可点回会话）
- [ ] 简版协作人格报告（低置信度 MBTI 猜测，明确标注"娱乐性近似"）

## Phase 6: AI Insight 抽取层（远期，仅记录不开工）

> 参考画像方向文档 Phase B/C。需 Phase 5 验证用户喜欢后才启动。
> 涉及 LLM API 调用 + 异步批处理 + token 成本控制。

- session_insights 表 + 单 session 结构化提炼
- persona_snapshots 表 + 跨时间窗聚合
- project_insights 表 + 项目级长期分析

## 明确不做（保持 MVP 边界）

- 不做实时监听
- 不做情绪分析
- 不做严肃心理学测评
- 不做云同步
- 不做对话编辑/删除
- 不修改原始 JSONL 文件

## 变更日志

### 2026-04-21
- 侧边栏标识失效目录：`ProjectInfo` 增加 `dir_exists`，后端通过 `Path::exists()` 判定，前端用 `⊘` + 删除线 + 半透明区分

### 2026-04-01
- Phase 1 全部完成，TS + Rust 编译通过，`pnpm tauri build` 产出 .app + .dmg
- Phase 2 & 3 代码已编写并通过编译，待实际运行验证

### 2026-03-31
- 项目初始化，Tauri v2 + React + TypeScript 模板创建
- 创建项目文档（CLAUDE.md / PROGRESS.md / README.md）
