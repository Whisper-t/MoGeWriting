<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { ask } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import { useRouter } from 'vue-router'
import { useWorkspaceStore } from '@/stores/workspace'
import { useEditorStore } from '@/stores/editor'
import { useThemeStore } from '@/stores/theme'
import { useAnnotationStore } from '@/stores/annotation'
import { useShortcutStore } from '@/stores/shortcut'
import Sidebar from '@/components/Sidebar.vue'
import TimelineView from '@/components/TimelineView.vue'
import InspectorPanel from '@/components/InspectorPanel.vue'
import StatusBar from '@/components/StatusBar.vue'
import TiptapEditor from '@/components/TiptapEditor.vue'
import ThemeSettings from '@/components/ThemeSettings.vue'
import BookInfoEditor from '@/components/BookInfoEditor.vue'
import ExportDialog from '@/components/ExportDialog.vue'
import ShortcutGuide from '@/components/ShortcutGuide.vue'
import HistoryPanel from '@/components/HistoryPanel.vue'

const router = useRouter()
const workspace = useWorkspaceStore()
const editor = useEditorStore()
const theme = useThemeStore()
const annotation = useAnnotationStore()
const shortcutStore = useShortcutStore()

// 编辑器组件引用
const tiptapRef = ref<InstanceType<typeof TiptapEditor> | null>(null)

// UI 状态
const showSidebar = ref(true)
const showTimeline = ref(false)
const showInspector = ref(false)
const showThemeSettings = ref(false)
const showBookInfo = ref(false)
const showExport = ref(false)
const showShortcutGuide = ref(false)
const showHistory = ref(false)
const showSearch = ref(false)
const searchQuery = ref('')
const replaceQuery = ref('')
const searchIndex = ref(0)
const searchResults = ref<{ from: number; to: number }[]>([])
const focusMode = ref(false)

function toggleFocusMode() {
  focusMode.value = !focusMode.value
  if (focusMode.value) {
    showSidebar.value = false
    showTimeline.value = false
    showInspector.value = false
    showThemeSettings.value = false
    showBookInfo.value = false
  }
}

/** 切换到时间线视图 */
function handleSwitchToTimeline() {
  showTimeline.value = true
  showSidebar.value = false
}

/** 从时间线返回目录 */
function handleBackToSidebar() {
  showTimeline.value = false
  showSidebar.value = true
}

/** 从时间线选择随笔后切回编辑器 */
function handleTimelineSelect() {
  showTimeline.value = false
  showSidebar.value = true
}

/** 在资源管理器中打开当前书籍文件夹 */
async function openInExplorer() {
  if (!workspace.currentWorkspace) return
  try {
    await invoke('open_in_explorer', { path: workspace.currentWorkspace.path })
  } catch (e) {
    console.error('打开资源管理器失败:', e)
  }
}

// ========== 搜索与替换 ==========

function doSearch() {
  if (!tiptapRef.value || !searchQuery.value) {
    searchResults.value = []
    return
  }
  // 获取编辑器纯文本，查找所有匹配位置
  const editorEl = document.querySelector('.tiptap-editor')
  if (!editorEl) return
  const text = editorEl.textContent || ''
  const results: { from: number; to: number }[] = []
  const query = searchQuery.value
  let idx = 0
  while ((idx = text.indexOf(query, idx)) !== -1) {
    results.push({ from: idx, to: idx + query.length })
    idx += query.length
  }
  searchResults.value = results
  searchIndex.value = results.length > 0 ? 0 : -1
  if (results.length > 0) {
    highlightMatch(0)
  }
}

function highlightMatch(index: number) {
  if (index < 0 || index >= searchResults.value.length) return
  const editorEl = document.querySelector('.tiptap-editor')
  if (!editorEl) return
  const { from } = searchResults.value[index]
  const query = searchQuery.value
  // 创建 Range 来选中匹配文字
  const walker = document.createTreeWalker(editorEl, NodeFilter.SHOW_TEXT)
  let charCount = 0
  let startNode: Text | null = null
  let startOffset = 0
  let endNode: Text | null = null
  let endOffset = 0
  while (walker.nextNode()) {
    const node = walker.currentNode as Text
    const nodeLen = node.textContent?.length || 0
    if (!startNode && charCount + nodeLen > from) {
      startNode = node
      startOffset = from - charCount
    }
    if (!endNode && charCount + nodeLen >= from + query.length) {
      endNode = node
      endOffset = from + query.length - charCount
      break
    }
    charCount += nodeLen
  }
  if (startNode && endNode) {
    const range = document.createRange()
    range.setStart(startNode, startOffset)
    range.setEnd(endNode, endOffset)
    const sel = window.getSelection()
    sel?.removeAllRanges()
    sel?.addRange(range)
    // 滚动到视图
    const rect = range.getBoundingClientRect()
    if (rect.top < 0 || rect.bottom > window.innerHeight) {
      startNode.parentElement?.scrollIntoView({ behavior: 'smooth', block: 'center' })
    }
  }
  searchIndex.value = index
}

function nextMatch() {
  if (searchResults.value.length === 0) return
  const next = (searchIndex.value + 1) % searchResults.value.length
  highlightMatch(next)
}

function prevMatch() {
  if (searchResults.value.length === 0) return
  const prev = (searchIndex.value - 1 + searchResults.value.length) % searchResults.value.length
  highlightMatch(prev)
}

function replaceCurrent() {
  if (!tiptapRef.value || searchResults.value.length === 0) return
  const editorEl = document.querySelector('.tiptap-editor .ProseMirror') as HTMLElement
  if (!editorEl) return
  // 用 execCommand 替换选中文字
  document.execCommand('insertText', false, replaceQuery.value)
  // 重新搜索
  setTimeout(() => doSearch(), 50)
}

function replaceAll() {
  if (!tiptapRef.value || searchResults.value.length === 0) return
  // 获取编辑器 HTML，全局替换
  const editorEl = document.querySelector('.tiptap-editor .ProseMirror') as HTMLElement
  if (!editorEl) return
  const html = editorEl.innerHTML
  const escaped = searchQuery.value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
  // 简单替换：在 HTML 中替换纯文本匹配
  // 注意：这可能会错误匹配 HTML 标签内的文本，但对于编辑器生成的 HTML 相对安全
  editorEl.innerHTML = html.replace(
    new RegExp(`(>([^<]*))${escaped}`, 'g'),
    `$1${replaceQuery.value}`,
  )
  doSearch()
}

function closeSearch() {
  showSearch.value = false
  searchQuery.value = ''
  replaceQuery.value = ''
  searchResults.value = []
  window.getSelection()?.removeAllRanges()
}

onMounted(() => {
  theme.applyTheme()
  if (!workspace.currentWorkspace) {
    router.push('/')
  }
  editor.startTimer()
})

onUnmounted(() => {
  editor.stopTimer()
})

async function handleBack() {
  if (editor.isDirty) {
    const confirmed = await ask('您有未保存的修改，确定要退出吗？未保存的内容将会丢失。', {
      title: '未保存警告',
      kind: 'warning',
    })
    if (!confirmed) return
  }
  
  editor.stopTimer()
  workspace.closeWorkspace()
  router.push('/')
}

function handleContentChange(markdown: string) {
  editor.setContent(markdown)
}

function handleWordCount(_count: number) {
  // 字数由 TiptapEditor 的 CharacterCount 扩展提供
  // 但完整字数统计由 editor.store.updateWordCount() 基于 Markdown 内容计算
  editor.updateWordCount()
}

// ========== 批注操作 ==========

/** 添加新批注到当前选中文字 */
function handleAddAnnotation() {
  if (!tiptapRef.value) return
  const id = crypto.randomUUID()
  const success = tiptapRef.value.addAnnotation(id)
  if (success) {
    showInspector.value = true
    annotation.openPanel()
  }
}

/** 点击侧边栏批注卡片时，滚动到对应文字 */
function handleAnnotationClick(annotationId: string) {
  tiptapRef.value?.scrollToAnnotation(annotationId)
}

/** 悬停批注卡片时高亮对应文字 */
function handleAnnotationHover(annotationId: string | null) {
  annotation.setHovered(annotationId)
}

// 自动保存（每10分钟）
let autoSaveInterval: ReturnType<typeof setInterval> | null = null
onMounted(() => {
  autoSaveInterval = setInterval(async () => {
    if (editor.isDirty) {
      await editor.saveFile()
    }
  }, 600000)
})
onUnmounted(() => {
  if (autoSaveInterval) clearInterval(autoSaveInterval)
})

// 快捷键
function handleKeydown(e: KeyboardEvent) {
  if (shortcutStore.match(e, shortcutStore.shortcuts.save)) {
    e.preventDefault()
    editor.saveFile()
  }
  if (shortcutStore.match(e, shortcutStore.shortcuts.addAnnotation)) {
    e.preventDefault()
    handleAddAnnotation()
  }
  if (shortcutStore.match(e, shortcutStore.shortcuts.export)) {
    e.preventDefault()
    showExport.value = true
  }
  if (shortcutStore.match(e, shortcutStore.shortcuts.focusMode)) {
    e.preventDefault()
    toggleFocusMode()
  }
  if (shortcutStore.match(e, shortcutStore.shortcuts.newChapter)) {
    e.preventDefault()
    workspace.createChapter('无题')
  }
  // Ctrl+F 搜索
  if ((e.ctrlKey || e.metaKey) && e.key === 'f') {
    e.preventDefault()
    showSearch.value = true
  }
  // Escape 关闭搜索
  if (e.key === 'Escape' && showSearch.value) {
    closeSearch()
  }
}
// 使用捕获阶段确保即使焦点在编辑器内也能生效
onMounted(() => window.addEventListener('keydown', handleKeydown, true))
onUnmounted(() => window.removeEventListener('keydown', handleKeydown, true))
</script>

<template>
  <div class="editor-layout" v-if="workspace.currentWorkspace">
    <!-- 左侧边栏 / 时间线 -->
    <Sidebar
      v-if="showSidebar && !showTimeline"
      @toggle="showSidebar = false"
      @switchToTimeline="handleSwitchToTimeline"
    />
    <TimelineView
      v-if="showTimeline"
      @back="handleBackToSidebar"
      @selectChapter="handleTimelineSelect"
    />

    <!-- 主编辑区 -->
    <div class="editor-main">
      <!-- 顶栏（专注模式隐藏） -->
      <header v-if="!focusMode" class="editor-header">
        <div class="header-left">
          <button
            v-if="!showSidebar"
            class="icon-btn"
            @click="showSidebar = true"
            title="显示目录"
          >
            ☰
          </button>
          <button class="icon-btn" @click="handleBack" title="返回首页">
            ←
          </button>
          <span class="current-title">
            {{ workspace.selectedChapter?.title || workspace.currentWorkspace.name }}
          </span>
          <span v-if="editor.isDirty" class="dirty-mark" title="有未保存的修改">●</span>
        </div>
        <div class="header-right">
          <button
            class="icon-btn"
            style="width: auto; padding: 0 10px; font-size: 13px; gap: 4px;"
            @click="showBookInfo = !showBookInfo"
          >
            ⓘ 书籍信息
          </button>
          <button
            class="icon-btn"
            @click="openInExplorer"
            title="在资源管理器中打开"
          >
            ◰
          </button>
          <button
            class="icon-btn"
            @click="showExport = true"
            title="导出 / 备份"
          >
            ⇡
          </button>
          <button
            class="icon-btn"
            @click="showHistory = true"
            title="历史版本"
          >
            ⌛
          </button>
          <button
            class="icon-btn"
            @click="toggleFocusMode"
            title="专注模式 (F11)"
          >
            ◻
          </button>
          <button
            class="icon-btn"
            @click="showThemeSettings = !showThemeSettings"
            :title="showThemeSettings ? '关闭排版设置' : '排版设置'"
          >
            ⚙
          </button>
          <button
            class="icon-btn"
            @click="showShortcutGuide = true"
            title="快捷键"
          >
            ⌨
          </button>
          <button
            class="icon-btn"
            @click="showInspector = !showInspector"
            :title="showInspector ? '关闭灵感面板' : '灵感面板'"
          >
            ◇
          </button>
          <button
            class="btn-save"
            @click="editor.saveFile()"
            :disabled="!editor.isDirty"
          >
            {{ editor.saving ? '保存中…' : '保存' }}
          </button>
        </div>
      </header>

      <!-- 排版设置面板 -->
      <ThemeSettings v-if="showThemeSettings && !focusMode" @close="showThemeSettings = false" />

      <!-- 书籍简介面板 -->
      <BookInfoEditor v-if="showBookInfo && !focusMode" @close="showBookInfo = false" />

      <!-- 搜索替换面板 -->
      <div v-if="showSearch" class="search-panel">
        <div class="search-row">
          <input
            ref="searchInput"
            v-model="searchQuery"
            placeholder="查找…"
            @input="doSearch"
            @keyup.enter="nextMatch"
            autofocus
          />
          <span class="search-count" v-if="searchResults.length > 0">
            {{ searchIndex + 1 }}/{{ searchResults.length }}
          </span>
          <button class="search-btn" @click="prevMatch" title="上一个">↑</button>
          <button class="search-btn" @click="nextMatch" title="下一个">↓</button>
          <button class="search-btn" @click="closeSearch" title="关闭">✕</button>
        </div>
        <div class="search-row" v-if="searchResults.length > 0">
          <input
            v-model="replaceQuery"
            placeholder="替换为…"
            @keyup.enter="replaceCurrent"
          />
          <button class="search-btn replace" @click="replaceCurrent">替换</button>
          <button class="search-btn replace-all" @click="replaceAll">全部替换</button>
        </div>
      </div>

      <!-- 编辑器 -->
      <div class="editor-content">
        <TiptapEditor
          v-if="workspace.selectedChapter"
          ref="tiptapRef"
          :content="editor.content"
          :file-path="editor.currentFilePath"
          :show-grid="theme.config.showGrid"
          @update:content="handleContentChange"
          @word-count="handleWordCount"
        />
        <div v-else class="empty-editor">
          <span class="empty-icon">·</span>
          <p>请在左侧侧边栏选择或新建一个章节开始写作</p>
        </div>
      </div>

      <!-- 状态栏（专注模式隐藏） -->
      <StatusBar v-if="!focusMode" />
    </div>

    <!-- 右侧灵感面板 -->
    <InspectorPanel
      v-if="showInspector"
      @close="showInspector = false"
      @click-annotation="handleAnnotationClick"
      @hover-annotation="handleAnnotationHover"
      @add-annotation="handleAddAnnotation"
    />

    <!-- 导出对话框 -->
    <ExportDialog
      v-if="showExport"
      @close="showExport = false"
    />

    <!-- 快捷键指南 -->
    <ShortcutGuide
      v-if="showShortcutGuide"
      @close="showShortcutGuide = false"
    />

    <!-- 历史版本面板 -->
    <HistoryPanel
      v-if="showHistory"
      @close="showHistory = false"
    />
  </div>
</template>

<style scoped>
.editor-layout {
  height: 100%;
  display: flex;
  background: var(--bg-color);
}

.editor-main {
  flex: 1;
  display: flex;
  position: relative;
  flex-direction: column;
  min-width: 0;
}

.editor-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px;
  height: 44px;
  border-bottom: 1px solid var(--border-color);
  flex-shrink: 0;
  user-select: none;
}

.header-left,
.header-right {
  display: flex;
  align-items: center;
  gap: 4px;
}

.icon-btn {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
  font-size: 16px;
  transition: background 0.15s;
  color: var(--text-color);
}
.icon-btn:hover {
  background: var(--border-color);
}

.current-title {
  font-size: 14px;
  font-weight: 500;
  margin-left: 8px;
  color: var(--text-color);
  opacity: 0.8;
  max-width: 300px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.dirty-mark {
  color: var(--accent-color);
  font-size: 10px;
  margin-left: 4px;
  animation: pulse 1.5s infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.3; }
}

.btn-save {
  padding: 4px 14px;
  background: var(--accent-color);
  color: #fff;
  border-radius: 4px;
  font-size: 13px;
  transition: background 0.2s;
}
.btn-save:hover {
  background: var(--accent-hover);
}
.btn-save:disabled {
  opacity: 0.4;
  cursor: default;
}

.editor-content {
  flex: 1;
  overflow: hidden;
  position: relative;
}

/* 搜索面板 */
.search-panel {
  padding: 8px 16px;
  border-bottom: 1px solid var(--border-color);
  background: var(--surface-color);
  flex-shrink: 0;
}

.search-row {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: 4px;
}
.search-row:last-child { margin-bottom: 0; }

.search-row input {
  flex: 1;
  padding: 4px 10px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 13px;
  background: var(--bg-color);
  color: var(--text-color);
  outline: none;
}
.search-row input:focus {
  border-color: var(--accent-color);
}

.search-count {
  font-size: 11px;
  color: var(--text-color);
  opacity: 0.5;
  white-space: nowrap;
  min-width: 36px;
  text-align: center;
}

.search-btn {
  width: 26px;
  height: 26px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  font-size: 12px;
  color: var(--text-color);
  transition: background 0.15s;
  flex-shrink: 0;
}
.search-btn:hover { background: var(--border-color); }

.search-btn.replace,
.search-btn.replace-all {
  width: auto;
  padding: 0 10px;
  font-size: 12px;
}
.search-btn.replace-all {
  background: var(--accent-color);
  color: #fff;
}
.search-btn.replace-all:hover {
  background: var(--accent-hover);
}

.empty-editor {
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: var(--text-color);
  opacity: 0.5;
  gap: 16px;
  background: var(--bg-color);
}
.empty-editor .empty-icon {
  font-size: 48px;
  opacity: 0.8;
}
</style>