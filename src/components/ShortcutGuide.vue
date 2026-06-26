<script setup lang="ts">

import { ref, onMounted, onUnmounted } from 'vue'

import { useShortcutStore, type ShortcutMap } from '@/stores/shortcut'

const emit = defineEmits<{

  close: []

}>()

const shortcutStore = useShortcutStore()

// State for recording a new shortcut

const recordingKey = ref<keyof ShortcutMap | null>(null)

const currentKeys = ref<string[]>([])

const labels: Record<keyof ShortcutMap, string> = {

  save: '保存文件',

  addAnnotation: '添加灵感批注',

  export: '导出与备份',

  focusMode: '切换专注模式',

  newChapter: '新建章节 (无题)',

}

function startRecording(key: keyof ShortcutMap) {

  recordingKey.value = key

  currentKeys.value = []

}

function handleKeyDown(e: KeyboardEvent) {

  if (!recordingKey.value) return

  

  e.preventDefault()

  e.stopPropagation()

  if (e.key === 'Escape') {

    recordingKey.value = null

    currentKeys.value = []

    return

  }

  const keys: string[] = []

  if (e.ctrlKey || e.metaKey) keys.push('Ctrl')

  if (e.shiftKey) keys.push('Shift')

  if (e.altKey) keys.push('Alt')

  const keyName = e.key

  if (!['Control', 'Shift', 'Alt', 'Meta'].includes(keyName)) {

    // Standardize key name

    let mainKey = keyName.toUpperCase()

    if (mainKey === ' ') mainKey = 'Space'

    keys.push(mainKey)

    

    // Save the shortcut

    const newShortcut = keys.join('+')

    shortcutStore.updateShortcut(recordingKey.value, newShortcut)

    recordingKey.value = null

    currentKeys.value = []

  } else {

    currentKeys.value = keys

  }

}

function handleKeyUp(e: KeyboardEvent) {

  if (!recordingKey.value) return

  e.preventDefault()

  e.stopPropagation()

  // If they released a modifier without pressing a key, just reset

  if (currentKeys.value.length > 0 && !currentKeys.value.some(k => !['Ctrl', 'Shift', 'Alt'].includes(k))) {

    currentKeys.value = []

  }

}

onMounted(() => {

  window.addEventListener('keydown', handleKeyDown, true)

  window.addEventListener('keyup', handleKeyUp, true)

})

onUnmounted(() => {

  window.removeEventListener('keydown', handleKeyDown, true)

  window.removeEventListener('keyup', handleKeyUp, true)

})

</script>

<template>

  <div class="dialog-overlay" @click.self="emit('close')">

    <div class="dialog">

      <div class="dialog-header">

        <h2 class="dialog-title">快捷键设</h2>

        <button class="close-btn" @click="emit('close')"></button>

      </div>

      <div class="shortcut-list">

        <div 

          v-for="(value, key) in shortcutStore.shortcuts" 

          :key="key" 

          class="shortcut-item"

        >

          <span class="shortcut-label">{{ labels[key] }}</span>

          

          <button 

            class="shortcut-btn" 

            :class="{ recording: recordingKey === key }"

            @click="startRecording(key)"

            :title="recordingKey === key ? '按Esc 取消' : '点击修改快捷键'"

          >

            <template v-if="recordingKey === key">

              {{ currentKeys.length ? currentKeys.join('+') : '请按下快捷键…' }}

            </template>

            <template v-else>

              {{ value }}

            </template>

          </button>

        </div>

      </div>

      <div class="dialog-actions">

        <button class="btn-secondary" @click="shortcutStore.resetShortcuts()">恢复默认</button>

        <button class="btn-primary" @click="emit('close')">完成</button>

      </div>

    </div>

  </div>

</template>

<style scoped>

.dialog-overlay {

  position: fixed;

  inset: 0;

  background: rgba(0, 0, 0, 0.4);

  display: flex;

  align-items: center;

  justify-content: center;

  z-index: 9999;

  backdrop-filter: blur(2px);

}

.dialog {

  background: var(--surface-color);

  border: 1px solid var(--border-color);

  border-radius: 12px;

  padding: 32px;

  width: 420px;

  max-width: 90vw;

  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.1);

}

.dialog-header {

  display: flex;

  justify-content: space-between;

  align-items: center;

  margin-bottom: 24px;

}

.dialog-title {

  font-size: 18px;

  font-weight: 600;

  color: var(--text-color);

  margin: 0;

}

.close-btn {

  background: transparent;

  border: none;

  font-size: 16px;

  color: var(--text-color);

  opacity: 0.5;

  cursor: pointer;

  padding: 4px;

}

.close-btn:hover {

  opacity: 1;

}

.shortcut-list {

  display: flex;

  flex-direction: column;

  gap: 12px;

  margin-bottom: 32px;

}

.shortcut-item {

  display: flex;

  justify-content: space-between;

  align-items: center;

  padding: 8px 0;

  border-bottom: 1px solid var(--border-color);

}

.shortcut-label {

  font-size: 14px;

  color: var(--text-color);

  opacity: 0.8;

}

.shortcut-btn {

  background: var(--bg-color);

  border: 1px solid var(--border-color);

  border-radius: 6px;

  padding: 6px 12px;

  font-size: 13px;

  font-family: "Cascadia Code", "Fira Code", "Consolas", monospace;

  color: var(--text-color);

  cursor: pointer;

  min-width: 120px;

  text-align: center;

  transition: all 0.2s;

}

.shortcut-btn:hover:not(.recording) {

  border-color: var(--accent-color);

  color: var(--accent-color);

}

.shortcut-btn.recording {

  background: var(--accent-color);

  color: #fff;

  border-color: var(--accent-color);

  animation: pulse 1.5s infinite;

}

@keyframes pulse {

  0% { box-shadow: 0 0 0 0 color-mix(in srgb, var(--accent-color) 40%, transparent); }

  70% { box-shadow: 0 0 0 6px transparent; }

  100% { box-shadow: 0 0 0 0 transparent; }

}

.dialog-actions {

  display: flex;

  justify-content: space-between;

  align-items: center;

}

.btn-secondary {

  padding: 8px 16px;

  border: 1px solid var(--border-color);

  border-radius: 6px;

  font-size: 13px;

  color: var(--text-color);

  opacity: 0.7;

  transition: all 0.2s;

}

.btn-secondary:hover {

  opacity: 1;

  border-color: var(--text-color);

}

.btn-primary {

  padding: 8px 24px;

  background: var(--accent-color);

  color: #fff;

  border-radius: 6px;

  font-size: 14px;

  transition: background 0.2s;

}

.btn-primary:hover {

  background: var(--accent-hover);

}

</style>

