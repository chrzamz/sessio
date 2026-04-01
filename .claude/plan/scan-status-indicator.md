# 📋 实施计划：Scan & Index 状态指示器

## 任务目标

为 "Scan & Index" 按钮增加**非阻塞式**的加载状态反馈与完成提示，改善 MVP 阶段的用户体验。

## 任务类型

- [x] 前端（主要）
- [x] 后端（推荐：最小改动解除主线程阻塞）
- [ ] 全栈

## 关键背景（来自上下文检索与 codex 分析）

1. **数据规模**：本机 `~/.claude` + `~/.codex` 共 **627 个 JSONL / 164MB**，scan 耗时可达数秒
2. **重要技术事实**：当前 `#[tauri::command] fn scan_and_index(...)` 在 Tauri v2 中被标记为 `Blocking`，**实际运行在主线程**（不是线程池），扫描期间 UI 会冻结 —— 这与用户"非阻塞"的诉求不符
3. **现状前端状态**：`App.tsx:44-53` 已有 `scanning` boolean；`Sidebar.tsx:31-37` 按钮显示 "Scanning..." 文字，但无 spinner、无完成反馈（仅 `console.log`）

## 技术方案（推荐 Plan A）

**核心原则**：最小改动 + 满足"非阻塞 + 仅状态"诉求。

采用 **纯前端 + 后端最小解阻塞** 方案：
1. 前端：按钮内嵌 SVG spinner（Tailwind `animate-spin`），完成后显示 3 秒 toast 横条
2. 后端：将 `fn` 改为 `async fn` + `tauri::async_runtime::spawn_blocking`，解除主线程阻塞（真正做到非阻塞）
3. 不做实时进度事件（超出"仅状态"范围，且需要更大改动）

## 实施步骤

### 步骤 1：后端解除主线程阻塞（src-tauri/src/commands.rs）

将 `scan_and_index` 改为 `async fn`，内部用 `spawn_blocking` 执行阻塞 IO。

**修改前**（`src-tauri/src/commands.rs:22-57`）：

```rust
#[tauri::command]
pub fn scan_and_index(db: State<'_, Database>) -> Result<ScanResult, String> {
    let scanner = crate::scanner::Scanner::new();
    // ...同步循环...
}
```

**修改后**（伪代码）：

```rust
#[tauri::command]
pub async fn scan_and_index(app: tauri::AppHandle) -> Result<ScanResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let db = app.state::<Database>();
        let scanner = crate::scanner::Scanner::new();
        let files = scanner.discover_sessions();
        let total_files = files.len() as u32;

        let start = std::time::Instant::now();
        let mut parsed: u32 = 0;
        let mut failed: u32 = 0;

        for path in &files {
            match scanner.parse_session(path) {
                Ok(session) => {
                    if let Err(e) = db.upsert_session(&session) {
                        eprintln!("Failed to index {}: {}", path.display(), e);
                        failed += 1;
                    } else {
                        parsed += 1;
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse {}: {}", path.display(), e);
                    failed += 1;
                }
            }
        }

        Ok(ScanResult {
            total_files,
            parsed,
            failed,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    })
    .await
    .map_err(|e| e.to_string())?
}
```

**验收**：`pnpm tauri dev` 启动后点击 scan，UI 列表滚动、菜单点击等操作不卡顿。

### 步骤 2：前端按钮 spinner（src/components/Sidebar.tsx）

修改按钮内容，`scanning` 时显示旋转图标 + "Scanning..." 文字。

**修改前**（`src/components/Sidebar.tsx:31-37`）：

```tsx
<button
  onClick={onScan}
  disabled={scanning}
  className="w-full px-3 py-1.5 text-sm bg-zinc-900 text-white rounded hover:bg-zinc-700 disabled:opacity-50"
>
  {scanning ? "Scanning..." : "Scan & Index"}
</button>
```

**修改后**（伪代码）：

```tsx
<button
  onClick={onScan}
  disabled={scanning}
  className="w-full px-3 py-1.5 text-sm bg-zinc-900 text-white rounded hover:bg-zinc-700 disabled:opacity-50 flex items-center justify-center gap-2"
>
  {scanning && (
    <svg className="animate-spin h-3.5 w-3.5" viewBox="0 0 24 24" fill="none">
      <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
      <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v4a4 4 0 00-4 4H4z" />
    </svg>
  )}
  <span>{scanning ? "Scanning..." : "Scan & Index"}</span>
</button>
```

**验收**：点击后出现旋转图标（TailwindCSS `animate-spin` 即可），视觉反馈明显。

### 步骤 3：完成结果 Toast（src/App.tsx + 新增状态）

替换 `console.log` 为可见的 inline toast banner（右下角 fixed 定位，3 秒自动消失）。

**当前**（`src/App.tsx:44-53`）：

```tsx
const handleScan = async () => {
  setScanning(true);
  try {
    const result = await scanAndIndex();
    console.log(`Scanned ${result.total_files} files, parsed ${result.parsed} in ${result.duration_ms}ms`);
    await refreshData();
  } finally {
    setScanning(false);
  }
};
```

**修改后**（伪代码）：

```tsx
const [toast, setToast] = useState<{ msg: string; kind: "success" | "error" } | null>(null);

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
    setTimeout(() => setToast(null), 3000);
  }
};

// 在 JSX 底部追加：
{toast && (
  <div className={`fixed bottom-4 right-4 px-4 py-2 rounded shadow-lg text-sm text-white ${
    toast.kind === "success" ? "bg-zinc-900" : "bg-red-600"
  }`}>
    {toast.msg}
  </div>
)}
```

**验收**：扫描完成后右下角出现提示条，3 秒后消失；失败时红色提示。

## 关键文件

| 文件 | 操作 | 说明 |
|------|------|------|
| `src-tauri/src/commands.rs:22-57` | 修改 | `scan_and_index` 改为 `async fn` + `spawn_blocking` |
| `src/components/Sidebar.tsx:31-37` | 修改 | 按钮内嵌 SVG spinner + 文字 |
| `src/App.tsx:44-53` | 修改 | 增加 `toast` 状态，scan 完成/失败后显示 |

## 风险与缓解

| 风险 | 缓解措施 |
|------|----------|
| `spawn_blocking` 内部无法直接持有 `State<'_, Database>`（生命周期），需通过 `app.state::<Database>()` 获取 | 使用 `AppHandle` 注入（codex 建议的模式）；测试后若 `db.conn.lock()` 存在问题再评估 `Arc<Database>` 共享 |
| 改为 `async fn` 后，`lib.rs` 的 `generate_handler!` 注册无需改动（Tauri 自动处理） | 运行 `pnpm tauri dev` 验证编译通过 |
| toast 3 秒消失可能被下一次 scan 覆盖 | 使用 `setTimeout` 清理+再次 setToast 即可（简单实现足够，MVP 不需要 toast 队列） |
| 用户在 scan 中途点击其他按钮，Database 写入与读取并发 | `Database` 已使用 `Arc<Mutex<Connection>>`（见 `db.rs`），SQLite + Mutex 保证串行安全 |

## 明确不做

- ❌ 实时进度事件（`emit("scan-progress", ...)` + 前端 `listen` 订阅）—— 超出"仅状态即可"范围
- ❌ 进度百分比 / 当前处理文件路径显示 —— 同上
- ❌ 引入 toast UI 库（react-hot-toast 等）—— MVP 自制够用
- ❌ 引入 icon 库（lucide-react 等）—— 一个内联 SVG 足够
- ❌ 增量索引优化 —— PROGRESS.md 已列入 Phase 4，本任务不动

## 验收标准

1. 点击 "Scan & Index" 后：按钮内出现旋转 spinner，文字变为 "Scanning..."，按钮禁用
2. 扫描期间：**可以正常滚动会话列表、切换过滤器、点击会话查看详情**（真·非阻塞，由步骤 1 保证）
3. 扫描完成：右下角出现 toast "扫描完成：X/Y 个会话，耗时 N.Ns"，3 秒后消失
4. 扫描失败：显示红色 toast 错误消息
5. 无控制台报错，`pnpm tauri dev` 编译通过

## 备选方案（不推荐，仅作记录）

**Plan B — 纯前端**：不改后端，只加 spinner + toast。
- 优点：改动更小，只动前端
- 缺点：**UI 仍会在扫描期间冻结**，违反"非阻塞"承诺
- 判断：用户明确说"非阻塞、不阻断页面使用"，所以 Plan B 不达标

**Plan C — 加进度事件**：后端 `emit("scan-progress")`，前端显示 `x/y`。
- 优点：用户能看到实时进度
- 缺点：超出"仅状态即可"范围，改动大（后端 emit + 前端 listen + unlisten 生命周期）
- 判断：用户显式说"不需要进度"，所以不做

## SESSION_ID（供 /ccg:execute 使用）

- CODEX_SESSION: `019db03a-92d8-74e2-8657-1b1246e47efb`
- GEMINI_SESSION: 无（API 权限错误连续 3 次失败，已跳过）
