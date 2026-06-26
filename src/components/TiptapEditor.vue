<script setup lang="ts">

import { watch, onUnmounted, ref } from 'vue'

import { useEditor, EditorContent } from '@tiptap/vue-3'

import StarterKit from '@tiptap/starter-kit'

import Underline from '@tiptap/extension-underline'

import Placeholder from '@tiptap/extension-placeholder'

import CharacterCount from '@tiptap/extension-character-count'

import { useThemeStore } from '@/stores/theme'

import { useAnnotationStore } from '@/stores/annotation'

import { Annotation } from '@/extensions/Annotation'

import {

  markdownToHtml,

  htmlToMarkdown,

  annotationsToComments,

  commentsToAnnotations,

} from '@/composables/useMarkdown'

const props = defineProps<{

  /** 编辑器内容（Markdown 格式?*/

  content: string

  /** 当前编辑的文件路?*/

  filePath: string

  /** 是否显示稿纸网格?*/

  showGrid: boolean

}>()

const emit = defineEmits<{

  /** 内容变化时触发，传?Markdown 格式 */

  'update:content': [markdown: string]

  /** 字数变化 */

  'wordCount': [count: number]

  /** 编辑器就?*/

  'ready': []

}>()

const theme = useThemeStore()

const annotationStore = useAnnotationStore()

// ?Markdown 转为 HTML 用于编辑器显?// 流程：Markdown ?HTML ?注释?span + 提取批注数据

const rawHtml = markdownToHtml(props.content)

const { html: processedHtml, annotations: loadedAnnotations } = commentsToAnnotations(rawHtml)

annotationStore.loadAnnotations(loadedAnnotations)

const editor = useEditor({

  content: processedHtml,

  extensions: [

    StarterKit.configure({

      heading: {

        levels: [1, 2, 3, 4],

      },

      horizontalRule: {

        HTMLAttributes: {

          class: 'editor-hr',

        },

      },

      codeBlock: false,

    }),

    Underline,

    Annotation,

    Placeholder.configure({

      placeholder: '开始写作…',

      emptyEditorClass: 'is-editor-empty',

    }),

    CharacterCount.configure({

      limit: null, // 无字数上限
    }),

  ],

  editorProps: {

    attributes: {

      class: 'tiptap-editor',

      spellcheck: 'false',

    },

  },

  onUpdate: ({ editor }) => {

    // 获取编辑HTML（含 annotation span）
    const editorHtml = editor.getHTML()

    // annotation span 转为灵感注释

    const htmlWithComments = annotationsToComments(editorHtml, annotationStore.annotations)

    // 转为 Markdown

    const markdown = htmlToMarkdown(htmlWithComments)

    emit('update:content', markdown)

    emit('wordCount', editor.storage.characterCount?.characters() ?? 0)

  },

  onCreate: () => {

    emit('ready')

  },

})

// 编辑器容器引用
const editorWrapper = ref<HTMLElement | null>(null)

// 计算网格偏移量（使网格线与文字底端对齐）

function updateGridPosition() {

  if (!editorWrapper.value) return

  const paddingTop = 48 // 匹配编辑器上 padding

  // 为了让底部对齐网格线，我们需要让第一条网格线恰好出现在第一行文字的底部。
  // 一行的高度是 fontSize * lineHeight

  // 所以偏移量是 paddingTop + fontSize * lineHeight

  const offset = paddingTop + theme.config.fontSize * theme.config.lineHeight

  editorWrapper.value.style.backgroundPosition = `0 ${offset}px`

}

// 监听外部内容变化（切换文件时）
watch(

  () => props.content,

  (newContent) => {

    if (!editor.value) return

    // 获取当前编辑器的 Markdown 内容

    const editorHtml = editor.value.getHTML()

    const htmlWithComments = annotationsToComments(editorHtml, annotationStore.annotations)

    const currentMarkdown = htmlToMarkdown(htmlWithComments)

    // 如果内容确实不同，更新编辑器

    if (currentMarkdown !== newContent) {

      const rawHtml = markdownToHtml(newContent)

      const { html: newHtml, annotations: newAnnotations } = commentsToAnnotations(rawHtml)

      annotationStore.loadAnnotations(newAnnotations)

      editor.value.commands.setContent(newHtml)

    }

  }

)

// 监听文件路径变化（切换到新文件）

watch(

  () => props.filePath,

  (newPath, oldPath) => {

    if (!editor.value || !newPath || newPath === oldPath) return

    const rawHtml = markdownToHtml(props.content)

    const { html: newHtml, annotations: newAnnotations } = commentsToAnnotations(rawHtml)

    annotationStore.loadAnnotations(newAnnotations)

    editor.value.commands.setContent(newHtml)

  }

)

// 监听主题字体/间距变化，实时更新网格线位置

watch(

  () => [

    theme.config.fontSize,

    theme.config.lineHeight,

  ],

  () => {

    updateGridPosition()

  },

  { immediate: true }

)

onUnmounted(() => {

  editor.value?.destroy()

})

// ========== 悬停联动：侧边栏卡片 ?编辑器批注文?==========

watch(

  () => annotationStore.hoveredId,

  (newId, oldId) => {

    // 移除旧的高亮

    if (oldId) {

      const oldEls = document.querySelectorAll(`.tiptap-editor .annotation[data-annotation-id="${oldId}"]`)

      oldEls.forEach((el) => el.classList.remove('active'))

    }

    // 添加新的高亮

    if (newId) {

      const newEls = document.querySelectorAll(`.tiptap-editor .annotation[data-annotation-id="${newId}"]`)

      newEls.forEach((el) => el.classList.add('active'))

    }

  },

)

// ========== 暴露给父组件的方?==========

/** 在当前选中文字上添加批?*/

function addAnnotation(annotationId: string): boolean {

  if (!editor.value) return false

  const { from, to, empty } = editor.value.state.selection

  if (empty) return false // 没有选中文字

  // 获取选中文字

  const selectedText = editor.value.state.doc.textBetween(from, to)

  if (!selectedText.trim()) return false

  // ?store 中记录批注，必须先执行以?onUpdate 同步触发时找不到数据

  annotationStore.addAnnotation(annotationId, selectedText.trim())

  // 应用 annotation mark，这会同步触?onUpdate

  editor.value.chain().focus().setMark('annotation', { annotationId }).run()

  return true

}

/** 移除当前光标位置的批?*/

function removeAnnotation(): boolean {

  if (!editor.value) return false

  const { from, to } = editor.value.state.selection

  const marks = editor.value.state.doc.rangeHasMark(from, to, editor.value.schema.marks.annotation)

  if (!marks) return false

  // 获取 annotation ID

  const resolvedFrom = editor.value.state.doc.resolve(from)

  const annotationMarks = resolvedFrom.marks().filter(

    (m) => m.type.name === 'annotation'

  )

  for (const mark of annotationMarks) {

    const id = mark.attrs.annotationId

    annotationStore.removeAnnotation(id)

  }

  editor.value.chain().focus().unsetMark('annotation').run()

  return true

}

/** 滚动到指定批注对应的文字位置 */

function scrollToAnnotation(annotationId: string): void {

  if (!editor.value) return

  const doc = editor.value.state.doc

  let found = false

  doc.descendants((node, pos) => {

    if (found) return false

    if (node.marks) {

      for (const mark of node.marks) {

        if (mark.type.name === 'annotation' && mark.attrs.annotationId === annotationId) {

          // 选中该文?          editor.value!.chain().focus().setTextSelection({ from: pos, to: pos + node.nodeSize }).run()

          // 滚动到视图
          const dom = editor.value!.view.domAtPos(pos)

          if (dom.node) {

            const el = dom.node.nodeType === Node.TEXT_NODE ? dom.node.parentElement : dom.node as HTMLElement

            el?.scrollIntoView({ behavior: 'smooth', block: 'center' })

          }

          found = true

          return false

        }

      }

    }

    return true

  })

}

defineExpose({

  addAnnotation,

  removeAnnotation,

  scrollToAnnotation,

})

</script>

<template>

  <div

    ref="editorWrapper"

    class="tiptap-wrapper"

    :class="{ 'has-grid': props.showGrid }"

    :style="{

      '--editor-font-size': `${theme.config.fontSize}px`,

      '--editor-line-height': theme.config.lineHeight,

      '--editor-letter-spacing': `${theme.config.letterSpacing}px`,

      '--editor-font-family': theme.config.fontFamily,

      '--editor-margin-left': `${theme.config.marginLeft}px`,

      '--editor-margin-right': `${theme.config.marginRight}px`

    }"

  >

    <EditorContent :editor="editor" />

  </div>

</template>

<style>

/* ============================================

   Tiptap 编辑器样?   极简设计，专注于中文写作体验

   ============================================ */

.tiptap-wrapper {

  height: 100%;

  overflow-y: auto;

  overflow-x: hidden;

  position: relative;

}

/* 稿纸网格线（应用于编辑器容器?*/

.tiptap-wrapper.has-grid {

  background-image: repeating-linear-gradient(

    to bottom,

    transparent,

    transparent calc(var(--grid-line-height) - 1px),

    var(--border-color) calc(var(--grid-line-height) - 1px),

    var(--border-color) var(--grid-line-height)

  );

  background-size: 100% var(--grid-line-height);

  /* 让网格线跟随内容滚动，而非固定在容器上 */

  background-attachment: local;

  /* 网格偏移 = 编辑器上 padding，使网格线与文字对齐 */

  background-position: 0 48px;

}

/* ProseMirror 核心区域 */

.tiptap-editor {

  min-height: 100%;

  padding: 48px var(--editor-margin-right, 80px) 48px var(--editor-margin-left, 80px);

  outline: none;

  color: var(--text-color);

  font-size: var(--editor-font-size, 16px);

  line-height: var(--editor-line-height, 2.0);

  letter-spacing: var(--editor-letter-spacing, 0.5px);

  font-family: var(--editor-font-family, "Noto Serif SC", "Source Han Serif SC", "SimSun", "宋体", serif);

  max-width: 880px;

  margin: 0 auto;

  word-break: break-all;

  overflow-wrap: break-word;

}

/* ========== 标题 ========== */

.tiptap-editor h1 {

  font-size: 1.75em;

  font-weight: 700;

  margin: 1.5em 0 0.6em;

  line-height: 1.35;

  text-indent: 0;

  letter-spacing: 0.05em;

}

.tiptap-editor h2 {

  font-size: 1.4em;

  font-weight: 600;

  margin: 1.2em 0 0.5em;

  line-height: 1.4;

  text-indent: 0;

}

.tiptap-editor h3 {

  font-size: 1.15em;

  font-weight: 600;

  margin: 1em 0 0.4em;

  line-height: 1.4;

  text-indent: 0;

}

.tiptap-editor h4 {

  font-size: 1.05em;

  font-weight: 600;

  margin: 0.8em 0 0.3em;

  line-height: 1.4;

  text-indent: 0;

}

/* ========== 段落（中文排版核心） ========== */

.tiptap-editor p {

  text-indent: 2em;

  margin: 0;

  padding: 0;

}

/* 标题后的第一段保持缩?*/

.tiptap-editor h1 + p,

.tiptap-editor h2 + p,

.tiptap-editor h3 + p,

.tiptap-editor h4 + p {

  text-indent: 2em;

}

/* ========== 行内样式 ========== */

.tiptap-editor strong {

  font-weight: 700;

  color: inherit;

}

.tiptap-editor em {

  font-style: italic;

}

.tiptap-editor u {

  text-decoration: underline;

  text-underline-offset: 0.2em;

}

.tiptap-editor s {

  text-decoration: line-through;

  opacity: 0.6;

}

/* ========== 引用?========== */

.tiptap-editor blockquote {

  border-left: 3px solid var(--accent-color);

  padding: 0.4em 0 0.4em 1.2em;

  margin: 0.8em 0;

  opacity: 0.85;

  text-indent: 0;

}

.tiptap-editor blockquote p {

  text-indent: 0;

}

/* ========== 代码 ========== */

.tiptap-editor code {

  font-family: "Cascadia Code", "Fira Code", "Consolas", monospace;

  font-size: 0.88em;

  background: var(--border-color);

  padding: 2px 6px;

  border-radius: 3px;

  text-indent: 0;

}

.tiptap-editor pre {

  background: var(--border-color);

  padding: 16px 20px;

  border-radius: 6px;

  margin: 0.8em 0;

  overflow-x: auto;

  text-indent: 0;

}

.tiptap-editor pre code {

  background: none;

  padding: 0;

  font-size: 0.9em;

  line-height: 1.5;

}

/* ========== 列表 ========== */

.tiptap-editor ul,

.tiptap-editor ol {

  padding-left: 2em;

  margin: 0.4em 0;

  text-indent: 0;

}

.tiptap-editor li {

  margin: 0.15em 0;

  text-indent: 0;

}

.tiptap-editor li p {

  text-indent: 0;

  margin: 0;

}

/* 嵌套列表 */

.tiptap-editor ul ul,

.tiptap-editor ol ol,

.tiptap-editor ul ol,

.tiptap-editor ol ul {

  margin: 0;

}

/* ========== 分割?========== */

.tiptap-editor hr {

  border: none;

  border-top: 1px solid var(--border-color);

  margin: 2em 0;

  text-indent: 0;

}

/* ========== 占位?========== */

.tiptap-editor p.is-editor-empty:first-child::before {

  content: attr(data-placeholder);

  float: left;

  color: var(--text-color);

  opacity: 0.25;

  pointer-events: none;

  height: 0;

  text-indent: 2em;

}

/* ========== 选中文字颜色 ========== */

.tiptap-editor ::selection {

  background: color-mix(in srgb, var(--accent-color) 20%, transparent);

}

/* ========== 焦点样式 ========== */

.tiptap-editor:focus {

  outline: none;

}

/* ========== 图片 ========== */

.tiptap-editor img {

  max-width: 100%;

  height: auto;

  border-radius: 4px;

  margin: 0.8em 0;

}

/* ========== 链接 ========== */

.tiptap-editor a {

  color: var(--accent-color);

  text-decoration: underline;

  text-underline-offset: 0.15em;

  cursor: pointer;

}

/* ========== 灵感批注标记 ========== */

.tiptap-editor .annotation {

  text-decoration: underline;

  text-decoration-style: wavy;

  text-decoration-color: var(--accent-color);

  text-underline-offset: 0.3em;

  background: color-mix(in srgb, var(--accent-color) 8%, transparent);

  border-radius: 2px;

  padding: 0 2px;

  cursor: pointer;

  transition: background 0.15s;

}

.tiptap-editor .annotation:hover {

  background: color-mix(in srgb, var(--accent-color) 18%, transparent);

}

/* 侧边栏中高亮对应的批?*/

.tiptap-editor .annotation.active {

  background: color-mix(in srgb, var(--accent-color) 25%, transparent);

  text-decoration-color: #6b5335;

}

/* ========== 表格（未来扩展） ========== */

.tiptap-editor table {

  border-collapse: collapse;

  margin: 0.8em 0;

  width: 100%;

  text-indent: 0;

}

.tiptap-editor th,

.tiptap-editor td {

  border: 1px solid var(--border-color);

  padding: 8px 12px;

  text-align: left;

  text-indent: 0;

}

.tiptap-editor th {

  background: var(--bg-color);

  font-weight: 600;

}

</style>