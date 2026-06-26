/**
 * Markdown ↔ HTML 双向转换工具
 *
 * 数据流：
 *   .md 文件 --[marked]--> HTML --[Tiptap]--> 编辑器渲染
 *   编辑器  --[Tiptap.getHTML()]--> HTML --[turndown]--> .md 文件
 *
 * 灵感注释（<!-- ... -->）在转换中被保留，在 TXT 导出时被 Rust 端清洗。
 */

import { marked } from 'marked'
import TurndownService from 'turndown'
import type { AnnotationData } from '@/stores/annotation'

// ========== Markdown → HTML ==========

// 配置 marked，保留 HTML 注释（灵感标记）
marked.setOptions({
  gfm: true,
  breaks: false, // 中文写作不自动换行
})

/**
 * 将 Markdown 文本转为 HTML（用于加载到 Tiptap 编辑器）
 */
export function markdownToHtml(markdown: string): string {
  if (!markdown || !markdown.trim()) {
    return ''
  }
  const result = marked.parse(markdown)
  return typeof result === 'string' ? result : ''
}

// ========== HTML → Markdown ==========

// 创建 turndown 实例，针对中文写作优化
const turndown = new TurndownService({
  headingStyle: 'atx',
  hr: '---',
  bulletListMarker: '-',
  codeBlockStyle: 'fenced',
  emDelimiter: '*',
  strongDelimiter: '**',
})

// 自定义规则：保留 HTML 注释（灵感标记）
turndown.addRule('preserveComments', {
  filter: (node: Node) => {
    return node.nodeType === Node.COMMENT_NODE
  },
  replacement: (_content: string, node: TurndownService.Node) => {
    const commentNode = node as unknown as Comment
    return `<!--${commentNode.nodeValue ?? ''}-->`
  },
})

// 确保段落不使用物理缩进（缩进由 CSS text-indent 控制）
turndown.addRule('paragraph', {
  filter: 'p',
  replacement: (content: string) => {
    const trimmed = content.trim()
    if (!trimmed || trimmed === '<br>') {
      return '\n\n'
    }
    return `\n\n${trimmed}\n\n`
  },
})

// 处理 <u> 标签（下划线）
turndown.addRule('underline', {
  filter: (node: Node) => {
    return node.nodeName === 'U'
  },
  replacement: (content: string) => {
    return `<u>${content}</u>`
  },
})

/**
 * 将 HTML 转为 Markdown 文本（用于保存到 .md 文件）
 */
export function htmlToMarkdown(html: string): string {
  if (!html || !html.trim()) {
    return ''
  }

  // 保护灵感批注注释，turndown 会完全丢弃 HTML Comment 节点（nodeType 8），
  // 自定义规则 preserveComments 根本不会被调用。
  // 解决方案：在 turndown 处理前用占位符替换，处理后还原。
  const commentSlots: string[] = []
  let safeHtml = html.replace(
    /(<!--inspiration:[\s\S]*?-->)([\s\S]*?)(<!--\/inspiration-->)/g,
    (_m, open: string, text: string, close: string) => {
      const idx = commentSlots.length
      commentSlots.push(open)
      commentSlots.push(close)
      return `INSPPLACEHOLDER${idx * 2}END${text}INSPPLACEHOLDER${idx * 2 + 1}END`
    },
  )

  let markdown = turndown.turndown(safeHtml)

  // 还原灵感批注注释
  markdown = markdown.replace(/INSPPLACEHOLDER(\d+)END/g, (_m, idx) => {
    return commentSlots[parseInt(idx)] ?? ''
  })

  // 清理多余空行
  markdown = markdown.replace(/\n{4,}/g, '\n\n')

  // 确保开头没有多余空行
  markdown = markdown.trimStart()

  return markdown
}

/**
 * 统计 Markdown 文本中的字数
 */
export function countMarkdownWords(markdown: string): number {
  // 移除 Markdown 语法
  let text = markdown
  // 移除 HTML 注释
  text = text.replace(/<!--[\s\S]*?-->/g, '')
  // 移除代码块
  text = text.replace(/```[\s\S]*?```/g, '')
  // 移除行内代码
  text = text.replace(/`[^`]+`/g, '')
  // 移除标题标记
  text = text.replace(/^#{1,6}\s+/gm, '')
  // 移除加粗/斜体
  text = text.replace(/\*{1,3}(.+?)\*{1,3}/g, '$1')
  // 移除链接
  text = text.replace(/\[([^\]]+)\]\([^)]+\)/g, '$1')
  // 移除图片
  text = text.replace(/!\[([^\]]*)\]\([^)]+\)/g, '')
  // 移除列表标记
  text = text.replace(/^[\s]*[-*+]\s+/gm, '')
  text = text.replace(/^[\s]*\d+\.\s+/gm, '')
  // 移除水平线
  text = text.replace(/^[-*_]{3,}\s*$/gm, '')
  // 移除引用标记
  text = text.replace(/^>\s?/gm, '')

  // 统计中文字符和英文单词
  const chineseChars = (text.match(/[一-鿿]/g) || []).length
  const englishWords = text
    .replace(/[一-鿿]/g, ' ')
    .split(/\s+/)
    .filter((w) => w.length > 0).length

  return chineseChars + englishWords
}

/**
 * 将 Markdown 转为纯文本预览（用于侧边栏预览等）
 */
export function markdownToPlainText(markdown: string, maxLength: number = 100): string {
  const html = markdownToHtml(markdown)
  // 简单的 HTML 标签剥离
  const text = html
    .replace(/<[^>]+>/g, '')
    .replace(/<!--[\s\S]*?-->/g, '')
    .replace(/&amp;/g, '&')
    .replace(/&lt;/g, '<')
    .replace(/&gt;/g, '>')
    .replace(/&quot;/g, '"')
    .replace(/&#39;/g, "'")
    .trim()

  if (text.length <= maxLength) return text
  return text.slice(0, maxLength) + '…'
}

// ========== 灵感批注序列化/反序列化 ==========

const ANNOTATION_SPAN_RE = /<span(?:\s+[^>]*?)?\s+data-annotation-id="([^"]*)"(?:\s+[^>]*?)?>([\s\S]*?)<\/span>/g
const INSPIRATION_COMMENT_RE = /<!--inspiration:(\{[\s\S]*?\})-->([\s\S]*?)<!--\/inspiration-->/g

/**
 * 将编辑器 HTML 中的 annotation span 转为灵感注释
 *
 * 输入：<span data-annotation-id="uuid" class="annotation">文字</span>
 * 输出：<!--inspiration:{"id":"uuid","content":"...","createdAt":"..."}-->文字<!--/inspiration-->
 *
 * @param html 编辑器输出的 HTML
 * @param annotationMap 批注 ID → 批注数据 的映射
 * @returns 包含注释的 HTML
 */
export function annotationsToComments(
  html: string,
  annotationMap: Map<string, AnnotationData>,
): string {
  return html.replace(
    ANNOTATION_SPAN_RE,
    (_match: string, id: string, text: string) => {
      const data = annotationMap.get(id)
      if (!data) return text // 找不到数据则只保留文字

      const commentData = {
        id: data.id,
        content: data.content,
        createdAt: data.createdAt,
        modifiedAt: data.modifiedAt,
        quotedText: text,
      }
      const json = JSON.stringify(commentData)
      return `<!--inspiration:${json}-->${text}<!--/inspiration-->`
    },
  )
}

/**
 * 将 Markdown 中的灵感注释转为编辑器 annotation span
 *
 * 输入：<!--inspiration:{...}-->文字<!--/inspiration-->
 * 输出：<span data-annotation-id="uuid" class="annotation">文字</span>
 *
 * @param html 从 Markdown 解析后的 HTML
 * @returns {{ html: 替换后的 HTML, annotations: 提取出的批注数据数组 }}
 */
export function commentsToAnnotations(html: string): {
  html: string
  annotations: AnnotationData[]
} {
  const extracted: AnnotationData[] = []

  const result = html.replace(
    INSPIRATION_COMMENT_RE,
    (_match: string, jsonStr: string, text: string) => {
      try {
        const data = JSON.parse(jsonStr)
        extracted.push({
          id: data.id,
          content: data.content || '',
          createdAt: data.createdAt || new Date().toISOString(),
          modifiedAt: data.modifiedAt || data.createdAt || new Date().toISOString(),
          quotedText: data.quotedText || text || '',
        })
        return `<span data-annotation-id="${data.id}" class="annotation">${text}</span>`
      } catch {
        // JSON 解析失败，保留原文
        return text
      }
    },
  )

  return { html: result, annotations: extracted }
}

/**
 * 预处理 Markdown 文本，确保灵感注释格式正确
 * 修复可能被编辑器破坏的注释格式
 */
export function preprocessMarkdownForLoad(markdown: string): string {
  // 修复不完整的 inspiration 注释：缺少结束标记的情况
  let result = markdown
  // 匹配 <!--inspiration:...--> 后面没有 <!--/inspiration--> 的情况，自动闭合
  result = result.replace(
    /<!--inspiration:\{[\s\S]*?\}-->([\s\S]*?)(?=<!--inspiration:|<!--\/inspiration-->|$)/g,
    (_match: string, _json: string, text: string) => {
      // 如果 text 后面没有闭合标签，添加一个
      if (!_match.includes('<!--/inspiration-->')) {
        return `<!--inspiration:${_json}-->${text}<!--/inspiration-->`
      }
      return _match
    },
  )
  return result
}