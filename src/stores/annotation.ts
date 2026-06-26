/**
 * 灵感批注状态管理
 *
 * 管理所有批注的元数据（ID、内容、时间戳等），
 * 与编辑器中的 annotation mark 通过 UUID 关联。
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export interface AnnotationData {
  id: string
  /** 批注内容 */
  content: string
  /** 创建时间 */
  createdAt: string
  /** 修改时间 */
  modifiedAt: string
  /** 被标注的文本片段（用于侧边栏预览） */
  quotedText: string
}

export const useAnnotationStore = defineStore('annotation', () => {
  /** 所有批注，key 为 annotationId */
  const annotations = ref<Map<string, AnnotationData>>(new Map())
  /** 当前编辑的批注 ID */
  const editingId = ref<string | null>(null)
  /** 当前悬停的批注 ID */
  const hoveredId = ref<string | null>(null)
  /** 侧边栏是否展开 */
  const panelOpen = ref(false)

  /** 批注列表（按创建时间排序） */
  const annotationList = computed(() => {
    return Array.from(annotations.value.values())
      .sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime())
  })

  /** 获取某个批注 */
  function getAnnotation(id: string): AnnotationData | undefined {
    return annotations.value.get(id)
  }

  /** 添加新批注 */
  function addAnnotation(id: string, quotedText: string = ''): AnnotationData {
    const now = new Date().toISOString()
    const data: AnnotationData = {
      id,
      content: '',
      createdAt: now,
      modifiedAt: now,
      quotedText,
    }
    annotations.value.set(id, data)
    editingId.value = id
    return data
  }

  /** 更新批注内容 */
  function updateAnnotation(id: string, content: string) {
    const existing = annotations.value.get(id)
    if (existing) {
      existing.content = content
      existing.modifiedAt = new Date().toISOString()
    }
  }

  /** 删除批注 */
  function removeAnnotation(id: string) {
    annotations.value.delete(id)
    if (editingId.value === id) {
      editingId.value = null
    }
    if (hoveredId.value === id) {
      hoveredId.value = null
    }
  }

  /** 批量加载批注（从文件解析时） */
  function loadAnnotations(data: AnnotationData[]) {
    annotations.value.clear()
    for (const item of data) {
      annotations.value.set(item.id, item)
    }
  }

  /** 获取所有批注的序列化数据 */
  function getAllAnnotations(): AnnotationData[] {
    return annotationList.value
  }

  /** 设置编辑中的批注 */
  function setEditing(id: string | null) {
    editingId.value = id
  }

  /** 设置悬停的批注 */
  function setHovered(id: string | null) {
    hoveredId.value = id
  }

  /** 切换侧边栏 */
  function togglePanel() {
    panelOpen.value = !panelOpen.value
  }

  function openPanel() {
    panelOpen.value = true
  }

  function closePanel() {
    panelOpen.value = false
  }

  return {
    annotations,
    annotationList,
    editingId,
    hoveredId,
    panelOpen,
    getAnnotation,
    addAnnotation,
    updateAnnotation,
    removeAnnotation,
    loadAnnotations,
    getAllAnnotations,
    setEditing,
    setHovered,
    togglePanel,
    openPanel,
    closePanel,
  }
})