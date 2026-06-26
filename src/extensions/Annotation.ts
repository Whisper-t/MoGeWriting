/**
 * 自定义 Tiptap Annotation Mark 扩展
 *
 * 用于在编辑器中标记「灵感批注」文字。
 * 标记的文字会以特殊下划线高亮显示。
 *
 * 存储格式（Markdown 源码）：
 *   <!--inspiration:{"id":"uuid","content":"灵感内容","createdAt":"..."}-->被标注的文字<!--/inspiration-->
 *
 * 编辑器内渲染：
 *   <span class="annotation" data-annotation-id="uuid">被标注的文字</span>
 */

import { Mark, mergeAttributes } from '@tiptap/core'

export interface AnnotationAttributes {
  annotationId: string
}

export const Annotation = Mark.create<{
  annotationId: string
}>({
  name: 'annotation',

  // 定义属性
  addAttributes() {
    return {
      annotationId: {
        default: null,
        parseHTML: (element) => element.getAttribute('data-annotation-id'),
        renderHTML: (attributes) => {
          if (!attributes.annotationId) {
            return {}
          }
          return {
            'data-annotation-id': attributes.annotationId,
            class: 'annotation',
          }
        },
      },
    }
  },

  // 解析 HTML：从 HTML 中识别 annotation mark
  parseHTML() {
    return [
      {
        tag: 'span[data-annotation-id]',
        getAttrs: (element) => {
          if (typeof element === 'string') return {}
          const annotationId = element.getAttribute('data-annotation-id')
          if (!annotationId) return false
          return { annotationId }
        },
      },
    ]
  },

  // 渲染为 HTML
  renderHTML({ HTMLAttributes }) {
    return [
      'span',
      mergeAttributes(
        { 'data-annotation-id': HTMLAttributes.annotationId, class: 'annotation' },
        HTMLAttributes,
      ),
      0,
    ]
  },

  // 快捷键：Ctrl+Shift+A 添加批注
  addKeyboardShortcuts() {
    return {
      'Mod-Shift-a': () => {
        return this.editor.commands.toggleMark(this.name, {
          annotationId: '', // 占位，实际 ID 由外部注入
        })
      },
    }
  },
})