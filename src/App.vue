<script setup lang="ts">
import { onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useThemeStore } from '@/stores/theme'
import { useWorkspaceStore } from '@/stores/workspace'

const theme = useThemeStore()
const workspace = useWorkspaceStore()

onMounted(async () => {
  theme.applyTheme()
  try {
    const homeDir = await invoke<string>('get_home_dir')
    await workspace.init(`${homeDir}\\MoGeWriting`)
  } catch {
    await workspace.init('C:\\Users\\Default\\MoGeWriting')
  }
})
</script>

<template>
  <router-view v-slot="{ Component }">
    <transition name="page-fade" mode="out-in">
      <component :is="Component" />
    </transition>
  </router-view>
</template>

<style>
#app {
  width: 100%;
  height: 100%;
}

.page-fade-enter-active,
.page-fade-leave-active {
  transition: opacity 0.2s ease;
}
.page-fade-enter-from,
.page-fade-leave-to {
  opacity: 0;
}
</style>