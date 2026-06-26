import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface ShortcutMap {
  save: string
  addAnnotation: string
  export: string
  focusMode: string
  newChapter: string
}

const defaultShortcuts: ShortcutMap = {
  save: 'Ctrl+S',
  addAnnotation: 'Ctrl+Shift+A',
  export: 'Ctrl+Shift+E',
  focusMode: 'F11',
  newChapter: 'Ctrl+N',
}

export const useShortcutStore = defineStore('shortcut', () => {
  const shortcuts = ref<ShortcutMap>({ ...defaultShortcuts })

  // Load from localStorage
  const saved = localStorage.getItem('app-shortcuts')
  if (saved) {
    try {
      const parsed = JSON.parse(saved)
      shortcuts.value = { ...defaultShortcuts, ...parsed }
    } catch (e) {
      console.error('Failed to parse shortcuts', e)
    }
  }

  function saveShortcuts() {
    localStorage.setItem('app-shortcuts', JSON.stringify(shortcuts.value))
  }

  function updateShortcut(key: keyof ShortcutMap, value: string) {
    shortcuts.value[key] = value
    saveShortcuts()
  }

  function resetShortcuts() {
    shortcuts.value = { ...defaultShortcuts }
    saveShortcuts()
  }

  /**
   * Helper function to check if a KeyboardEvent matches a registered shortcut.
   * e.g. matchShortcut(e, shortcuts.value.save)
   */
  function match(e: KeyboardEvent, shortcut: string): boolean {
    const keys = shortcut.split('+').map(k => k.trim().toLowerCase())
    
    const ctrlOrMeta = keys.includes('ctrl') || keys.includes('meta') || keys.includes('cmd')
    const shift = keys.includes('shift')
    const alt = keys.includes('alt')
    
    // Find the main key (not a modifier)
    const mainKey = keys.find(k => !['ctrl', 'meta', 'cmd', 'shift', 'alt'].includes(k))

    if (!mainKey) return false

    // Check modifiers
    if (ctrlOrMeta !== (e.ctrlKey || e.metaKey)) return false
    if (shift !== e.shiftKey) return false
    if (alt !== e.altKey) return false
    
    // Check main key
    return e.key.toLowerCase() === mainKey
  }

  return {
    shortcuts,
    updateShortcut,
    resetShortcuts,
    match
  }
})
