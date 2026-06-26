# 码字 — 极简本地写作

> 极简、高颜值、纯粹本地化、无干扰的沉浸式中文写作软件。
>
> 数据完全掌控在用户手中，纯 Markdown 格式保证永久可读性。

## 技术栈

**Tauri 2.0 (Rust) + Vue 3 + TypeScript + Tiptap**

## 快速开始

```bash
npm install          # 安装依赖
npm run tauri dev    # 开发模式（热重载）
npm run tauri build  # 生产构建（Windows 安装包）
```

## 项目结构

```
码字/
├── src/                          # Vue 3 前端
│   ├── views/                    # HomeView（首页）、EditorView（编辑器）
│   ├── components/               # Sidebar、TiptapEditor、TimelineView 等 8 个组件
│   ├── stores/                   # workspace、editor、theme、annotation、shortcut
│   ├── composables/              # useMarkdown（Markdown ↔ HTML 转换）、useTauriCommands
│   ├── extensions/               # Annotation（自定义 Tiptap 批注扩展）
│   ├── types/                    # TypeScript 类型定义
│   └── styles/                   # 全局样式
├── src-tauri/                    # Tauri 2.0 Rust 后端
│   └── src/
│       ├── models.rs             # 数据模型（BookIndex、ChapterEntry 等）
│       └── commands/
│           ├── workspace.rs      # 工作区 CRUD + 软删除回收站
│           ├── file_io.rs        # 文件读写 + 原子化保存 + 历史快照 + 快照 GC
│           ├── history.rs        # TXT / ZIP / EPUB 导出 + ZIP 恢复
│           └── epub.rs           # EPUB 电子书构建
└── PROJECT.md                    # 完整开发文档
```

## 核心功能

### 编辑器
- **WYSIWYG 富文本** — Tiptap 无头框架，所见即所得，隐藏 Markdown 语法
- **Markdown 双向转换** — 加载时 `marked` 解析，保存时 `turndown` 序列化
- **中文排版** — 段首缩进 2em，字体/字号/行距/字间距/左右边距 6 项动态调节
- **主题系统** — 浅色 / 深色 / 护眼三套主题，CSS 变量驱动
- **稿纸网格线** — `linear-gradient` 动态绘制，`fontSize × lineHeight` 实时联动，像素级对齐
- **专注模式** — `F11` 一键隐藏所有 UI，仅剩纯白页面

### 目录与组织
- **分卷模式** — 小说支持分卷 + 章节，vuedraggable 拖拽排序，实时回写 `book.json`
- **平铺模式** — 随笔/散文直接新建篇章
- **时间线视图** — 随笔按年份分组，垂直时间轴展示，支持升序/降序

### 灵感批注
- **选中标注** — 选中文字后 `Ctrl+Shift+A` 添加批注，波浪下划线 + 淡色背景高亮
- **隐式存储** — 批注内容存为 `<!--inspiration:{...}-->文字<!--/inspiration-->`，任何 Markdown 阅读器自动隐藏
- **侧边栏联动** — 点击卡片滚动到对应文字，悬停卡片高亮编辑器中的批注

### 数据安全
- **原子化保存** — 写临时文件 → `sync_all()` 强制刷盘 → `rename()` 原子替换，断电不丢稿
- **历史快照** — 每次保存自动在 `.history/` 创建 gzip 压缩快照（`.md.gz`），兼容旧版明文快照
- **快照垃圾回收** — 按时间梯级自动清理：1 小时内全保留，1～24 小时每小时 1 个，1～7 天每天 1 个，以此类推，每个文件最多 100 个
- **软删除回收站** — 删除的章节/书籍移动到 `.trash/` 目录，带时间戳防重名，误删可随时恢复

### 导出
- **TXT** — 按 `book.json` 顺序合并章节，正则清洗 HTML 注释和 Markdown 语法
- **PDF** — A5 出版级排版，CSS `@page` 规则，封面 + 目录 + 页眉页脚 + 章节自动分页
- **EPUB** — 完整 EPUB 2.0 标准，支持 Kindle、Apple Books、微信读书
- **ZIP 备份** — 全量打包 `.md` + `book.json` + `.history`，支持恢复

### 其它
- **搜索替换** — `Ctrl+F` 打开面板，实时匹配计数，上一个/下一个导航，替换当前/全部替换
- **封面图片** — 支持上传封面，嵌入 PDF 打印
- **书籍信息** — 编辑书名、作者、简介/大纲
- **资源管理器** — 一键打开文件存储目录

## 数据存储

```
用户主目录/SelfWriting/
├── 我的小说/
│   ├── book.json              # 索引文件（排序、元数据）
│   ├── 第一章.md               # 纯 Markdown 正文
│   ├── 第二章.md
│   ├── .history/              # 压缩历史快照（gzip）
│   │   ├── 第一章_20260601_120000.md.gz
│   │   └── 第一章_20260601_121500.md.gz
│   └── .trash/                # 软删除回收站
│       └── 第三章_20260601_130000
├── 随笔集/
│   └── ...
```

所有数据以纯文件形式暴露，可用任意编辑器（VS Code、Typora、记事本）直接打开，可用 Git 做版本管理。

## 键盘快捷键

| 快捷键 | 功能 |
|---|---|
| `Ctrl+S` | 保存 |
| `Ctrl+F` | 搜索替换 |
| `Ctrl+Shift+A` | 选中文字添加批注 |
| `Ctrl+Shift+E` | 导出对话框 |
| `F11` | 专注模式 |
| `Escape` | 关闭搜索 / 关闭面板 |

## 完整文档

详见 [PROJECT.md](./PROJECT.md)