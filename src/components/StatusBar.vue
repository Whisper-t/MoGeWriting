<script setup lang="ts">
import { useEditorStore } from '@/stores/editor'
import { useWorkspaceStore } from '@/stores/workspace'

const editor = useEditorStore()
const workspace = useWorkspaceStore()
</script>

<template>
  <footer class="status-bar">
    <div class="status-left">
      <span v-if="workspace.selectedChapter?.title" class="status-item">
        {{ workspace.selectedChapter.title }}
      </span>
      <span v-if="workspace.selectedChapter?.title" class="status-separator">·</span>
      <span class="status-item">{{ editor.wordCount.toLocaleString() }} 字</span>
    </div>
    <div class="status-right">
      <span class="status-item">写作 {{ editor.formattedDuration }}</span>
      <span class="status-separator">·</span>
      <span class="status-item" :class="{ dirty: editor.isDirty }">
        {{ editor.isDirty ? '未保存' : '已保存' }}
      </span>
    </div>
  </footer>
</template>

<style scoped>
.status-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px;
  height: 28px;
  border-top: 1px solid var(--border-color);
  background: var(--surface-color);
  flex-shrink: 0;
  user-select: none;
}

.status-left,
.status-right {
  display: flex;
  align-items: center;
  gap: 6px;
}

.status-item {
  font-size: 11px;
  color: var(--text-color);
  opacity: 0.5;
}

.status-separator {
  font-size: 11px;
  color: var(--text-color);
  opacity: 0.25;
}

.status-item.dirty {
  color: var(--accent-color);
  opacity: 0.7;
}
</style>