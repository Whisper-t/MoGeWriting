<script setup lang="ts">
import { ref, computed } from 'vue'
import { save, open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import { useWorkspaceStore } from '@/stores/workspace'
import type { ChapterEntry } from '@/types'

defineEmits<{ close: [] }>()

const workspace = useWorkspaceStore()

const exporting = ref(false)
const exportStatus = ref('')
const showRestore = ref(false)

const bookName = computed(() => workspace.bookIndex?.name || '未命名')
const bookType = computed(() => workspace.bookIndex?.type === 'novel' ? '小说' : (workspace.bookIndex?.type === 'essay' ? '随笔' : ''))

// ========== TXT 导出 ==========

async function handleExportTxt() {
  if (!workspace.currentWorkspace) return
  exporting.value = true
  exportStatus.value = '正在导出 TXT…'

  try {
    const filePath = await save({
      defaultPath: `${bookName.value}.txt`,
      filters: [{ name: '纯文本', extensions: ['txt'] }],
    })

    if (!filePath) {
      exporting.value = false
      exportStatus.value = ''
      return
    }

    await invoke('export_txt', {
      request: {
        book_path: workspace.currentWorkspace.path,
        output_path: filePath,
        format: 'txt',
      },
    })

    exportStatus.value = `✓ TXT 已导出到：${filePath}`
  } catch (e) {
    exportStatus.value = `✗ 导出失败：${e}`
  } finally {
    exporting.value = false
  }
}

// ========== ZIP 导出 ==========

async function handleExportZip() {
  if (!workspace.currentWorkspace) return
  exporting.value = true
  exportStatus.value = '正在打包 ZIP…'

  try {
    const filePath = await save({
      defaultPath: `${bookName.value}_备份.zip`,
      filters: [{ name: 'ZIP 压缩包', extensions: ['zip'] }],
    })

    if (!filePath) {
      exporting.value = false
      exportStatus.value = ''
      return
    }

    await invoke('export_zip', {
      request: {
        book_path: workspace.currentWorkspace.path,
        output_path: filePath,
        format: 'zip',
      },
    })

    exportStatus.value = `✓ ZIP 备份已导出到：${filePath}`
  } catch (e) {
    exportStatus.value = `✗ 导出失败：${e}`
  } finally {
    exporting.value = false
  }
}

// ========== PDF 导出 ==========

async function handleExportPdf() {
  if (!workspace.bookIndex || !workspace.currentWorkspace) return
  exporting.value = true
  exportStatus.value = '正在生成 PDF…'

  try {
    const chapters = collectChapters()
    const chapterContents: { title: string; content: string }[] = []

    for (const ch of chapters) {
      try {
        const content = await invoke<string>('read_file', {
          path: `${workspace.currentWorkspace.path}\\${ch.file}`,
        })
        chapterContents.push({ title: ch.title, content })
      } catch {
        chapterContents.push({ title: ch.title, content: '' })
      }
    }

    // 读取封面图片（如果有）
    let coverBase64 = ''
    const coverImage = workspace.bookIndex.cover_image
    if (coverImage) {
      try {
        const imgPath = `${workspace.currentWorkspace.path}\\${coverImage}`
        coverBase64 = await invoke<string>('read_file_base64', { path: imgPath })
      } catch {
        // 图片不存在，忽略
      }
    }

    const printHtml = generatePrintHtml(chapterContents, coverBase64, coverImage)

    // 写入临时文件，用 Rust 命令在默认浏览器中打开
    const tempPath = `${workspace.currentWorkspace.path}\\print_temp.html`
    await invoke('write_file', { request: { path: tempPath, content: printHtml } })
    await invoke('open_in_browser', { path: tempPath })

    exportStatus.value = '✓ 已在浏览器中打开，请按 Ctrl+P 并选择「另存为 PDF」'
  } catch (e) {
    exportStatus.value = `✗ 生成失败：${e}`
  } finally {
    exporting.value = false
  }
}

function collectChapters(): ChapterEntry[] {
  const book = workspace.bookIndex!
  if (book.volumes) {
    const sorted = [...book.volumes].sort((a, b) => a.order - b.order)
    const result: ChapterEntry[] = []
    for (const vol of sorted) {
      const chs = [...vol.chapters].sort((a, b) => a.order - b.order)
      result.push(...chs)
    }
    return result
  }
  if (book.chapters) {
    return [...book.chapters].sort((a, b) => a.order - b.order)
  }
  return []
}

function generatePrintHtml(
  chapters: { title: string; content: string }[],
  coverBase64: string,
  coverImage: string | undefined,
): string {
  const author = workspace.bookIndex?.author || '佚名'
  const description = workspace.bookIndex?.description || ''
  const book = workspace.bookIndex!

  // 封面图片 HTML
  const mimeType = coverImage
    ? (coverImage.endsWith('.png') ? 'image/png' : coverImage.endsWith('.jpg') || coverImage.endsWith('.jpeg') ? 'image/jpeg' : coverImage.endsWith('.webp') ? 'image/webp' : 'image/png')
    : 'image/png'
  const coverImgHtml = coverBase64
    ? `<img src="data:${mimeType};base64,${coverBase64}" class="cover-img" />`
    : ''

  // 生成目录
  let tocHtml = ''
  if (book.volumes) {
    // 小说模式：分卷展示
    const sortedVols = [...book.volumes].sort((a, b) => a.order - b.order)
    for (const vol of sortedVols) {
      tocHtml += `<div class="toc-volume">${escapeHtml(vol.title)}</div>\n`
      const sortedChs = [...vol.chapters].sort((a, b) => a.order - b.order)
      for (const ch of sortedChs) {
        tocHtml += `<div class="toc-item">${escapeHtml(ch.title)}</div>\n`
      }
    }
  } else if (book.chapters) {
    // 随笔模式：平铺
    const sortedChs = [...book.chapters].sort((a, b) => a.order - b.order)
    for (const ch of sortedChs) {
      tocHtml += `<div class="toc-item">${escapeHtml(ch.title)}</div>\n`
    }
  }

  const chapterHtml = chapters
    .map((ch) => `
    <section class="chapter">
      <h1 class="chapter-title">${escapeHtml(ch.title)}</h1>
      <div class="chapter-content">${markdownToPrintHtml(ch.content)}</div>
    </section>`)
    .join('\n')

  return `<!DOCTYPE html>
<html lang="zh-CN">
<head>
<meta charset="UTF-8">
<title>${escapeHtml(bookName.value)}</title>
<style>
  @page {
    size: A5;
    margin: 2cm 2.5cm 2.5cm 2.5cm;
    @top-center {
      content: "${escapeHtml(bookName.value)}";
      font-size: 9pt;
      color: #999;
      font-family: "SimSun", "宋体", serif;
    }
    @bottom-center {
      content: counter(page);
      font-size: 9pt;
      color: #999;
      font-family: "SimSun", "宋体", serif;
    }
  }
  @page cover { @top-center { content: none; } @bottom-center { content: none; } }
  @page toc { @top-center { content: none; } @bottom-center { content: none; } }
  * { margin: 0; padding: 0; box-sizing: border-box; }
  body {
    font-family: "SimSun", "宋体", "Noto Serif SC", serif;
    font-size: 12pt;
    line-height: 2;
    color: #333;
  }
  .cover {
    page: cover;
    page-break-after: always;
    text-align: center;
    padding-top: 35%;
  }
  .cover h1 { font-size: 24pt; font-weight: bold; margin-bottom: 1em; letter-spacing: 0.1em; }
  .cover .author { font-size: 14pt; color: #666; }
  .cover .description { font-size: 10pt; color: #999; margin-top: 2em; line-height: 1.8; }
  .cover-img {
    max-width: 60%;
    max-height: 30vh;
    margin-bottom: 2em;
    border-radius: 2px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.15);
    object-fit: contain;
  }
  .toc {
    page: toc;
    page-break-after: always;
    padding-top: 3cm;
  }
  .toc h2 {
    text-align: center;
    font-size: 18pt;
    margin-bottom: 1.5em;
    letter-spacing: 0.3em;
  }
  .toc-volume {
    font-size: 13pt;
    font-weight: bold;
    margin: 1em 0 0.5em 0;
    padding-left: 0.5em;
    border-left: 3px solid #999;
  }
  .toc-item {
    font-size: 11pt;
    padding: 0.25em 0 0.25em 1.5em;
    color: #444;
    border-bottom: 1px dotted #ddd;
  }
  .chapter { page-break-before: always; }
  .chapter-title { font-size: 16pt; font-weight: bold; margin-bottom: 1.5em; text-align: center; }
  .chapter-content p { text-indent: 2em; margin: 0; }
  .chapter-content h1, .chapter-content h2, .chapter-content h3 { text-indent: 0; }
  .chapter-content blockquote {
    border-left: 2px solid #ccc;
    padding-left: 1em;
    margin: 0.5em 0;
    color: #666;
  }
  @media print { body { -webkit-print-color-adjust: exact; } }
</style>
</head>
<body>
  <div class="cover">
    ${coverImgHtml}
    <h1>${escapeHtml(bookName.value)}</h1>
    <p class="author">${escapeHtml(author)}</p>
    ${description ? `<p class="description">${escapeHtml(description)}</p>` : ''}
  </div>
  <div class="toc">
    <h2>目 录</h2>
    ${tocHtml}
  </div>
  ${chapterHtml}
</body>
</html>`
}

function markdownToPrintHtml(markdown: string): string {
  let html = markdown
  // 移除灵感注释
  html = html.replace(/<!--inspiration:[\s\S]*?-->[\s\S]*?<!--\/inspiration-->/g, '')
  html = html.replace(/<!--[\s\S]*?-->/g, '')
  // 先处理块级元素
  html = html.replace(/^#### (.+)$/gm, '<h4>$1</h4>')
  html = html.replace(/^### (.+)$/gm, '<h3>$1</h3>')
  html = html.replace(/^## (.+)$/gm, '<h2>$1</h2>')
  html = html.replace(/^# (.+)$/gm, '<h1>$1</h1>')
  html = html.replace(/^> (.+)$/gm, '<blockquote>$1</blockquote>')
  html = html.replace(/^---$/gm, '<hr>')
  // 处理行内样式
  html = html.replace(/\*\*\*(.+?)\*\*\*/g, '<strong><em>$1</em></strong>')
  html = html.replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>')
  html = html.replace(/\*(.+?)\*/g, '<em>$1</em>')
  // 将段落包裹在 <p> 中，但跳过已包含块级标签的行
  const lines = html.split('\n')
  const result: string[] = []
  for (const line of lines) {
    const trimmed = line.trim()
    if (!trimmed) {
      result.push('')
      continue
    }
    // 已经是块级标签的行不再包裹
    if (/^<(h[1-4]|blockquote|hr|section|div)/.test(trimmed)) {
      result.push(line)
    } else {
      result.push(`<p>${trimmed}</p>`)
    }
  }
  html = result.join('\n')
  html = html.replace(/<p>\s*<\/p>/g, '')
  return html
}

function escapeHtml(text: string): string {
  return text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
}

// ========== EPUB 导出 ==========

async function handleExportEpub() {
  if (!workspace.currentWorkspace) return
  exporting.value = true
  exportStatus.value = '正在生成 EPUB…'

  try {
    const filePath = await save({
      defaultPath: `${bookName.value}.epub`,
      filters: [{ name: 'EPUB 电子书', extensions: ['epub'] }],
    })

    if (!filePath) {
      exporting.value = false
      exportStatus.value = ''
      return
    }

    await invoke('export_epub', {
      request: {
        book_path: workspace.currentWorkspace.path,
        output_path: filePath,
        format: 'epub',
      },
    })

    exportStatus.value = `✓ EPUB 已导出到：${filePath}`
  } catch (e) {
    exportStatus.value = `✗ 导出失败：${e}`
  } finally {
    exporting.value = false
  }
}

// ========== ZIP 恢复 ==========

async function handleRestore() {
  if (!workspace.currentWorkspace) return
  exporting.value = true
  exportStatus.value = '正在选择备份文件…'

  try {
    const filePath = await open({
      filters: [{ name: 'ZIP 压缩包', extensions: ['zip'] }],
      multiple: false,
    })

    if (!filePath) {
      exporting.value = false
      exportStatus.value = ''
      return
    }

    exportStatus.value = '正在恢复备份…'
    const selectedPath = typeof filePath === 'string' ? filePath : (filePath as { path: string }).path
    await invoke('restore_from_zip', {
      request: {
        zip_path: selectedPath,
        target_path: workspace.currentWorkspace.path,
      },
    })

    exportStatus.value = '✓ 备份已恢复成功！请重新打开当前作品'
    showRestore.value = false
  } catch (e) {
    exportStatus.value = `✗ 恢复失败：${e}`
  } finally {
    exporting.value = false
  }
}
</script>

<template>
  <div class="export-dialog-overlay" @click.self="$emit('close')">
    <div class="export-dialog">
      <div class="dialog-header">
        <h2>导出 / 备份</h2>
        <button class="close-btn" @click="$emit('close')">✕</button>
      </div>

      <div class="dialog-body">
        <div class="book-info">
          <span class="book-name">{{ bookName }}</span>
          <span class="book-type">{{ bookType }}</span>
        </div>

        <div class="export-options">
          <button class="export-option" @click="handleExportTxt" :disabled="exporting">
            <span class="option-icon">≡</span>
            <div class="option-info">
              <span class="option-title">导出 TXT</span>
              <span class="option-desc">按章节顺序合并，清洗灵感标记和 Markdown 语法</span>
            </div>
          </button>

          <button class="export-option" @click="handleExportPdf" :disabled="exporting">
            <span class="option-icon">◼</span>
            <div class="option-info">
              <span class="option-title">导出 PDF</span>
              <span class="option-desc">A5 出版级排版，章节自动分页，页眉页脚</span>
            </div>
          </button>

          <button class="export-option" @click="handleExportEpub" :disabled="exporting">
            <span class="option-icon">▣</span>
            <div class="option-info">
              <span class="option-title">导出 EPUB</span>
              <span class="option-desc">电子书格式，支持 Kindle、Apple Books、微信读书</span>
            </div>
          </button>

          <button class="export-option" @click="handleExportZip" :disabled="exporting">
            <span class="option-icon">⊞</span>
            <div class="option-info">
              <span class="option-title">ZIP 全量备份</span>
              <span class="option-desc">打包所有 .md 文件、book.json 和历史快照</span>
            </div>
          </button>

          <button class="export-option restore" @click="showRestore = true" :disabled="exporting">
            <span class="option-icon">↲</span>
            <div class="option-info">
              <span class="option-title">从备份恢复</span>
              <span class="option-desc">选择 ZIP 文件，覆盖当前作品数据</span>
            </div>
          </button>
        </div>

        <div v-if="exportStatus" class="export-status" :class="{ error: exportStatus.startsWith('✗') || exportStatus.startsWith('⚠') }">
          {{ exportStatus }}
        </div>

        <div v-if="showRestore" class="restore-warning">
          <p>⚠ 恢复操作将覆盖当前所有文件，此操作不可撤销。</p>
          <div class="restore-actions">
            <button class="btn-cancel" @click="showRestore = false">取消</button>
            <button class="btn-danger" @click="handleRestore">确认恢复</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.export-dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.3);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 300;
}

.export-dialog {
  background: var(--surface-color);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  width: 480px;
  max-width: 90vw;
  max-height: 85vh;
  overflow-y: auto;
}

.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px 24px 0;
}

.dialog-header h2 {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-color);
}

.close-btn {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  font-size: 14px;
  color: var(--text-color);
  transition: background 0.15s;
}
.close-btn:hover { background: var(--border-color); }

.dialog-body { padding: 20px 24px 24px; }

.book-info {
  display: flex;
  align-items: baseline;
  gap: 10px;
  margin-bottom: 20px;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--border-color);
}

.book-name { font-size: 16px; font-weight: 600; color: var(--text-color); }
.book-type { font-size: 12px; color: var(--text-color); opacity: 0.5; }

.export-options { display: flex; flex-direction: column; gap: 8px; }

.export-option {
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 14px 16px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  text-align: left;
  transition: all 0.15s;
  width: 100%;
  color: var(--text-color);
}
.export-option:hover:not(:disabled) {
  border-color: var(--accent-color);
  background: rgba(139, 115, 85, 0.04);
}
.export-option:disabled { opacity: 0.5; cursor: default; }
.export-option.restore:hover:not(:disabled) { border-color: var(--danger-color); }

.option-icon { font-size: 24px; flex-shrink: 0; }
.option-info { display: flex; flex-direction: column; gap: 2px; }
.option-title { font-size: 14px; font-weight: 500; }
.option-desc { font-size: 12px; opacity: 0.5; line-height: 1.4; }

.export-status {
  margin-top: 16px;
  padding: 10px 14px;
  background: rgba(139, 115, 85, 0.08);
  border-radius: 6px;
  font-size: 13px;
  color: var(--accent-color);
  line-height: 1.5;
}
.export-status.error {
  background: rgba(192, 57, 43, 0.08);
  color: var(--danger-color);
}

.restore-warning {
  margin-top: 16px;
  padding: 14px;
  border: 1px solid var(--danger-color);
  border-radius: 8px;
  background: rgba(192, 57, 43, 0.04);
}
.restore-warning p { font-size: 13px; color: var(--danger-color); margin-bottom: 12px; }
.restore-actions { display: flex; justify-content: flex-end; gap: 8px; }

.btn-cancel {
  padding: 6px 16px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 13px;
  color: var(--text-color);
}
.btn-cancel:hover { border-color: var(--text-color); }

.btn-danger {
  padding: 6px 16px;
  background: var(--danger-color);
  color: #fff;
  border-radius: 4px;
  font-size: 13px;
}
.btn-danger:hover { opacity: 0.9; }
</style>