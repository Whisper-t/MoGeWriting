import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export type ThemeMode = 'light' | 'dark' | 'sepia'

export interface ThemeConfig {
  mode: ThemeMode
  fontSize: number
  fontFamily: string
  lineHeight: number
  letterSpacing: number
  marginLeft: number
  marginRight: number
  showGrid: boolean
}

export const useThemeStore = defineStore('theme', () => {
  const config = ref<ThemeConfig>({
    mode: 'light',
    fontSize: 16,
    fontFamily: '"Noto Serif SC", "Source Han Serif SC", "SimSun", "宋体", serif',
    lineHeight: 2.0,
    letterSpacing: 0.5,
    marginLeft: 80,
    marginRight: 80,
    showGrid: false,
  })

  // Load from localStorage
  const saved = localStorage.getItem('app-theme')
  if (saved) {
    try {
      const parsed = JSON.parse(saved)
      config.value = { ...config.value, ...parsed }
    } catch (e) {
      console.error('Failed to parse theme config', e)
    }
  }

  function saveConfig() {
    localStorage.setItem('app-theme', JSON.stringify(config.value))
  }

  const themeMode = computed(() => config.value.mode)
  const fontSize = computed(() => config.value.fontSize)
  const lineHeight = computed(() => config.value.lineHeight)

  // 计算网格线高度（像素）
  const gridLineHeight = computed(() => {
    return config.value.fontSize * config.value.lineHeight
  })

  function setTheme(mode: ThemeMode) {
    config.value.mode = mode
    saveConfig()
    applyTheme()
  }

  function setFontSize(size: number) {
    config.value.fontSize = Math.max(12, Math.min(28, size))
    saveConfig()
    applyTheme()
  }

  function setFontFamily(family: string) {
    config.value.fontFamily = family
    saveConfig()
    applyTheme()
  }

  function setLineHeight(height: number) {
    config.value.lineHeight = Math.max(1.2, Math.min(3.0, height))
    saveConfig()
    applyTheme()
  }

  function setLetterSpacing(spacing: number) {
    config.value.letterSpacing = Math.max(0, Math.min(5, spacing))
    saveConfig()
    applyTheme()
  }

  function setMargins(left: number, right: number) {
    config.value.marginLeft = Math.max(20, Math.min(200, left))
    config.value.marginRight = Math.max(20, Math.min(200, right))
    saveConfig()
    applyTheme()
  }

  function toggleGrid() {
    config.value.showGrid = !config.value.showGrid
    saveConfig()
    applyTheme()
  }

  function applyTheme() {
    const root = document.documentElement
    const c = config.value

    // 主题颜色
    const themes: Record<ThemeMode, { bg: string; text: string; surface: string; border: string; accent: string; accentHover: string }> = {
      light: { bg: '#ffffff', text: '#000000', surface: '#f5f5f5', border: '#e5e5e5', accent: '#333333', accentHover: '#000000' },
      dark: { bg: '#1a1a1a', text: '#d4d4d4', surface: '#252525', border: '#3a3a3a', accent: '#8b7355', accentHover: '#6b5335' },
      sepia: { bg: '#f4ecd8', text: '#5b4636', surface: '#faf3e0', border: '#d4c5a9', accent: '#8b7355', accentHover: '#6b5335' },
    }

    const theme = themes[c.mode]
    root.style.setProperty('--bg-color', theme.bg)
    root.style.setProperty('--text-color', theme.text)
    root.style.setProperty('--surface-color', theme.surface)
    root.style.setProperty('--border-color', theme.border)
    root.style.setProperty('--accent-color', theme.accent)
    root.style.setProperty('--accent-hover', theme.accentHover)
    root.style.setProperty('--font-size', `${c.fontSize}px`)
    root.style.setProperty('--line-height', String(c.lineHeight))
    root.style.setProperty('--letter-spacing', `${c.letterSpacing}px`)
    root.style.setProperty('--margin-left', `${c.marginLeft}px`)
    root.style.setProperty('--margin-right', `${c.marginRight}px`)
    root.style.setProperty('--grid-line-height', `${gridLineHeight.value}px`)
  }

  return {
    config,
    themeMode,
    fontSize,
    lineHeight,
    gridLineHeight,
    setTheme,
    setFontSize,
    setFontFamily,
    setLineHeight,
    setLetterSpacing,
    setMargins,
    toggleGrid,
    applyTheme,
  }
})