import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import * as api from '@/composables/useTauriCommands'
import { countMarkdownWords } from '@/composables/useMarkdown'
import type { HistorySnapshot } from '@/types'

export const useEditorStore = defineStore('editor', () => {
  /** 当前编辑的内容（Markdown 格式） */
  const content = ref('')
  /** 当前文件路径 */
  const currentFilePath = ref('')
  /** 是否已修改（未保存） */
  const isDirty = ref(false)
  /** 保存状态 */
  const saving = ref(false)
  /** 历史快照列表 */
  const historySnapshots = ref<HistorySnapshot[]>([])
  /** 字数统计 */
  const wordCount = ref(0)
  /** 写作时长（秒） */
  const writingDuration = ref(0)
  /** 计时器 */
  let timerInterval: ReturnType<typeof setInterval> | null = null

  /** 格式化的写作时长 */
  const formattedDuration = computed(() => formatDuration(writingDuration.value))

  /** 加载文件内容 */
  async function loadFile(path: string) {
    currentFilePath.value = path
    try {
      const raw = await api.readFile(path)
      content.value = raw
      isDirty.value = false
      updateWordCount()
      await loadHistory()
    } catch (e) {
      console.error('Failed to load file:', e)
      content.value = ''
    }
  }

  /** 保存文件 */
  async function saveFile() {
    if (!currentFilePath.value) return
    saving.value = true
    try {
      // content 已经是 Markdown 格式，直接写入
      await api.writeFile({
        path: currentFilePath.value,
        content: content.value,
      })
      isDirty.value = false
      updateWordCount()
    } catch (e) {
      console.error('Failed to save file:', e)
    } finally {
      saving.value = false
    }
  }

  /** 设置内容（从编辑器更新） */
  function setContent(markdown: string) {
    content.value = markdown
    isDirty.value = true
    updateWordCount()
  }

  /** 更新字数统计 */
  function updateWordCount() {
    wordCount.value = countMarkdownWords(content.value)
  }

  /** 清空编辑器状态 */
  function clear() {
    content.value = ''
    currentFilePath.value = ''
    isDirty.value = false
    historySnapshots.value = []
    wordCount.value = 0
    resetTimer()
  }

  /** 加载历史快照 */
  async function loadHistory() {
    if (!currentFilePath.value) return
    try {
      historySnapshots.value = await api.listHistory(currentFilePath.value)
    } catch (e) {
      console.error('Failed to load history:', e)
    }
  }

  /** 从快照恢复 */
  async function restoreFromSnapshot(snapshotPath: string) {
    if (!currentFilePath.value) return
    await api.restoreSnapshot(snapshotPath, currentFilePath.value)
    await loadFile(currentFilePath.value)
  }

  // ========== 写作计时 ==========

  function startTimer() {
    if (timerInterval) return
    timerInterval = setInterval(() => {
      writingDuration.value++
    }, 1000)
  }

  function stopTimer() {
    if (timerInterval) {
      clearInterval(timerInterval)
      timerInterval = null
    }
  }

  function resetTimer() {
    stopTimer()
    writingDuration.value = 0
  }

  function formatDuration(seconds: number): string {
    const h = Math.floor(seconds / 3600)
    const m = Math.floor((seconds % 3600) / 60)
    const s = seconds % 60
    if (h > 0) {
      return `${h}小时${m}分`
    }
    if (m > 0) {
      return `${m}分${s}秒`
    }
    return `${s}秒`
  }

  return {
    content,
    currentFilePath,
    isDirty,
    saving,
    historySnapshots,
    wordCount,
    writingDuration,
    formattedDuration,
    loadFile,
    saveFile,
    setContent,
    updateWordCount,
    loadHistory,
    restoreFromSnapshot,
    startTimer,
    stopTimer,
    formatDuration,
    clear,
  }
})