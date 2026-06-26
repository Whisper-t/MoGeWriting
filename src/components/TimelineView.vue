<script setup lang="ts">
import { computed, ref } from 'vue'
import { useWorkspaceStore } from '@/stores/workspace'
import { useEditorStore } from '@/stores/editor'
import type { ChapterEntry } from '@/types'

const emit = defineEmits<{
  back: []
  /** 选择一篇随?*/
  'selectChapter': [chapter: ChapterEntry]
}>()

const workspace = useWorkspaceStore()
const editor = useEditorStore()

// 排序方向
const sortAsc = ref(false)

/** 按时间排序的随笔列表 */
const sortedChapters = computed(() => {
  const chapters = workspace.bookIndex?.chapters ?? []
  const sorted = [...chapters].sort((a, b) => {
    const dateA = new Date(a.created_at || a.modified_at || 0).getTime()
    const dateB = new Date(b.created_at || b.modified_at || 0).getTime()
    return sortAsc.value ? dateA - dateB : dateB - dateA
  })
  return sorted
})

/** 按年份分?*/
const groupedByYear = computed(() => {
  const groups: { year: string; items: ChapterEntry[] }[] = []
  for (const ch of sortedChapters.value) {
    const date = new Date(ch.created_at || ch.modified_at || 0)
    const year = date.getFullYear().toString()
    let group = groups.find((g) => g.year === year)
    if (!group) {
      group = { year, items: [] }
      groups.push(group)
    }
    group.items.push(ch)
  }
  return groups
})

function formatDate(dateStr?: string): string {
  if (!dateStr) return '未知'
  const d = new Date(dateStr)
  return d.toLocaleDateString('zh-CN', {
    month: 'long',
    day: 'numeric',
    weekday: 'short',
  })
}

function formatTime(dateStr?: string): string {
  if (!dateStr) return ''
  const d = new Date(dateStr)
  return d.toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit',
  })
}

async function handleSelect(ch: ChapterEntry) {
  workspace.selectChapter(ch)
  const path = workspace.getChapterPath(ch)
  await editor.loadFile(path)
  editor.startTimer()
  emit('selectChapter', ch)
}
</script>

<template>
  <aside class="timeline">
    <div class="timeline-header">
      <button class="icon-btn" @click="$emit('back')" title="返回目录"></button>
      <span class="timeline-title">时间</span>
      <button
        class="sort-btn"
        @click="sortAsc = !sortAsc"
        :title="sortAsc ? '降序' : '升序'"
      >
        {{ sortAsc ? '↓' : '↑' }}
      </button>
    </div>

    <div class="timeline-body">
      <div v-if="sortedChapters.length === 0" class="empty-state">
        <p>还没有随笔</p>
      </div>

      <template v-for="group in groupedByYear" :key="group.year">
        <!-- 年份标记 -->
        <div class="year-marker">
          <span class="year-dot"></span>
          <span class="year-label">{{ group.year }}</span>
          <span class="year-count">{{ group.items.length }} </span>
        </div>

        <!-- 该年份的随笔 -->
        <div
          v-for="item in group.items"
          :key="item.id"
          class="timeline-item"
          :class="{ active: workspace.selectedChapter?.id === item.id }"
          @click="handleSelect(item)"
        >
          <div class="timeline-line">
            <div class="timeline-dot"></div>
          </div>
          <div class="timeline-card">
            <div class="card-header">
              <span class="card-title">{{ item.title }}</span>
              <span class="card-time">{{ formatTime(item.created_at) }}</span>
            </div>
            <div class="card-date">{{ formatDate(item.created_at) }}</div>
            <div v-if="item.word_count" class="card-meta">
              {{ item.word_count.toLocaleString() }} ?            </div>
          </div>
        </div>
      </template>
    </div>
  </aside>
</template>

<style scoped>
.timeline {
  width: var(--sidebar-width);
  height: 100%;
  border-right: 1px solid var(--border-color);
  background: var(--surface-color);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  user-select: none;
}

.timeline-header {
  display: flex;
  align-items: center;
  padding: 0 8px;
  height: 44px;
  border-bottom: 1px solid var(--border-color);
  gap: 8px;
}

.timeline-title {
  flex: 1;
  font-size: 14px;
  font-weight: 600;
  color: var(--text-color);
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

.sort-btn {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  font-size: 14px;
  color: var(--text-color);
  opacity: 0.5;
  transition: all 0.15s;
}
.sort-btn:hover {
  opacity: 1;
  background: var(--border-color);
}

.timeline-body {
  flex: 1;
  overflow-y: auto;
  padding: 16px 0;
}

.empty-state {
  text-align: center;
  padding: 40px 16px;
  font-size: 13px;
  color: var(--text-color);
  opacity: 0.4;
}

/* 年份标记 */
.year-marker {
  display: flex;
  align-items: center;
  padding: 8px 16px;
  gap: 8px;
}

.year-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--accent-color);
  flex-shrink: 0;
}

.year-label {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-color);
}

.year-count {
  font-size: 11px;
  color: var(--text-color);
  opacity: 0.4;
}

/* 时间线条?*/
.timeline-item {
  display: flex;
  padding: 0 16px;
  cursor: pointer;
  position: relative;
}

.timeline-line {
  width: 16px;
  display: flex;
  flex-direction: column;
  align-items: center;
  flex-shrink: 0;
  position: relative;
}

.timeline-line::before {
  content: '';
  position: absolute;
  top: 0;
  bottom: 0;
  left: 50%;
  width: 1px;
  background: var(--border-color);
  transform: translateX(-50%);
}

.timeline-item:last-child .timeline-line::before {
  bottom: 50%;
}

.timeline-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--border-color);
  margin-top: 14px;
  position: relative;
  z-index: 1;
  transition: all 0.15s;
}

.timeline-item.active .timeline-dot {
  background: var(--accent-color);
  transform: scale(1.5);
}

.timeline-card {
  flex: 1;
  padding: 8px 12px;
  margin: 2px 0;
  border-radius: 6px;
  transition: background 0.15s;
  min-width: 0;
}

.timeline-item:hover .timeline-card {
  background: var(--bg-color);
}

.timeline-item.active .timeline-card {
  background: color-mix(in srgb, var(--accent-color) 6%, transparent);
}

.card-header {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: 8px;
}

.card-title {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-color);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.card-time {
  font-size: 10px;
  color: var(--text-color);
  opacity: 0.4;
  flex-shrink: 0;
}

.card-date {
  font-size: 11px;
  color: var(--text-color);
  opacity: 0.5;
  margin-top: 2px;
}

.card-meta {
  font-size: 10px;
  color: var(--text-color);
  opacity: 0.35;
  margin-top: 4px;
}
</style>