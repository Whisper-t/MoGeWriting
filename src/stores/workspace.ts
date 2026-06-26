import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { WorkspaceSummary, BookIndex, VolumeEntry, ChapterEntry } from '@/types'
import * as api from '@/composables/useTauriCommands'
import { useEditorStore } from '@/stores/editor'

export const useWorkspaceStore = defineStore('workspace', () => {
  // 根目录路径
  const rootPath = ref('')
  // 工作区列表
  const workspaces = ref<WorkspaceSummary[]>([])
  // 当前打开的工作区
  const currentWorkspace = ref<WorkspaceSummary | null>(null)
  // 当前 book.json 索引
  const bookIndex = ref<BookIndex | null>(null)
  // 当前选中的章节
  const selectedChapter = ref<ChapterEntry | null>(null)
  // 加载状态
  const loading = ref(false)

  async function init(root: string) {
    rootPath.value = root
    await refreshWorkspaces()
  }

  async function refreshWorkspaces() {
    if (!rootPath.value) return
    loading.value = true
    try {
      workspaces.value = await api.listWorkspaces(rootPath.value)
    } catch (e) {
      console.error('Failed to list workspaces:', e)
    } finally {
      loading.value = false
    }
  }

  async function createWorkspace(
    name: string,
    workType: string,
    structureType: string,
    description?: string,
    author?: string
  ) {
    const ws = await api.createWorkspace(rootPath.value, name, workType, structureType, description, author)
    await refreshWorkspaces()
    return ws
  }

  async function deleteWorkspace(path: string) {
    await api.deleteWorkspace(path)
    if (currentWorkspace.value?.path === path) {
      currentWorkspace.value = null
      bookIndex.value = null
    }
    await refreshWorkspaces()
  }

  async function openWorkspace(ws: WorkspaceSummary) {
    currentWorkspace.value = ws
    bookIndex.value = await api.readBookIndex(ws.path)
  }

  function closeWorkspace() {
    currentWorkspace.value = null
    bookIndex.value = null
    selectedChapter.value = null
    useEditorStore().clear()
  }

  async function createChapter(title: string, volumeId?: string) {
    if (!currentWorkspace.value) return
    const book = await api.createChapter({
      book_path: currentWorkspace.value.path,
      title,
      volume_id: volumeId,
    })
    bookIndex.value = book
  }

  async function createVolume(title: string) {
    if (!currentWorkspace.value) return
    const book = await api.createVolume({
      book_path: currentWorkspace.value.path,
      title,
    })
    bookIndex.value = book
  }

  async function renameEntry(oldPath: string, newName: string) {
    if (!currentWorkspace.value) return
    const book = await api.renameEntry({
      book_path: currentWorkspace.value.path,
      old_path: oldPath,
      new_name: newName,
    })
    bookIndex.value = book
  }

  async function deleteChapter(chapterId: string) {
    if (!currentWorkspace.value) return
    const book = await api.deleteChapter(currentWorkspace.value.path, chapterId)
    bookIndex.value = book
    if (selectedChapter.value?.id === chapterId) {
      selectedChapter.value = null
    }
  }

  async function deleteVolume(volumeId: string) {
    if (!currentWorkspace.value) return
    const book = await api.deleteVolume(currentWorkspace.value.path, volumeId)
    bookIndex.value = book
  }

  async function reorder(volumes?: VolumeEntry[], chapters?: ChapterEntry[]) {
    if (!currentWorkspace.value) return
    const book = await api.reorder({
      book_path: currentWorkspace.value.path,
      volumes,
      chapters,
    })
    bookIndex.value = book
  }

  function selectChapter(chapter: ChapterEntry | null) {
    selectedChapter.value = chapter
  }

  /** 直接写入 bookIndex（用于更新简介等元数据） */
  async function writeBookIndex(book: BookIndex) {
    if (!currentWorkspace.value) return
    await api.writeBookIndex(currentWorkspace.value.path, book)
    bookIndex.value = book
  }

  async function setCoverImage(imagePath: string) {
    if (!currentWorkspace.value || !bookIndex.value) return
    const coverName = await api.setCoverImage(currentWorkspace.value.path, imagePath)
    const updated = { ...bookIndex.value, cover_image: coverName }
    await writeBookIndex(updated)
  }

  // 获取章节完整路径
  function getChapterPath(chapter: ChapterEntry): string {
    if (!currentWorkspace.value) return ''
    return `${currentWorkspace.value.path}\\${chapter.file}`
  }

  return {
    rootPath,
    workspaces,
    currentWorkspace,
    bookIndex,
    selectedChapter,
    loading,
    init,
    refreshWorkspaces,
    createWorkspace,
    deleteWorkspace,
    openWorkspace,
    closeWorkspace,
    createChapter,
    createVolume,
    renameEntry,
    deleteChapter,
    deleteVolume,
    reorder,
    selectChapter,
    writeBookIndex,
    setCoverImage,
    getChapterPath,
  }
})