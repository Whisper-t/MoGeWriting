# 码字 — 极简本地码字软件

> 极简、高颜值、纯粹本地化、无干扰的沉浸式中文写作软件。
> 
> 技术栈：**Tauri 2.0 (Rust) + Vue 3 + TypeScript + Tiptap**

---

## 目录

- [一、产品设计](#一产品设计)
- [二、技术架构](#二技术架构)
- [三、项目目录结构](#三项目目录结构)
- [四、开发日志](#四开发日志)
  - [阶段一：底层架构与文件管理引擎](#阶段一底层架构与文件管理引擎)
  - [阶段二：所见即所得编辑器与中文排版](#阶段二所见即所得编辑器与中文排版)
  - [阶段三：侧边栏批注与灵感系统](#阶段三侧边栏批注与灵感系统)
  - [阶段四：界面组装与数据绑定](#阶段四界面组装与数据绑定)
  - [阶段五：导出引擎、备份恢复与封版](#阶段五导出引擎备份恢复与封版)
- [五、数据存储规范](#五数据存储规范)
- [六、键盘快捷键](#六键盘快捷键)
- [七、运行与构建](#七运行与构建)

---

## 一、产品设计

### 核心理念

- 极简、高颜值、纯粹本地化、无干扰的沉浸式中文写作体验
- 数据完全掌控在用户手中，纯 Markdown 格式保证永久可读性
- 灵感批注与正文解耦但紧密联动，存储为隐形 HTML 注释

### 目标平台

- Windows 桌面端

### 核心功能矩阵

| 功能 | 说明 |
|---|---|
| 工作区管理 | 创建/删除小说或随笔集，卡片式首页展示 |
| 多级目录 | 小说模式支持分卷+章节，拖拽排序 |
| 时间线 | 随笔模式按时间轴展示，支持升序/降序 |
| WYSIWYG 编辑器 | Tiptap 富文本，隐藏 Markdown 语法 |
| 中文排版 | 段首缩进、字体/行距/字间距/边距动态调节 |
| 主题系统 | 浅色/深色/护眼三套主题，CSS 变量驱动 |
| 稿纸网格线 | 动态生成，与文字行高像素级对齐 |
| 灵感批注 | 选中文字添加批注，侧边栏联动，存为 HTML 注释 |
| 历史快照 | 每次保存自动在 .history 目录创建时间戳副本 |
| 导出 TXT | 按章节顺序合并，清洗注释和 Markdown 语法 |
| 导出 PDF | A5 出版级排版，@page CSS 规则，章节自动分页 |
| ZIP 备份 | 全量打包 .md + book.json + .history |
| ZIP 恢复 | 从备份文件恢复全部数据 |
| EPUB 导出 | 电子书格式，支持 Kindle、Apple Books |
| 原子化保存 | 写临时文件→重命名，防断电丢稿 |
| 软删除回收站 | .trash 目录，误删可恢复 |
| 搜索替换 | Ctrl+F 内置查找/替换 |

---

## 二、技术架构

```
┌──────────────────────────────────────────────────────┐
│                    Tauri 2.0 Shell                     │
│  ┌─────────────────────┐  ┌─────────────────────────┐ │
│  │    Rust 后端          │  │    Vue 3 前端            │ │
│  │                       │  │                          │ │
│  │  • 文件系统 I/O       │  │  • Tiptap 富文本编辑器    │ │
│  │  • book.json 解析     │  │  • 主题/排版系统          │ │
│  │  • .history 快照      │  │  • 灵感批注系统           │ │
│  │  • ZIP 打包/解压      │  │  • 拖拽排序               │ │
│  │  • TXT 净化导出       │  │  • 时间线视图             │ │
│  │  • 字数统计            │  │  • 导出/备份 UI           │ │
│  └─────────────────────┘  └─────────────────────────┘ │
│                         ↕ IPC                          │
└──────────────────────────────────────────────────────┘
```

### 数据流

```
┌─────────────────────────────────────────────────────┐
│  .md 文件 ──[marked]──→ HTML ──[Tiptap]──→ 编辑器    │
│  编辑器 ──[Tiptap.getHTML()]──→ HTML ──[turndown]──→ .md │
│                                                      │
│  灵感批注：                                           │
│  <!--inspiration:{...}-->文字<!--/inspiration-->      │
│  ↕ annotationsToComments / commentsToAnnotations     │
│  <span data-annotation-id="uuid" class="annotation">  │
└─────────────────────────────────────────────────────┘
```

### 依赖清单

**Rust (Cargo.toml)**

| 依赖 | 用途 |
|---|---|
| `tauri` 2.x | 桌面应用框架 |
| `tauri-plugin-opener` | 系统文件打开 |
| `tauri-plugin-dialog` | 文件保存/打开对话框 |
| `serde` / `serde_json` | JSON 序列化 |
| `chrono` | 时间戳处理 |
| `walkdir` | 目录遍历 |
| `zip` | ZIP 压缩/解压 |
| `uuid` | 唯一 ID 生成 |
| `regex-lite` | TXT 导出正则清洗 |
| `flate2` | 历史快照 gzip 压缩 |
| `base64` | 封面图片 base64 编码 |
| `dirs` | 用户主目录获取 |

**前端 (package.json)**

| 依赖 | 用途 |
|---|---|
| `vue` 3.x | UI 框架 |
| `vue-router` 4.x | 页面路由 |
| `pinia` | 状态管理 |
| `@tiptap/vue-3` | 富文本编辑器 |
| `@tiptap/starter-kit` | 编辑器基础扩展 |
| `@tiptap/extension-underline` | 下划线支持 |
| `@tiptap/extension-placeholder` | 占位文本 |
| `@tiptap/extension-character-count` | 字数统计 |
| `marked` | Markdown → HTML 解析 |
| `turndown` | HTML → Markdown 序列化 |
| `vuedraggable` | 拖拽排序 |
| `sortablejs` | 拖拽底层库 |
| `@tauri-apps/api` | Tauri 前端 API |
| `@tauri-apps/plugin-dialog` | 文件对话框 |

---

## 三、项目目录结构

```
码字/
├── index.html                          # 应用入口 HTML
├── package.json                        # 前端依赖与脚本
├── vite.config.ts                      # Vite 构建配置
├── tsconfig.json                       # TypeScript 配置
├── tsconfig.node.json                  # Node 端 TS 配置
│
├── src/                                # Vue 3 前端源码
│   ├── main.ts                         # 应用入口：挂载 Pinia + Router
│   ├── App.vue                         # 根组件：页面过渡动画
│   │
│   ├── views/                          # 页面视图
│   │   ├── HomeView.vue                # 工作台首页：卡片式作品列表
│   │   └── EditorView.vue              # 主编辑器：三栏布局、快捷键
│   │
│   ├── components/                     # 可复用组件
│   │   ├── TiptapEditor.vue            # Tiptap 富文本编辑器核心
│   │   ├── Sidebar.vue                 # 左侧目录树（拖拽排序）
│   │   ├── TimelineView.vue            # 随笔时间线视图
│   │   ├── InspectorPanel.vue          # 右侧灵感批注面板
│   │   ├── ThemeSettings.vue           # 排版设置面板
│   │   ├── BookInfoEditor.vue          # 书籍简介/大纲编辑
│   │   ├── ExportDialog.vue            # 导出/备份对话框
│   │   └── StatusBar.vue               # 底部状态栏
│   │
│   ├── stores/                         # Pinia 状态管理
│   │   ├── workspace.ts                # 工作区状态（CRUD、book.json）
│   │   ├── editor.ts                   # 编辑器状态（内容、保存、计时）
│   │   ├── theme.ts                    # 主题系统（CSS 变量、排版参数）
│   │   └── annotation.ts               # 批注状态（灵感卡片 CRUD）
│   │
│   ├── composables/                    # 组合式函数
│   │   ├── useMarkdown.ts              # Markdown ↔ HTML 双向转换
│   │   │                               #  + 批注注释 ↔ span 序列化
│   │   └── useTauriCommands.ts         # Tauri Rust 命令封装
│   │
│   ├── extensions/                     # Tiptap 自定义扩展
│   │   └── Annotation.ts              # 自定义批注 mark 扩展
│   │
│   ├── types/                          # TypeScript 类型定义
│   │   └── index.ts                    # 全部数据模型接口
│   │
│   └── styles/                         # 全局样式
│       └── global.css                  # CSS 变量、重置、滚动条、聚焦环
│
├── src-tauri/                          # Tauri 2.0 Rust 后端
│   ├── Cargo.toml                      # Rust 依赖
│   ├── tauri.conf.json                 # Tauri 窗口与打包配置
│   ├── capabilities/                   # 权限配置
│   │   └── default.json
│   └── src/
│       ├── main.rs                     # Rust 入口
│       ├── lib.rs                      # 插件注册 + 命令注册
│       ├── models.rs                   # 全部数据模型（BookIndex 等）
│       └── commands/
│           ├── mod.rs                  # 模块声明
│           ├── workspace.rs            # 工作区 CRUD + book.json 引擎
│           ├── file_io.rs              # 文件读写 + 历史快照
│           └── history.rs              # TXT/ZIP/EPUB 导出 + ZIP 恢复
│           └── epub.rs                # EPUB 电子书构建
│
└── dist/                               # Vite 构建输出
```

---

## 四、开发日志

### 阶段一：底层架构与文件管理引擎

**目标**：在 Rust 端搭建稳固的本地数据基座。

**完成工作**：

1. **项目初始化**
   - 清理 Tauri + Vue 3 样板代码
   - 安装 Tiptap、vue-router、pinia 等核心依赖

2. **Rust 数据模型** (`models.rs`)
   - 定义 `BookIndex`（book.json 结构）、`ChapterEntry`、`VolumeEntry`
   - 定义 `WorkspaceSummary`（首页展示用）、`SaveRequest`、`ExportRequest` 等
   - 支持小说（分卷+章节）和随笔（平铺章节）两种模式

3. **工作区 API** (`commands/workspace.rs`)
   - `list_workspaces`：扫描根目录，返回所有有效工作区摘要
   - `create_workspace`：创建新书籍/随笔集，自动生成 `book.json`
   - `read_book_index` / `write_book_index`：book.json 读写
   - `create_chapter` / `create_volume`：创建章节/分卷，自动生成 .md 文件
   - `rename_entry` / `delete_chapter` / `delete_volume`：重命名/删除
   - `reorder`：拖拽排序后回写 book.json
   - `list_directory` / `delete_workspace` / `get_home_dir`

4. **文件 I/O** (`commands/file_io.rs`)
   - `read_file` / `write_file` / `write_files`：.md 文件读写
   - `save_history_snapshot`：每次保存自动在 `.history/` 创建时间戳副本
   - `list_history` / `read_history_snapshot` / `restore_snapshot`：历史快照管理

5. **导出引擎** (`commands/history.rs`)
   - `export_txt`：按 book.json 顺序合并章节，正则清洗 HTML 注释和 Markdown 语法
   - `export_zip`：递归打包全部文件为 ZIP
   - `restore_from_zip`：从 ZIP 解压覆盖恢复
   - `count_words`：中文字数统计

6. **前端基础框架**
   - 创建 `router/index.ts`：Home 和 Editor 两个路由
   - 创建 `types/index.ts`：TypeScript 类型定义
   - 创建 `composables/useTauriCommands.ts`：Tauri 命令封装
   - 创建 `stores/workspace.ts`、`stores/editor.ts`、`stores/theme.ts`：Pinia 状态管理
   - 创建 `styles/global.css`：CSS 变量、极简滚动条、选中颜色

**新增文件**：`models.rs`、`commands/` 目录（3 个文件）、`lib.rs` 重写、`types/index.ts`、`stores/` 目录（3 个文件）、`composables/useTauriCommands.ts`、`router/index.ts`、`styles/global.css`

---

### 阶段二：所见即所得编辑器与中文排版

**目标**：打造无干扰且符合中文直觉的纯净写作体验。

**完成工作**：

1. **Markdown ↔ HTML 双向转换** (`composables/useMarkdown.ts`)
   - 使用 `marked` 将 Markdown 解析为 HTML（加载时）
   - 使用 `turndown` 将 HTML 序列化为 Markdown（保存时）
   - 自定义 turndown 规则：保留 HTML 注释、段落无物理缩进、下划线保留
   - 字数统计函数：移除 Markdown 语法后统计中文字符+英文单词

2. **Tiptap 编辑器** (`components/TiptapEditor.vue`)
   - 集成 StarterKit（标题、加粗、斜体、列表、引用等）
   - 集成 Underline、Placeholder、CharacterCount 扩展
   - 中文排版 CSS：`text-indent: 2em` 段首缩进、标题无缩进
   - 监听主题变化，实时更新字号/行距/字间距/字体/边距

3. **主题系统** (`stores/theme.ts`)
   - 浅色/深色/护眼三套主题，CSS 变量驱动
   - 字号(12-28px)、行距(1.2-3.0)、字间距(0-5px)、左右边距(20-200px) 动态调节
   - 6 种中文字体预设：宋体/黑体/楷体/思源宋体/思源黑体/微软雅黑

4. **稿纸网格线**
   - 使用 CSS `repeating-linear-gradient` 绘制
   - 网格线高度 = `fontSize × lineHeight`，随字号/行距自动重算
   - 背景偏移量动态跟踪，确保与文字行像素级对齐

5. **排版设置面板** (`components/ThemeSettings.vue`)
   - 主题切换按钮组
   - 字体选择下拉框
   - 字号/行距/字间距/边距滑块
   - 网格线开关 + 实时线高显示

6. **写作体验增强**
   - F11 专注模式：隐藏所有 UI 边栏
   - Ctrl+S 保存快捷键
   - 30 秒自动保存
   - 状态栏：实时字数 + 写作时长计时

7. **首页视图** (`views/HomeView.vue`)
   - 卡片式作品列表，按修改时间降序
   - 新建对话框：名称、类型（小说/随笔）、简介、作者
   - 删除确认对话框
   - 主题切换按钮

**新增文件**：`composables/useMarkdown.ts`、`views/HomeView.vue`、`views/EditorView.vue`、`components/TiptapEditor.vue`、`components/ThemeSettings.vue`、`components/Sidebar.vue`、`components/InspectorPanel.vue`、`components/StatusBar.vue`

---

### 阶段三：侧边栏批注与灵感系统

**目标**：让灵感与正文解耦但又紧密联动，这是技术最密集的阶段。

**完成工作**：

1. **自定义 Tiptap Annotation 扩展** (`extensions/Annotation.ts`)
   - 创建 `annotation` mark，可应用于选中文字
   - 渲染为 `<span data-annotation-id="uuid" class="annotation">`
   - 显示样式：波浪下划线 + 淡色背景高亮
   - 支持 `parseHTML` / `renderHTML` 双向转换
   - 快捷键 `Mod-Shift-a` 预留

2. **隐式注释存储格式**
   ```
   <!--inspiration:{"id":"uuid","content":"灵感内容","createdAt":"...","modifiedAt":"...","quotedText":"被标注文字"}-->被标注的文字<!--/inspiration-->
   ```
   - 灵感元数据以 JSON 嵌入 HTML 注释
   - 被标注文字夹在开始/结束注释之间
   - 任何 Markdown 阅读器都会自动隐藏注释，正文完全纯净

3. **批注序列化/反序列化** (`composables/useMarkdown.ts` 新增)
   - `annotationsToComments(html, annotationMap)`：编辑器 span → 注释
   - `commentsToAnnotations(html)`：注释 → 编辑器 span + 提取批注数据
   - 在 TiptapEditor 的 save/load 流程中无缝集成

4. **批注状态管理** (`stores/annotation.ts`)
   - 批注 CRUD：添加、更新、删除、批量加载
   - 编辑/悬停状态追踪
   - 侧边栏开关状态

5. **侧边栏联动** (`components/InspectorPanel.vue` 重写)
   - 连接真实批注数据，卡片列表展示
   - 点击卡片：编辑器自动滚动到对应文字并选中
   - 悬停卡片：对应批注文字自动高亮（`active` 类切换）
   - 引文预览：「被标注的文字」
   - 编辑/删除操作

6. **编辑器集成** (`components/TiptapEditor.vue` 更新)
   - 导入 Annotation 扩展
   - 暴露 `addAnnotation(id)` / `removeAnnotation()` / `scrollToAnnotation(id)` 方法
   - 悬停联动：监听 `annotationStore.hoveredId` 变化，动态切换 CSS 类

7. **快捷键**
   - `Ctrl+Shift+A`：选中文字一键添加批注

**新增文件**：`extensions/Annotation.ts`、`stores/annotation.ts`
**重写文件**：`components/InspectorPanel.vue`、`components/TiptapEditor.vue`（集成批注）

---

### 阶段四：界面组装与数据绑定

**目标**：将零散的功能模块拼装为高颜值的现代化极简桌面软件。

**完成工作**：

1. **拖拽排序** (`components/Sidebar.vue` 重写)
   - 引入 `vuedraggable`（基于 SortableJS）
   - 分卷拖拽排序：`⋮⋮` 手柄，拖拽结束自动回写 book.json
   - 章节拖拽排序（卷内）：`⋮` 手柄，拖拽结束自动回写
   - 随笔拖拽排序：`⋮` 手柄，拖拽结束自动回写
   - 拖拽时显示半透明幽灵占位，手柄平时隐藏、悬停显现

2. **随笔时间线视图** (`components/TimelineView.vue`)
   - 按年份分组，垂直时间轴 UI
   - 显示创建时间、标题、字数
   - 支持升序/降序切换
   - 点击条目直接加载对应随笔
   - 从侧边栏「◷ 时间线」按钮进入

3. **书籍简介编辑** (`components/BookInfoEditor.vue`)
   - 展示书名、类型、创建时间、章节数、总字数
   - 可编辑作者和简介/大纲
   - 失焦自动保存，手动保存按钮
   - 顶栏「ⓘ」按钮展开

4. **界面组装**
   - 三栏布局：侧边栏 | 编辑器 | 灵感面板
   - 各面板可独立展开/折叠
   - 专注模式一键隐藏全部 UI
   - 页面过渡动画（fade）

**新增文件**：`components/TimelineView.vue`、`components/BookInfoEditor.vue`
**重写文件**：`components/Sidebar.vue`（拖拽排序）
**更新文件**：`views/EditorView.vue`（集成时间线、书籍信息）、`stores/workspace.ts`（添加 writeBookIndex）

---

### 阶段五：导出引擎、备份恢复与封版

**目标**：完成作品的最后输出通路与整机安全保障。

**完成工作**：

1. **Tauri 对话框集成**
   - 安装 `tauri-plugin-dialog`，注册到 Rust 后端
   - 使用 `save()` 打开文件保存对话框
   - 使用 `open()` 打开文件选择对话框（ZIP 恢复）

2. **导出对话框** (`components/ExportDialog.vue`)
   - **TXT 导出**：调用 Rust `export_txt`，用户选择保存路径
   - **PDF 导出**：生成 A5 打印 HTML（@page 规则、页眉书名、页脚页码），打开新窗口调用 `window.print()`
   - **ZIP 备份**：调用 Rust `export_zip`，用户选择保存路径
   - **ZIP 恢复**：选择 ZIP 文件，调用 Rust `restore_from_zip`，覆盖恢复
   - 状态消息反馈（成功/失败）
   - 恢复操作二次确认

3. **PDF 打印模板**
   - CSS `@page` 规则：A5 尺寸、2cm 页边距
   - 封面页：书名、作者、简介
   - 章节自动分页：`page-break-before: always`
   - 页眉：书名 | 页脚：页码
   - 清洗灵感注释，保留基本 Markdown 格式

4. **首页恢复入口** (`views/HomeView.vue` 更新)
   - 顶栏「📥 恢复」按钮，无需打开书籍即可恢复备份
   - 状态消息显示

5. **全局 UI 抛光**
   - 滚动条：默认透明，悬停时显现
   - 聚焦环：仅键盘导航时显示 `:focus-visible`
   - 页面过渡动画：`page-fade` 淡入淡出
   - 图片拖拽禁用
   - 窗口配置：最小 800×600，默认 1200×800，标题"码字"

6. **快捷键汇总**
   - `Ctrl+Shift+E`：打开导出对话框

**新增文件**：`components/ExportDialog.vue`
**更新文件**：`views/EditorView.vue`（导出按钮+快捷键）、`views/HomeView.vue`（恢复按钮）、`src-tauri/tauri.conf.json`（窗口配置）、`src-tauri/Cargo.toml`（dialog 插件）、`src-tauri/src/lib.rs`（注册 dialog 插件）、`src/styles/global.css`（滚动条/聚焦环抛光）、`src/App.vue`（页面过渡）

---

### 阶段六：体验增强与格式扩展

**新增功能**：

1. **原子化保存** (`commands/file_io.rs`)
   - 新增 `atomic_write` 函数：写临时文件 → 重命名
   - `write_file` 和 `write_files` 全部改用原子化写入
   - 断电不丢稿，要么旧文件完整要么新文件完整

2. **软删除回收站** (`commands/workspace.rs`)
   - 新增 `move_to_trash` 函数：删除变为移动到 `.trash/`
   - `delete_chapter`、`delete_volume`、`delete_workspace` 全部软删除
   - 带时间戳防重名，误删可随时从 `.trash/` 恢复

3. **搜索与替换** (`views/EditorView.vue`)
   - `Ctrl+F` 打开搜索面板，`Escape` 关闭
   - 实时匹配计数、上一个/下一个导航
   - 替换当前 / 全部替换

4. **EPUB 电子书导出** (`commands/epub.rs`)
   - 完整 EPUB 2.0 标准实现
   - ZIP 容器 + XHTML 内容 + OPF 清单 + NCX 目录
   - 支持 Kindle、Apple Books、微信读书
   - 导出对话框新增「EPUB」选项

5. **在资源管理器中打开** (`commands/file_io.rs`)
   - 新增 `open_in_explorer` 命令
   - 编辑器顶栏 `◰` 按钮打开当前书籍文件夹
   - 首页 `◰ 目录` 按钮打开存储根目录

6. **历史快照压缩** (`commands/file_io.rs`)
   - 快照保存为 `.md.gz` 格式，使用 gzip 压缩
   - 读取时自动解压，兼容旧版 `.md` 快照
   - 恢复时自动解压或直接复制

7. **图标统一与文字修正**
   - 全部 emoji 图标替换为极简单字符 Unicode 符号
   - 省略号 `...` 替换为排版正确的 `…`
   - 状态图标统一：`✓` / `✗` / `⚠`

**新增文件**：`commands/epub.rs`
**更新文件**：`commands/file_io.rs`、`commands/workspace.rs`、`commands/history.rs`、`lib.rs`、`views/EditorView.vue`、`views/HomeView.vue`、`components/ExportDialog.vue`、`components/BookInfoEditor.vue`、`components/Sidebar.vue`、`components/InspectorPanel.vue`、`components/HistoryPanel.vue`、`components/ShortcutGuide.vue`、`components/TiptapEditor.vue`、`stores/editor.ts`、`composables/useMarkdown.ts`、`styles/global.css`

---

## 五、数据存储规范

### 目录结构

```
用户文档/SelfWriting/           # 根目录（可配置）
├── 我的小说/                    # 一本书 = 一个文件夹
│   ├── book.json                # 索引文件：书名、类型、卷/章节排序
│   ├── 第一章.md                # 正文（纯 Markdown）
│   ├── 第二章.md
│   ├── 第三章.md
│   ├── .trash/                  # 回收站（软删除文件）
│   └── .history/                # 隐藏历史快照（gzip 压缩）
│       ├── 第一章_20260101_120000.md.gz
│       └── 第一章_20260101_121500.md.gz
│
├── 随笔集/
│   ├── book.json
│   ├── 春日随笔.md
│   ├── 夏夜.md
│   └── .history/
│       └── ...
```

### book.json 结构

```json
{
  "name": "我的小说",
  "type": "novel",
  "description": "一部关于...的小说",
  "author": "作者名",
  "created_at": "2026-01-01T12:00:00",
  "modified_at": "2026-06-26T15:30:00",
  "volumes": [
    {
      "id": "uuid-1",
      "title": "第一卷",
      "order": 0,
      "chapters": [
        {
          "id": "uuid-2",
          "title": "引子",
          "file": "引子.md",
          "order": 0,
          "word_count": 1200,
          "created_at": "2026-01-01T12:00:00",
          "modified_at": "2026-06-26T15:30:00"
        }
      ]
    }
  ]
}
```

### 灵感批注存储格式

```
正文内容<!--inspiration:{"id":"uuid","content":"这里有一个灵感想法","createdAt":"2026-06-26T15:30:00","modifiedAt":"2026-06-26T15:30:00","quotedText":"正文内容"}-->正文内容<!--/inspiration-->继续正文
```

用任意 Markdown 阅读器打开时，`<!-- ... -->` 注释自动隐藏，正文纯净可读。

---

## 六、键盘快捷键

| 快捷键 | 作用域 | 功能 |
|---|---|---|
| `Ctrl+S` | 编辑器 | 保存当前文件 |
| `Ctrl+F` | 编辑器 | 打开搜索替换面板 |
| `Ctrl+Shift+A` | 编辑器 | 选中文字添加灵感批注 |
| `Ctrl+Shift+E` | 编辑器 | 打开导出/备份对话框 |
| `F11` | 编辑器 | 切换专注模式（隐藏所有 UI） |

---

## 七、运行与构建

### 开发模式

```bash
# 安装依赖
npm install

# 启动 Tauri 开发环境（含热重载）
npm run tauri dev

# 仅启动前端开发服务器
npm run dev
```

### 生产构建

```bash
# 构建 Windows 安装包
npm run tauri build
```

### 技术栈版本

| 技术 | 版本 |
|---|---|
| Tauri | 2.x |
| Rust | 2021 edition |
| Vue | 3.5.x |
| TypeScript | 5.6.x |
| Vite | 6.x |
| Tiptap | 2.x (latest) |
| Pinia | 2.x (latest) |
| Vue Router | 4.x |

---

> 🤖 本项目由 Claude Code 辅助开发，遵循极简设计原则，所有代码采用 MIT 协议。