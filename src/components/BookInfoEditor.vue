<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke, convertFileSrc } from '@tauri-apps/api/core'
import { useWorkspaceStore } from '@/stores/workspace'

defineEmits<{ close: [] }>()

const workspace = useWorkspaceStore()

const description = ref(workspace.bookIndex?.description || '')
const author = ref(workspace.bookIndex?.author || '')
const autoNumbering = ref(workspace.bookIndex?.auto_numbering ?? false)
const saving = ref(false)
const saveSuccess = ref(false)

// 监听 bookIndex 变化（外部更新时同步）
watch(
  () => workspace.bookIndex,
  (book) => {
    if (book) {
      description.value = book.description || ''
      author.value = book.author || ''
      autoNumbering.value = book.auto_numbering ?? false
    }
  },
  { immediate: true },
)

async function save() {
  if (!workspace.bookIndex || !workspace.currentWorkspace) return
  saving.value = true
  try {
    const updated = { ...workspace.bookIndex }
    updated.description = description.value.trim() || undefined
    updated.author = author.value.trim() || undefined
    updated.auto_numbering = autoNumbering.value
    await workspace.writeBookIndex(updated)
    
    saveSuccess.value = true
    setTimeout(() => {
      saveSuccess.value = false
    }, 2000)
  } catch (e) {
    console.error('Failed to save book info:', e)
  } finally {
    saving.value = false
  }
}

function formatDate(dateStr: string): string {
  const d = new Date(dateStr)
  return d.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}

function getCoverUrl(): string {
  const cover = workspace.bookIndex?.cover_image
  if (!cover || !workspace.currentWorkspace) return ''
  return convertFileSrc(`${workspace.currentWorkspace.path}\\${cover}`)
}

async function selectCover() {
  try {
    const filePath = await open({
      filters: [{ name: 'Image', extensions: ['png', 'jpg', 'jpeg', 'webp'] }],
      multiple: false,
    })
    if (filePath) {
      const selectedPath = typeof filePath === 'string' ? filePath : (filePath as any).path
      await workspace.setCoverImage(selectedPath)
    }
  } catch (e) {
    console.error('Failed to select cover:', e)
  }
}

const totalWords = ref(0)

// 异步获取全书总字数
async function loadTotalWords() {
  if (!workspace.currentWorkspace) return
  try {
    totalWords.value = await invoke<number>('count_words', {
      bookPath: workspace.currentWorkspace.path,
    })
  } catch {
    totalWords.value = 0
  }
}

onMounted(() => {
  loadTotalWords()
})

// 每次打开面板时刷新字数
watch(
  () => workspace.currentWorkspace,
  () => {
    loadTotalWords()
  },
)
const totalChapters = workspace.bookIndex?.volumes
  ? workspace.bookIndex.volumes.reduce((sum, v) => sum + v.chapters.length, 0)
  : workspace.bookIndex?.chapters?.length || 0
</script>

<template>
  <div class="book-info">
    <div class="info-header">
      <span>书籍信息</span>
      <button class="close-btn" @click="$emit('close')">✕</button>
    </div>

    <div class="info-body">
      <!-- 基本信息 -->
      <div class="info-row">
        <span class="info-label">书名</span>
        <span class="info-value">{{ workspace.bookIndex?.name }}</span>
      </div>
      <div class="info-row">
        <span class="info-label">类型</span>
        <span class="info-value">
          {{ workspace.bookIndex?.type === 'novel' ? '小说' : '随笔' }}
        </span>
      </div>
      <div class="info-row">
        <span class="info-label">创建</span>
        <span class="info-value">
          {{ workspace.bookIndex?.created_at ? formatDate(workspace.bookIndex.created_at) : '-' }}
        </span>
      </div>
      <div class="info-row">
        <span class="info-label">章节</span>
        <span class="info-value">{{ totalChapters }} 章</span>
      </div>
      <div class="info-row">
        <span class="info-label">字数</span>
        <span class="info-value">{{ totalWords.toLocaleString() }} 字</span>
      </div>

      <!-- 作者 -->
      <div class="info-field">
        <label>作者</label>
        <input
          v-model="author"
          placeholder="未设置"
          @blur="save"
        />
      </div>

      <!-- 封面设置 -->
      <div class="info-field">
        <label>封面图片</label>
        <div class="cover-setting">
          <img v-if="workspace.bookIndex?.cover_image" :src="getCoverUrl()" class="cover-preview" alt="cover" />
          <div v-else class="cover-placeholder">暂无封面</div>
          <button class="btn-secondary btn-sm" @click="selectCover">选择图片</button>
        </div>
      </div>

      <!-- 自动编号 -->
      <div class="info-field toggle-field">
        <label class="toggle-label">
          <input type="checkbox" v-model="autoNumbering" @change="save" />
          <span>自动添加「第X章/卷」编号</span>
        </label>
      </div>

      <!-- 简介/大纲 -->
      <div class="info-field">
        <label>简介 / 大纲</label>
        <textarea
          v-model="description"
          placeholder="写下这本书的简介、大纲或创作思路…"
          rows="6"
          @blur="save"
        />
      </div>

      <span class="save-indicator" :class="{ visible: saveSuccess }">
        ✓ 已自动保存
      </span>
    </div>
  </div>
</template>

<style scoped>
.book-info {
  position: absolute;
  top: 45px;
  bottom: 0;
  left: 0;
  right: 0;
  z-index: 10;
  background: var(--bg-color);
  user-select: none;
  overflow-y: auto;
}

.info-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 16px;
  font-size: 13px;
  font-weight: 600;
  color: var(--text-color);
}

.close-btn {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  font-size: 12px;
  color: var(--text-color);
  transition: background 0.15s;
}
.close-btn:hover {
  background: var(--border-color);
}

.info-body {
  padding: 0 16px 16px;
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 10px;
}

.info-row {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.info-label {
  font-size: 10px;
  color: var(--text-color);
  opacity: 0.5;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.info-value {
  font-size: 13px;
  color: var(--text-color);
}

.info-field {
  grid-column: 1 / -1;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.info-field label {
  font-size: 10px;
  color: var(--text-color);
  opacity: 0.5;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.info-field input,
.info-field textarea {
  width: 100%;
  padding: 8px 10px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--bg-color);
  color: var(--text-color);
  font-size: 13px;
  font-family: inherit;
  line-height: 1.6;
  outline: none;
  resize: vertical;
}
.info-field input:focus,
.info-field textarea:focus {
  border-color: var(--accent-color);
}

.save-indicator {
  grid-column: 1 / -1;
  justify-self: end;
  font-size: 12px;
  color: #10b981;
  opacity: 0;
  transform: translateY(4px);
  transition: all 0.3s;
  pointer-events: none;
}
.save-indicator.visible {
  opacity: 1;
  transform: translateY(0);
}

.cover-setting {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-top: 4px;
}
.cover-preview {
  width: 96px;
  height: 128px;
  object-fit: cover;
  border-radius: 4px;
  border: 1px solid var(--border-color);
}
.cover-placeholder {
  width: 96px;
  height: 128px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 10px;
  color: var(--text-color);
  opacity: 0.4;
  background: var(--bg-color);
  border: 1px dashed var(--border-color);
  border-radius: 4px;
}

.btn-secondary {
  padding: 6px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: transparent;
  color: var(--text-color);
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s;
}
.btn-secondary:hover {
  border-color: var(--accent-color);
  color: var(--accent-color);
}

.toggle-field {
  margin-top: 8px;
  margin-bottom: 8px;
}
.toggle-label {
  display: flex !important;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  font-size: 13px !important;
  text-transform: none !important;
  letter-spacing: normal !important;
  opacity: 0.8 !important;
}
.toggle-label input[type="checkbox"] {
  width: auto;
  accent-color: var(--accent-color);
}
</style>