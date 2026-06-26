
<script setup lang="ts">
import { ref, computed } from 'vue'
import { useAnnotationStore } from '@/stores/annotation'
import { useShortcutStore } from '@/stores/shortcut'
import type { AnnotationData } from '@/stores/annotation'

const emit = defineEmits<{
  close: []
  /** 点击批注卡片，滚动到对应文字 */
  'clickAnnotation': [annotationId: string]
  /** 悬停批注卡片 */
  'hoverAnnotation': [annotationId: string | null]
  /** 添加新批注 */
  'addAnnotation': []
}>()

const annotation = useAnnotationStore()
const shortcutStore = useShortcutStore()

// 编辑状态
const editingId = ref<string | null>(null)
const editContent = ref('')

function startEdit(item: AnnotationData) {
  editingId.value = item.id
  editContent.value = item.content
}

function saveEdit() {
  if (editingId.value) {
    annotation.updateAnnotation(editingId.value, editContent.value)
    editingId.value = null
    editContent.value = ''
  }
}

function cancelEdit() {
  editingId.value = null
  editContent.value = ''
}

function handleDelete(id: string) {
  annotation.removeAnnotation(id)
}

function handleClick(item: AnnotationData) {
  emit('clickAnnotation', item.id)
}

function handleMouseEnter(item: AnnotationData) {
  emit('hoverAnnotation', item.id)
}

function handleMouseLeave() {
  emit('hoverAnnotation', null)
}

function formatDate(dateStr: string): string {
  const d = new Date(dateStr)
  return d.toLocaleString('zh-CN', {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

const hasAnnotations = computed(() => annotation.annotationList.length > 0)
</script>

<template>
  <aside class="inspector">
    <div class="inspector-header">
      <span class="inspector-title">
        灵感笔记
        <span v-if="hasAnnotations" class="count-badge">
          {{ annotation.annotationList.length }}
        </span>
      </span>
      <button class="icon-btn" @click="$emit('close')" title="关闭面板">×</button>
    </div>

    <div class="inspector-body">
      <!-- 空状态 -->
      <div v-if="!hasAnnotations" class="empty-state">
        <p class="empty-icon">◇</p>
        <p class="empty-text">还没有灵感笔记</p>
        <p class="empty-hint">
          在编辑器中选中文字，然后点击下方按钮或
          <kbd>{{ shortcutStore.shortcuts.addAnnotation }}</kbd>
        </p>
      </div>

      <!-- 批注卡片列表 -->
      <div
        v-for="item in annotation.annotationList"
        :key="item.id"
        class="annotation-card"
        :class="{
          editing: editingId === item.id,
          hovered: annotation.hoveredId === item.id,
        }"
        @mouseenter="handleMouseEnter(item)"
        @mouseleave="handleMouseLeave"
      >
        <!-- 引文预览 -->
        <div v-if="item.quotedText" class="card-quote" @click="handleClick(item)">
          「{{ item.quotedText }}」
        </div>

        <!-- 编辑模式 -->
        <div v-if="editingId === item.id" class="card-edit">
          <textarea
            v-model="editContent"
            placeholder="写下你的灵感…"
            rows="4"
            autofocus
            @keyup.escape="cancelEdit"
          />
          <div class="card-edit-actions">
            <button class="btn-cancel" @click="cancelEdit">取消</button>
            <button class="btn-save" @click="saveEdit">保存</button>
          </div>
        </div>

        <!-- 查看模式 -->
        <div v-else class="card-view" @click="startEdit(item)">
          <p v-if="item.content" class="card-content">{{ item.content }}</p>
          <p v-else class="card-content placeholder">点击添加灵感内容</p>
          <div class="card-footer">
            <span class="card-date">{{ formatDate(item.modifiedAt) }}</span>
            <div class="card-actions">
              <button
                class="mini-btn locate-btn"
                @click.stop="handleClick(item)"
                title="定位到原文"
              >
                📍
              </button>
              <button
                class="mini-btn"
                @click.stop="handleDelete(item.id)"
                title="删除此批注"
              >
                删除
              </button>
            </div>
          </div>
        </div>
        </div>
      </div>

    <!-- 底部操作?-->
    <div class="inspector-footer">
      <button class="add-btn" @click="$emit('addAnnotation')">
        + 选中文字添加批注
      </button>
      <span class="shortcut-hint">{{ shortcutStore.shortcuts.addAnnotation }}</span>
    </div>
  </aside>
</template>

<style scoped>
.inspector {
  width: var(--inspector-width);
  height: 100%;
  border-left: 1px solid var(--border-color);
  background: var(--surface-color);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  user-select: none;
}

.inspector-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 12px;
  height: 44px;
  border-bottom: 1px solid var(--border-color);
}

.inspector-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-color);
  display: flex;
  align-items: center;
  gap: 8px;
}

.count-badge {
  font-size: 11px;
  background: var(--accent-color);
  color: #fff;
  padding: 1px 6px;
  border-radius: 10px;
  font-weight: 500;
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

.inspector-body {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
}

.empty-state {
  text-align: center;
  padding: 48px 16px;
  color: var(--text-color);
}

.empty-icon {
  font-size: 36px;
  margin-bottom: 12px;
}

.empty-text {
  font-size: 14px;
  margin-bottom: 8px;
  opacity: 0.6;
}

.empty-hint {
  font-size: 12px;
  opacity: 0.4;
  line-height: 1.6;
}

.empty-hint kbd {
  display: inline-block;
  padding: 1px 6px;
  border: 1px solid var(--border-color);
  border-radius: 3px;
  font-family: inherit;
  font-size: 11px;
  background: var(--bg-color);
}

/* 批注卡片 */
.annotation-card {
  margin-bottom: 10px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  overflow: hidden;
  transition: all 0.15s;
}

.annotation-card.hovered {
  border-color: var(--accent-color);
  box-shadow: 0 0 0 1px color-mix(in srgb, var(--accent-color) 15%, transparent);
}

.card-quote {
  padding: 8px 12px;
  font-size: 12px;
  color: var(--accent-color);
  background: color-mix(in srgb, var(--accent-color) 4%, transparent);
  border-bottom: 1px solid var(--border-color);
  cursor: pointer;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  transition: background 0.15s;
}

.card-quote:hover {
  background: color-mix(in srgb, var(--accent-color) 10%, transparent);
}

.card-view {
  padding: 10px 12px;
  cursor: pointer;
  transition: background 0.15s;
}

.card-view:hover {
  background: var(--bg-color);
}

.card-content {
  font-size: 13px;
  color: var(--text-color);
  opacity: 0.8;
  white-space: pre-wrap;
  line-height: 1.6;
}

.card-content.placeholder {
  opacity: 0.35;
  font-style: italic;
}

.card-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-top: 8px;
}

.card-date {
  font-size: 10px;
  opacity: 0.35;
}

.mini-btn {
  width: 22px;
  height: 22px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  border-radius: 4px;
  opacity: 0;
  transition: all 0.15s;
  color: var(--text-color);
}
.card-view:hover .mini-btn {
  opacity: 0.4;
}
.mini-btn:hover {
  opacity: 1 !important;
  background: var(--border-color);
  color: var(--danger-color);
}
.locate-btn:hover {
  color: var(--accent-color) !important;
}

.card-actions {
  display: flex;
  align-items: center;
  gap: 4px;
}

/* 编辑模式 */
.card-edit {
  padding: 10px 12px;
}

.card-edit textarea {
  width: 100%;
  padding: 8px;
  border: 1px solid var(--accent-color);
  border-radius: 4px;
  background: var(--bg-color);
  color: var(--text-color);
  font-size: 13px;
  resize: vertical;
  font-family: inherit;
  line-height: 1.6;
  outline: none;
}

.card-edit-actions {
  display: flex;
  justify-content: flex-end;
  gap: 6px;
  margin-top: 8px;
}

.card-edit-actions button {
  padding: 4px 12px;
  border-radius: 4px;
  font-size: 12px;
  transition: all 0.15s;
}

.btn-cancel {
  border: 1px solid var(--border-color);
  color: var(--text-color);
}
.btn-cancel:hover {
  border-color: var(--text-color);
}

.btn-save {
  background: var(--accent-color);
  color: #fff;
}
.btn-save:hover {
  background: var(--accent-hover);
}

/* 底部 */
.inspector-footer {
  padding: 12px;
  border-top: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.add-btn {
  width: 100%;
  padding: 8px;
  font-size: 13px;
  color: var(--accent-color);
  border: 1px dashed var(--border-color);
  border-radius: 6px;
  transition: all 0.15s;
}
.add-btn:hover {
  border-color: var(--accent-color);
  background: color-mix(in srgb, var(--accent-color) 4%, transparent);
}

.shortcut-hint {
  font-size: 10px;
  color: var(--text-color);
  opacity: 0.3;
  text-align: center;
}
</style>