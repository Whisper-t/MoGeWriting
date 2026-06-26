<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke, convertFileSrc } from '@tauri-apps/api/core'
import { useWorkspaceStore } from '@/stores/workspace'
import { useThemeStore } from '@/stores/theme'
import type { WorkspaceSummary } from '@/types'
import ShortcutGuide from '@/components/ShortcutGuide.vue'

const router = useRouter()
const workspace = useWorkspaceStore()
const theme = useThemeStore()

// 新建对话框
const showCreateDialog = ref(false)
const showShortcutGuide = ref(false)
const newName = ref('')
const newType = ref<string>('小说')
const newStructure = ref<'volume' | 'flat'>('volume')
const newDescription = ref('')
const newAuthor = ref('')

const deletingPath = ref<string | null>(null)

// 恢复状态
const restoring = ref(false)
const restoreStatus = ref('')

const showThemeDropdown = ref(false)

async function openRootFolder() {
  try {
    await invoke('open_in_explorer', { path: workspace.rootPath })
  } catch (e) {
    console.error('打开目录失败:', e)
  }
}

async function handleRestore() {
  restoring.value = true
  restoreStatus.value = '正在选择备份文件…'
  try {
    const filePath = await open({
      filters: [{ name: 'ZIP 压缩包', extensions: ['zip'] }],
      multiple: false,
    })
    if (!filePath) {
      restoring.value = false
      restoreStatus.value = ''
      return
    }
    restoreStatus.value = '正在恢复备份…'
    const selectedPath = typeof filePath === 'string' ? filePath : (filePath as { path: string }).path
    await invoke('restore_from_zip', {
      request: {
        zip_path: selectedPath,
        target_path: workspace.rootPath,
      },
    })
    restoreStatus.value = '✓ 备份恢复成功！'
    await workspace.refreshWorkspaces()
  } catch (e) {
    restoreStatus.value = `✗ 恢复失败：${e}`
  } finally {
    restoring.value = false
  }
}

onMounted(() => {
  theme.applyTheme()
  document.addEventListener('click', () => { showThemeDropdown.value = false })
})

async function handleCreate() {
  if (!newName.value.trim()) return
  await workspace.createWorkspace(
    newName.value.trim(),
    newType.value,
    newStructure.value,
    newDescription.value.trim() || undefined,
    newAuthor.value.trim() || undefined
  )
  showCreateDialog.value = false
  newName.value = ''
  newDescription.value = ''
  newAuthor.value = ''
}

async function handleOpen(ws: WorkspaceSummary) {
  await workspace.openWorkspace(ws)
  router.push('/editor')
}

async function handleDelete(path: string) {
  await workspace.deleteWorkspace(path)
  deletingPath.value = null
}

function formatDate(dateStr: string): string {
  const d = new Date(dateStr)
  return d.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}

function getCoverUrl(ws: WorkspaceSummary): string {
  if (!ws.cover_image) return ''
  // Use convertFileSrc to load local files in Tauri
  const coverPath = `${ws.path}\\${ws.cover_image}`
  return convertFileSrc(coverPath)
}
</script>

<template>
  <div class="home">
    <!-- 顶栏 -->
    <header class="home-header">
      <h1 class="app-title">墨格</h1>
      <div class="header-actions">
        <button class="icon-btn" @click="showShortcutGuide = true" title="快捷键设置">
          ⌨
        </button>
        <div class="theme-dropdown-container">
          <button class="theme-btn" @click.stop="showThemeDropdown = !showThemeDropdown" :title="`当前: ${theme.config.mode}`">
            {{ theme.config.mode === 'light' ? '☀' : theme.config.mode === 'dark' ? '🌙' : '☕' }}
          </button>
          <div v-if="showThemeDropdown" class="theme-dropdown" @click.stop>
            <div class="theme-option" :class="{ active: theme.config.mode === 'light' }" @click="theme.setTheme('light'); showThemeDropdown = false">
              ☀ 浅色模式
            </div>
            <div class="theme-option" :class="{ active: theme.config.mode === 'dark' }" @click="theme.setTheme('dark'); showThemeDropdown = false">
              🌙 深色模式
            </div>
            <div class="theme-option" :class="{ active: theme.config.mode === 'sepia' }" @click="theme.setTheme('sepia'); showThemeDropdown = false">
              ☕ 护眼模式
            </div>
          </div>
        </div>
        <button class="btn-secondary" @click="openRootFolder" title="打开文件存储目录">
            ◰ 目录
          </button>
          <button class="btn-secondary" @click="handleRestore" :disabled="restoring" title="从 ZIP 备份恢复">
          {{ restoring ? '恢复中…' : '↲ 恢复' }}
        </button>
        <button class="btn-primary" @click="showCreateDialog = true">新建</button>
      </div>
    </header>

    <!-- 恢复状态 -->
    <div v-if="restoreStatus" class="restore-status" :class="{ error: restoreStatus.startsWith('✗') }">
      {{ restoreStatus }}
    </div>

    <!-- 工作区列表 -->
    <main class="home-main">
      <div v-if="workspace.loading" class="loading">加载中…</div>
      <div v-else-if="workspace.workspaces.length === 0" class="empty-state">
        <p class="empty-icon">·</p>
        <p class="empty-text">还没有作品</p>
        <p class="empty-hint">点击「新建」开始你的第一本书或随笔集</p>
      </div>
      <div v-else class="workspace-grid">
        <div
          v-for="ws in workspace.workspaces"
          :key="ws.path"
          class="workspace-card"
          @click="handleOpen(ws)"
        >
          <div class="card-cover-container">
            <img v-if="ws.cover_image" :src="getCoverUrl(ws)" class="card-cover" alt="cover" />
            <div v-else class="card-cover-placeholder"></div>
          </div>
          <div class="card-content">
            <div class="card-type-badge">
              {{ ws.work_type }}
            </div>
          <h3 class="card-title">{{ ws.name }}</h3>
          <p v-if="ws.description" class="card-desc">{{ ws.description }}</p>
          <div class="card-meta">
            <span>{{ ws.total_chapters }} 章</span>
            <span>{{ ws.total_words.toLocaleString() }} 字</span>
          </div>
          <div class="card-footer">
            <span class="card-date">{{ formatDate(ws.modified_at) }}</span>
            <button
              class="card-delete"
              @click.stop="deletingPath = ws.path"
              title="删除"
            >
              ×
            </button>
          </div>
            </div>
          </div>
        </div>
    </main>

    <!-- 新建对话框 -->
    <div v-if="showCreateDialog" class="dialog-overlay" @click.self="showCreateDialog = false">
      <div class="dialog">
        <h2 class="dialog-title">新建作品</h2>
        <div class="form-group">
          <label>名称</label>
          <input v-model="newName" placeholder="书名或随笔集名称" @keyup.enter="handleCreate" autofocus />
        </div>
        <div class="form-group">
          <label>类型</label>
          <div class="type-input-group">
            <input v-model="newType" placeholder="例如：小说、随笔、散文" />
            <div class="type-suggestions">
              <button @click="newType = '小说'; newStructure = 'volume'">小说</button>
              <button @click="newType = '随笔'; newStructure = 'flat'">随笔</button>
              <button @click="newType = '散文'; newStructure = 'flat'">散文</button>
            </div>
          </div>
        </div>
        <div class="form-group">
          <label>结构模式</label>
          <div class="structure-options">
            <label class="structure-radio">
              <input type="radio" v-model="newStructure" value="volume" />
              <span>分卷模式（适合长篇小说，支持分卷和章节）</span>
            </label>
            <label class="structure-radio">
              <input type="radio" v-model="newStructure" value="flat" />
              <span>平铺模式（适合短篇或随笔，直接新建篇章）</span>
            </label>
          </div>
        </div>
        <div class="form-group">
          <label>简介</label>
          <textarea v-model="newDescription" placeholder="可选，简短介绍" rows="2" />
        </div>
        <div class="form-group">
          <label>作者</label>
          <input v-model="newAuthor" placeholder="可选" />
        </div>
        <div class="dialog-actions">
          <button class="btn-secondary" @click="showCreateDialog = false">取消</button>
          <button class="btn-primary" @click="handleCreate" :disabled="!newName.trim()">创建</button>
        </div>
      </div>
    </div>

    <!-- 删除确认 -->
    <div v-if="deletingPath" class="dialog-overlay" @click.self="deletingPath = null">
      <div class="dialog dialog-sm">
        <p class="confirm-text">确定要删除这个作品吗</p>
        <p class="confirm-warn">此操作不可撤销，所有文件将被永久删除</p>
        <div class="dialog-actions">
          <button class="btn-secondary" @click="deletingPath = null">取消</button>
          <button class="btn-danger" @click="handleDelete(deletingPath)">确认删除</button>
        </div>
      </div>
    </div>

    <!-- 快捷键指南 -->
    <ShortcutGuide
      v-if="showShortcutGuide"
      @close="showShortcutGuide = false"
    />
  </div>
</template>

<style scoped>
.home {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--bg-color);
}

.home-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 24px 40px;
  border-bottom: 1px solid var(--border-color);
}

.app-title {
  font-size: 24px;
  font-weight: 600;
  color: var(--text-color);
  letter-spacing: 4px;
}

.header-actions {
  display: flex;
  gap: 12px;
  align-items: center;
}

.theme-btn {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 18px;
  transition: background 0.2s;
  background: transparent;
  border: none;
  color: var(--text-color);
  cursor: pointer;
}
.theme-btn:hover {
  background: var(--border-color);
}

.theme-dropdown-container {
  position: relative;
}

.theme-dropdown {
  position: absolute;
  top: 120%;
  left: 50%;
  transform: translateX(-50%);
  background: var(--surface-color);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0,0,0,0.1);
  padding: 8px;
  display: flex;
  flex-direction: column;
  gap: 4px;
  z-index: 50;
  min-width: 120px;
}

.theme-option {
  padding: 8px 12px;
  font-size: 13px;
  color: var(--text-color);
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: background 0.2s;
}

.theme-option:hover {
  background: var(--bg-color);
}

.theme-option.active {
  background: color-mix(in srgb, var(--accent-color) 10%, transparent);
  color: var(--accent-color);
  font-weight: 500;
}

.icon-btn {
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
  font-size: 18px;
  transition: background 0.15s;
  color: var(--text-color);
  background: transparent;
  border: none;
  cursor: pointer;
}
.icon-btn:hover {
  background: var(--border-color);
}

.home-main {
  flex: 1;
  overflow-y: auto;
  padding: 40px;
}

.loading, .empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 60vh;
  color: var(--text-color);
  opacity: 0.6;
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 16px;
}

.empty-text {
  font-size: 18px;
  margin-bottom: 8px;
}

.empty-hint {
  font-size: 14px;
}

.workspace-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  align-items: stretch;
  gap: 20px;
  max-width: 1200px;
  margin: 0 auto;
}

.workspace-card {
  background: var(--surface-color);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  overflow: hidden;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.25, 0.8, 0.25, 1);
  position: relative;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.02);
  display: flex;
  flex-direction: column;
  height: 100%;
}
.workspace-card:hover {
  border-color: var(--accent-color);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.06);
  transform: translateY(-2px);
}

.card-cover-container {
  width: 100%;
  height: 140px;
  overflow: hidden;
  border-bottom: 1px solid var(--border-color);
}
.card-cover {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.5s ease;
}
.card-cover-placeholder {
  width: 100%;
  height: 100%;
  background: repeating-linear-gradient(
    45deg,
    var(--bg-color),
    var(--bg-color) 10px,
    var(--border-color) 10px,
    var(--border-color) 20px
  );
  opacity: 0.3;
  transition: opacity 0.5s ease;
}
.workspace-card:hover .card-cover {
  transform: scale(1.05);
}
.workspace-card:hover .card-cover-placeholder {
  opacity: 0.5;
}

.card-content {
  padding: 24px;
  flex: 1;
  display: flex;
  flex-direction: column;
}

.card-type-badge {
  align-self: flex-start;
  display: inline-block;
  font-size: 11px;
  padding: 4px 10px;
  border-radius: 6px;
  margin-bottom: 16px;
  letter-spacing: 1px;
  background: color-mix(in srgb, var(--accent-color) 10%, transparent);
  color: var(--accent-color);
  font-weight: 500;
}

.card-title {
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 8px;
  color: var(--text-color);
}

.card-desc {
  font-size: 13px;
  color: var(--text-color);
  opacity: 0.6;
  margin-bottom: 16px;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.card-meta {
  display: flex;
  gap: 16px;
  font-size: 12px;
  color: var(--text-color);
  opacity: 0.5;
  margin-bottom: 12px;
  margin-top: auto;
}

.card-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: 12px;
  color: var(--text-color);
  opacity: 0.4;
}

.card-delete {
  opacity: 0;
  font-size: 18px;
  width: 28px;
  height: 28px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}
.workspace-card:hover .card-delete {
  opacity: 0.5;
}
.card-delete:hover {
  opacity: 1 !important;
  color: var(--danger-color);
  background: rgba(192, 57, 43, 0.1);
}

.btn-secondary {
  padding: 8px 16px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 13px;
  color: var(--text-color);
  transition: all 0.2s;
}
.btn-secondary:hover {
  border-color: var(--accent-color);
}
.btn-secondary:disabled {
  opacity: 0.5;
  cursor: default;
}

/* 恢复状?*/
.restore-status {
  padding: 10px 40px;
  font-size: 13px;
  background: color-mix(in srgb, var(--accent-color) 8%, transparent);
  color: var(--accent-color);
  border-bottom: 1px solid var(--border-color);
}
.restore-status.error {
  background: rgba(192, 57, 43, 0.06);
  color: var(--danger-color);
}

/* 对话?*/
.dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.3);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}

.dialog {
  background: var(--surface-color);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  padding: 32px;
  width: 420px;
  max-width: 90vw;
}

.dialog-sm {
  width: 360px;
}

.dialog-title {
  font-size: 18px;
  margin-bottom: 24px;
  font-weight: 600;
}

.form-group {
  margin-bottom: 16px;
}
.form-group label {
  display: block;
  font-size: 13px;
  margin-bottom: 6px;
  color: var(--text-color);
  opacity: 0.7;
}
.form-group input,
.form-group textarea {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--bg-color);
  color: var(--text-color);
  font-size: 14px;
}
.form-group input:focus,
.form-group textarea:focus {
  border-color: var(--accent-color);
}

.type-input-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.type-suggestions {
  display: flex;
  gap: 8px;
}
.type-suggestions button {
  padding: 4px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 12px;
  background: var(--surface-color);
  color: var(--text-color);
  cursor: pointer;
  transition: all 0.2s;
}
.type-suggestions button:hover {
  border-color: var(--accent-color);
  color: var(--accent-color);
}

.structure-options {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-top: 4px;
}
.structure-radio {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  font-size: 13px;
  color: var(--text-color);
}
.structure-radio input[type="radio"] {
  width: auto;
  accent-color: var(--accent-color);
}

.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  margin-top: 24px;
}

.btn-primary {
  padding: 8px 20px;
  background: var(--accent-color);
  color: #fff;
  border-radius: 6px;
  font-size: 14px;
  transition: background 0.2s;
}
.btn-primary:hover {
  background: var(--accent-hover);
}
.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-secondary {
  padding: 8px 20px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 14px;
  color: var(--text-color);
  transition: all 0.2s;
}
.btn-secondary:hover {
  border-color: var(--text-color);
}

.btn-danger {
  padding: 8px 20px;
  background: var(--danger-color);
  color: #fff;
  border-radius: 6px;
  font-size: 14px;
}

.confirm-text {
  font-size: 16px;
  margin-bottom: 8px;
}
.confirm-warn {
  font-size: 13px;
  opacity: 0.6;
  margin-bottom: 8px;
}
</style>