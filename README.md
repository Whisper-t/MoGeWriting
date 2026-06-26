# 码字 — 极简本地写作

> 极简、高颜值、纯粹本地化、无干扰的沉浸式中文写作软件。

## 技术栈

**Tauri 2.0 (Rust) + Vue 3 + TypeScript + Tiptap**

## 快速开始

```bash
# 安装依赖
npm install

# 开发模式
npm run tauri dev

# 生产构建
npm run tauri build
```

## 项目结构

```
├── src/                    # Vue 3 前端
│   ├── views/              # 页面视图
│   ├── components/         # 可复用组件
│   ├── stores/             # Pinia 状态管理
│   ├── composables/        # 组合式函数
│   ├── extensions/         # Tiptap 自定义扩展
│   ├── types/              # TypeScript 类型
│   └── styles/             # 全局样式
├── src-tauri/              # Tauri 2.0 Rust 后端
│   └── src/
│       ├── models.rs       # 数据模型
│       └── commands/       # Tauri 命令
└── PROJECT.md              # 完整项目文档
```

## 功能特性

- **WYSIWYG 编辑器** — Tiptap 富文本，Markdown 双向转换
- **中文排版** — 段首缩进、字体/行距/字间距/边距动态调节
- **主题系统** — 浅色/深色/护眼三套主题
- **稿纸网格线** — 动态生成，像素级对齐
- **灵感批注** — 选中文字添加批注，隐式注释存储
- **拖拽排序** — 分卷/章节拖拽排序，实时回写
- **时间线视图** — 随笔按时间轴展示
- **历史快照** — 每次保存自动压缩备份
- **原子化保存** — 防断电丢稿
- **软删除回收站** — .trash 目录，误删可恢复
- **搜索替换** — Ctrl+F 内置查找替换
- **多格式导出** — TXT / PDF / EPUB / ZIP

## 键盘快捷键

| 快捷键 | 功能 |
|---|---|
| `Ctrl+S` | 保存 |
| `Ctrl+F` | 搜索替换 |
| `Ctrl+Shift+A` | 添加批注 |
| `Ctrl+Shift+E` | 导出 |
| `F11` | 专注模式 |

## 完整文档

详见 [PROJECT.md](./PROJECT.md)