<script setup lang="ts">
import { ref, computed } from 'vue'
import draggable from 'vuedraggable'
import { useWorkspaceStore } from '@/stores/workspace'
import { useEditorStore } from '@/stores/editor'
import type { ChapterEntry } from '@/types'

const emit = defineEmits<{
  toggle: []
  /** 切换到时间线视图（随笔模式） */
  'switchToTimeline': []
}>()

const workspace = useWorkspaceStore()
const editor = useEditorStore()

// 新建章节对话框
const showAddChapter = ref(false)
const showAddVolume = ref(false)
const newEntryName = ref('')
const targetVolumeId = ref('')

// 重命名ID
const renamingId = ref<string | null>(null)
const renameValue = ref('')
const renamingType = ref<'chapter' | 'volume'>('chapter')

// 折叠的卷（使用对象以确保 Vue 3 响应式追踪）
const collapsedVolumes = ref<Record<string, boolean>>({})

function toggleVolume(volId: string) {
  collapsedVolumes.value = { ...collapsedVolumes.value, [volId]: !collapsedVolumes.value[volId] }
}

function isVolumeCollapsed(volId: string): boolean {
  return !!collapsedVolumes.value[volId]
}

async function selectChapter(ch: ChapterEntry) {
  workspace.selectChapter(ch)
  const path = workspace.getChapterPath(ch)
  await editor.loadFile(path)
  editor.startTimer()
}

async function handleAddChapter() {
  if (!newEntryName.value.trim()) return
  let finalName = newEntryName.value.trim()
  if (workspace.bookIndex?.auto_numbering) {
    const volId = targetVolumeId.value
    let chapterIndex = 1
    if (volId) {
      const vol = workspace.bookIndex.volumes?.find((v) => v.id === volId)
      if (vol) chapterIndex = vol.chapters.length + 1
    } else {
      const vols = workspace.bookIndex.volumes || []
      const rootChapters = workspace.bookIndex.chapters?.length || 0
      // 如果没有卷，就用根章节数
      if (vols.length === 0) {
        chapterIndex = rootChapters + 1
      } else {
        // 如果有卷但没指定，默认加到第一
        chapterIndex = vols[0].chapters.length + 1
      }
    }
    finalName = `第${chapterIndex}章 ${finalName}`
  }
  await workspace.createChapter(
    finalName,
    targetVolumeId.value || undefined
  )
  showAddChapter.value = false
  newEntryName.value = ''
}

async function handleAddVolume() {
  if (!newEntryName.value.trim()) return
  let finalName = newEntryName.value.trim()
  if (workspace.bookIndex?.auto_numbering) {
    const volumesLength = workspace.bookIndex?.volumes?.length || 0
    finalName = `第${volumesLength + 1}卷 ${finalName}`
  }
  await workspace.createVolume(finalName)
  showAddVolume.value = false
  newEntryName.value = ''
}

async function handleRename(oldPath: string) {
  if (!renameValue.value.trim()) return
  await workspace.renameEntry(oldPath, renameValue.value.trim())
  renamingId.value = null
  renameValue.value = ''
}

async function handleDeleteChapter(id: string) {
  await workspace.deleteChapter(id)
}

async function handleDeleteVolume(id: string) {
  await workspace.deleteVolume(id)
}

function startRename(id: string, currentName: string, type: 'chapter' | 'volume' = 'chapter') {
  renamingId.value = id
  renameValue.value = currentName
  renamingType.value = type
}

// ========== 拖拽排序 ==========

/** 章节拖拽排序完成 */
async function onChapterDragEnd(volId: string) {
  if (!workspace.bookIndex?.volumes) return
  const vol = workspace.bookIndex.volumes.find((v) => v.id === volId)
  if (!vol) return
  // 更新章节 order
  const updatedChapters = vol.chapters.map((ch, i) => ({ ...ch, order: i }))
  vol.chapters = updatedChapters
  await workspace.reorder(workspace.bookIndex.volumes, undefined)
}

/** 分卷拖拽排序完成 */
async function onVolumeDragEnd() {
  if (!workspace.bookIndex?.volumes) return
  const updatedVolumes = workspace.bookIndex.volumes.map((vol, i) => {
    const updatedChapters = vol.chapters.map((ch, j) => ({ ...ch, order: j }))
    return { ...vol, order: i, chapters: updatedChapters }
  })
  await workspace.reorder(updatedVolumes, undefined)
}

/** 随笔章节拖拽排序完成 */
async function onEssayDragEnd() {
  if (!workspace.bookIndex?.chapters) return
  const updatedChapters = workspace.bookIndex.chapters.map((ch, i) => ({ ...ch, order: i }))
  await workspace.reorder(undefined, updatedChapters)
}

const isVolumeMode = computed(() => !!workspace.bookIndex?.volumes)
const isFlatMode = computed(() => !isVolumeMode.value)

const workTypeDisplay = computed(() => {
  const type = workspace.bookIndex?.type
  if (!type || type === 'novel') return '章节'
  if (type === 'essay') return '随笔'
  return type
})
</script>

<template>
  <aside class="sidebar">
    <div class="sidebar-header">
      <span class="sidebar-title">{{ workspace.currentWorkspace?.name }}</span>
      <button class="icon-btn" @click="emit('toggle')" title="收起侧边栏">✕</button>
    </div>

    <div class="sidebar-body">
      <!-- 分卷模式：分卷列表（拖拽排序） -->
      <template v-if="isVolumeMode && workspace.bookIndex?.volumes">
        <draggable
          :list="workspace.bookIndex.volumes"
          item-key="id"
          handle=".drag-handle"
          ghost-class="drag-ghost"
          :animation="200"
          @change="onVolumeDragEnd"
        >
          <template #item="{ element: vol }">
            <div class="volume-group">
              <div class="volume-header">
                <span class="drag-handle" title="拖拽排序">⋮⋮</span>
                <span class="volume-arrow" @click="toggleVolume(vol.id)">
                  {{ isVolumeCollapsed(vol.id) ? '▶' : '▼' }}
                </span>
                <span
                  v-if="renamingId !== vol.id || renamingType !== 'volume'"
                  class="volume-title"
                  @click="toggleVolume(vol.id)"
                >
                  {{ vol.title }}
                </span>
                <input
                  v-else
                  v-model="renameValue"
                  class="rename-input"
                  @keyup.enter="handleRename(vol.id)"
                  @keyup.escape="renamingId = null"
                  @click.stop
                  @blur="handleRename(vol.id)"
                  autofocus
                />
                <span class="chapter-count">{{ vol.chapters.length }} </span>
                <div class="volume-actions">
                  <button
                    class="mini-btn"
                    @click.stop="targetVolumeId = vol.id; showAddChapter = true"
                    title="在此卷添加章节"

                  >+</button>
                  <button
                    class="mini-btn"
                    @click.stop="startRename(vol.id, vol.title, 'volume')"
                    title="重命名"

                  >✎</button>
                  <button
                    class="mini-btn"
                    @click.stop="handleDeleteVolume(vol.id)"
                    title="删除"

                  >×</button>
                </div>
              </div>

              <!-- 卷内章节列表（拖拽排序） -->
              <div v-if="!isVolumeCollapsed(vol.id)" class="chapter-list">
                <draggable
                  :list="vol.chapters"
                  item-key="id"
                  handle=".chapter-drag-handle"
                  ghost-class="drag-ghost"
                  :animation="200"
                  @change="onChapterDragEnd(vol.id)"
                >
                  <template #item="{ element: ch }">
                    <div
                      class="chapter-item"
                      :class="{ active: workspace.selectedChapter?.id === ch.id }"
                    >
                      <span class="chapter-drag-handle" title="拖拽排序">⋮</span>
                      <span
                        v-if="renamingId !== ch.id || renamingType !== 'chapter'"
                        class="chapter-name"
                        @click="selectChapter(ch)"
                      >
                        {{ ch.title }}
                      </span>
                      <input
                        v-else
                        v-model="renameValue"
                        class="rename-input"
                        @keyup.enter="handleRename(ch.file)"
                        @keyup.escape="renamingId = null"
                        @click.stop
                        @blur="handleRename(ch.file)"
                        autofocus
                      />
                      <div class="chapter-actions">
                        <button
                          class="mini-btn"
                          @click.stop="startRename(ch.id, ch.title, 'chapter')"
                          title="重命名"

                        ></button>
                        <button
                          class="mini-btn"
                          @click.stop="handleDeleteChapter(ch.id)"
                          title="删除"
                        >×</button>
                      </div>
                    </div>
                  </template>
                </draggable>
              </div>
            </div>
          </template>
        </draggable>
      </template>

      <!-- 平铺模式：根级章节/随笔列表（拖拽排序） -->
      <template v-if="isFlatMode && workspace.bookIndex?.chapters">
        <draggable
          :list="workspace.bookIndex.chapters"
          item-key="id"
          handle=".chapter-drag-handle"
          ghost-class="drag-ghost"
          :animation="200"
          @change="onEssayDragEnd"
        >
          <template #item="{ element: ch }">
            <div
              class="chapter-item"
              :class="{ active: workspace.selectedChapter?.id === ch.id }"
            >
              <span class="chapter-drag-handle" title="拖拽排序">⋮</span>
              <span
                v-if="renamingId !== ch.id || renamingType !== 'chapter'"
                class="chapter-name"
                @click="selectChapter(ch)"
              >
                {{ ch.title }}
              </span>
              <input
                v-else
                v-model="renameValue"
                class="rename-input"
                @keyup.enter="handleRename(ch.file)"
                @keyup.escape="renamingId = null"
                @click.stop
                @blur="handleRename(ch.file)"
                autofocus
              />
              <span class="chapter-meta" v-if="ch.modified_at">
                {{ new Date(ch.modified_at).toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' }) }}
              </span>
              <div class="chapter-actions">
                <button
                  class="mini-btn"
                  @click.stop="startRename(ch.id, ch.title, 'chapter')"
                  title="重命名"

                ></button>
                <button
                  class="mini-btn"
                  @click.stop="handleDeleteChapter(ch.id)"
                  title="删除"
                >×</button>
              </div>
            </div>
          </template>
        </draggable>
      </template>

      <!-- 底部新建按钮 -->
      <div class="sidebar-footer">
        <button v-if="isVolumeMode" class="add-volume-btn" @click="showAddVolume = true">
          + 新建分卷
        </button>
        <button v-if="isFlatMode" class="add-volume-btn" @click="targetVolumeId = ''; showAddChapter = true">
          + 新建{{ workTypeDisplay }}
        </button>
        <button
          class="timeline-btn"
          @click="emit('switchToTimeline')"
          title="时间线视图"
        >
          📅 时间线
        </button>
      </div>
    </div>

    <!-- 新建章节/随笔对话框 -->
    <Teleport to="body">
      <div v-if="showAddChapter" class="mini-dialog-overlay" @click.self="showAddChapter = false">
        <div class="mini-dialog">
          <input
            v-model="newEntryName"
            :placeholder="targetVolumeId ? '章节名称' : `${workTypeDisplay}名称`"
            @keyup.enter="handleAddChapter"
            autofocus
          />
          <div class="mini-dialog-actions">
            <button @click="showAddChapter = false">取消</button>
            <button @click="handleAddChapter" :disabled="!newEntryName.trim()">确定</button>
          </div>
        </div>
      </div>

      <div v-if="showAddVolume" class="mini-dialog-overlay" @click.self="showAddVolume = false">
        <div class="mini-dialog">
          <input
            v-model="newEntryName"
            placeholder="分卷名称"
            @keyup.enter="handleAddVolume"
            autofocus
          />
          <div class="mini-dialog-actions">
            <button @click="showAddVolume = false">取消</button>
            <button @click="handleAddVolume" :disabled="!newEntryName.trim()">确定</button>
          </div>
        </div>
      </div>
    </Teleport>
  </aside>
</template>

<style scoped>
.sidebar {
  width: var(--sidebar-width);
  height: 100%;
  border-right: 1px solid var(--border-color);
  background: var(--surface-color);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
}

.sidebar-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 12px;
  height: 44px;
  border-bottom: 1px solid var(--border-color);
}

.sidebar-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-color);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.sidebar-body {
  flex: 1;
  overflow-y: auto;
  padding: 8px 0;
}

/* 拖拽手柄 */
.drag-handle {
  display: flex;
  align-items: center;
  cursor: grab;
  color: var(--text-color);
  opacity: 0.25;
  font-size: 12px;
  letter-spacing: 2px;
  padding: 0 2px;
  transition: opacity 0.15s;
}
.drag-handle:active {
  cursor: grabbing;
  opacity: 0.6;
}

.chapter-drag-handle {
  composes: drag-handle;
  display: flex;
  align-items: center;
  cursor: grab;
  color: var(--text-color);
  opacity: 0;
  font-size: 10px;
  padding: 0 2px;
  transition: opacity 0.15s;
  flex-shrink: 0;
}
.chapter-item:hover .chapter-drag-handle {
  opacity: 0.3;
}
.chapter-drag-handle:active {
  cursor: grabbing;
  opacity: 0.6 !important;
}

/* 拖拽幽灵 */
:deep(.drag-ghost) {
  opacity: 0.4;
  background: var(--accent-color);
}

/* 分卷 */
.volume-group {
  margin-bottom: 4px;
}

.volume-header {
  display: flex;
  align-items: center;
  padding: 8px 12px 8px 8px;
  gap: 4px;
  font-size: 13px;
  font-weight: 500;
  color: var(--text-color);
  opacity: 0.8;
  transition: background 0.15s;
}
.volume-header:hover {
  background: var(--bg-color);
}
.volume-header:hover .drag-handle {
  opacity: 0.5;
}

.volume-arrow {
  font-size: 10px;
  width: 14px;
  flex-shrink: 0;
  cursor: pointer;
}

.volume-title {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  cursor: pointer;
}

.chapter-count {
  font-size: 11px;
  opacity: 0.4;
  margin-right: 4px;
  flex-shrink: 0;
}

.volume-actions {
  display: flex;
  gap: 2px;
  opacity: 0;
  transition: opacity 0.15s;
}
.volume-header:hover .volume-actions {
  opacity: 1;
}

/* 章节 */
.chapter-list {
  padding: 0;
}

.chapter-item {
  display: flex;
  align-items: center;
  padding: 6px 12px 6px 24px;
  cursor: pointer;
  font-size: 13px;
  color: var(--text-color);
  opacity: 0.7;
  transition: all 0.15s;
  gap: 4px;
}
.chapter-item:hover {
  background: var(--bg-color);
  opacity: 0.9;
}
.chapter-item.active {
  opacity: 1;
  color: var(--accent-color);
  background: color-mix(in srgb, var(--accent-color) 6%, transparent);
}

.chapter-name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.chapter-meta {
  font-size: 10px;
  opacity: 0.35;
  flex-shrink: 0;
  margin-right: 4px;
}

.chapter-actions {
  display: flex;
  gap: 2px;
  opacity: 0;
  transition: opacity 0.15s;
}
.chapter-item:hover .chapter-actions {
  opacity: 1;
}

.mini-btn {
  width: 22px;
  height: 22px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  border-radius: 4px;
  transition: all 0.15s;
  color: var(--text-color);
  flex-shrink: 0;
}
.mini-btn:hover {
  background: var(--border-color);
}

.rename-input {
  flex: 1;
  padding: 2px 4px;
  font-size: 13px;
  border: 1px solid var(--accent-color);
  border-radius: 3px;
  background: var(--bg-color);
  color: var(--text-color);
  min-width: 0;
}

.add-volume-btn {
  width: 100%;
  padding: 10px;
  font-size: 13px;
  color: var(--accent-color);
  opacity: 0.7;
  transition: all 0.15s;
  text-align: center;
}
.add-volume-btn:hover {
  opacity: 1;
  background: var(--bg-color);
}

.sidebar-footer {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 8px 12px;
  border-top: 1px solid var(--border-color);
}

.timeline-btn {
  width: 100%;
  padding: 8px;
  font-size: 13px;
  color: var(--text-color);
  opacity: 0.5;
  transition: all 0.15s;
  text-align: center;
}
.timeline-btn:hover {
  opacity: 0.8;
  background: var(--bg-color);
}

.icon-btn {
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
.icon-btn:hover {
  background: var(--border-color);
}

/* 迷你对话?*/
.mini-dialog-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.2);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 200;
}
.mini-dialog {
  background: var(--surface-color);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 16px;
  width: 300px;
}
.mini-dialog input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--bg-color);
  color: var(--text-color);
  font-size: 14px;
  margin-bottom: 12px;
  outline: none;
}
.mini-dialog input:focus {
  border-color: var(--accent-color);
}
.mini-dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}
.mini-dialog-actions button {
  padding: 6px 16px;
  border-radius: 4px;
  font-size: 13px;
}
.mini-dialog-actions button:first-child {
  border: 1px solid var(--border-color);
  color: var(--text-color);
}
.mini-dialog-actions button:last-child {
  background: var(--accent-color);
  color: #fff;
}
.mini-dialog-actions button:last-child:disabled {
  opacity: 0.5;
}
</style>