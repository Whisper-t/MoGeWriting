<script setup lang="ts">

import { computed } from 'vue'

import { useThemeStore } from '@/stores/theme'



defineEmits<{ close: [] }>()



const theme = useThemeStore()



const gridLineHeightPx = computed(() => {

  return (theme.config.fontSize * theme.config.lineHeight).toFixed(1)

})



const fontFamilies = [

  { label: '宋体', value: '"SimSun", "宋体", serif' },

  { label: '黑体', value: '"SimHei", "黑体", sans-serif' },

  { label: '楷体', value: '"KaiTi", "楷体", serif' },

  { label: '思源宋体', value: '"Noto Serif SC", "Source Han Serif SC", serif' },

  { label: '思源黑体', value: '"Noto Sans SC", "Source Han Sans SC", sans-serif' },

  { label: '微软雅黑', value: '"Microsoft YaHei", sans-serif' },

]

</script>



<template>

  <div class="theme-settings">

    <div class="settings-header">

      <span>排版设置</span>

      <button class="close-btn" @click="$emit('close')"></button>

    </div>



    <div class="settings-body">

      <!-- 主题模式 -->

      <div class="setting-group">

        <label>主题</label>

        <div class="theme-toggles">

          <button

            :class="{ active: theme.config.mode === 'light' }"

            @click="theme.setTheme('light')"

          >浅色</button>

          <button

            :class="{ active: theme.config.mode === 'dark' }"

            @click="theme.setTheme('dark')"

          >深色</button>

          <button

            :class="{ active: theme.config.mode === 'sepia' }"

            @click="theme.setTheme('sepia')"

          >护眼</button>

        </div>

      </div>



      <!-- 字体 -->

      <div class="setting-group">

        <label>字体</label>

        <select

          :value="theme.config.fontFamily"

          @change="theme.setFontFamily(($event.target as HTMLSelectElement).value)"

        >

          <option v-for="f in fontFamilies" :key="f.value" :value="f.value">

            {{ f.label }}

          </option>

        </select>

      </div>



      <!-- 字号 -->

      <div class="setting-group">

        <label>字号：{{ theme.config.fontSize }}px</label>

        <input

          type="range"

          :value="theme.config.fontSize"

          @input="theme.setFontSize(Number(($event.target as HTMLInputElement).value))"

          min="12"

          max="28"

          step="1"

        />

        <div class="range-labels">

          <span>12</span>

          <span>28</span>

        </div>

      </div>



      <!-- 行距 -->

      <div class="setting-group">

        <label>行距：{{ theme.config.lineHeight.toFixed(1) }}</label>

        <input

          type="range"

          :value="theme.config.lineHeight"

          @input="theme.setLineHeight(Number(($event.target as HTMLInputElement).value))"

          min="1.2"

          max="3.0"

          step="0.1"

        />

        <div class="range-labels">

          <span>1.2</span>

          <span>3.0</span>

        </div>

      </div>



      <!-- 字间?-->

      <div class="setting-group">

        <label>字间距：{{ theme.config.letterSpacing.toFixed(1) }}px</label>

        <input

          type="range"

          :value="theme.config.letterSpacing"

          @input="theme.setLetterSpacing(Number(($event.target as HTMLInputElement).value))"

          min="0"

          max="5"

          step="0.5"

        />

        <div class="range-labels">

          <span>0</span>

          <span>5</span>

        </div>

      </div>



      <!-- 左右边距 -->

      <div class="setting-group">

        <label>左右边距：{{ theme.config.marginLeft }}px</label>

        <input

          type="range"

          :value="theme.config.marginLeft"

          @input="theme.setMargins(

            Number(($event.target as HTMLInputElement).value),

            Number(($event.target as HTMLInputElement).value)

          )"

          min="20"

          max="200"

          step="10"

        />

        <div class="range-labels">

          <span></span>

          <span></span>

        </div>

      </div>



      <!-- 稿纸网格?-->

      <div class="setting-group">

        <div class="grid-toggle-row">

          <label class="toggle-label">

            <input

              type="checkbox"

              :checked="theme.config.showGrid"

              @change="theme.toggleGrid()"

            />

            <span>稿纸网格</span>

          </label>

          <span v-if="theme.config.showGrid" class="grid-info">

            线高 {{ gridLineHeightPx }}px

          </span>

        </div>

      </div>

    </div>

  </div>

</template>



<style scoped>

.theme-settings {

  border-bottom: 1px solid var(--border-color);

  background: var(--surface-color);

  flex-shrink: 0;

  user-select: none;

}



.settings-header {

  display: flex;

  align-items: center;

  justify-content: space-between;

  padding: 8px 16px;

  font-size: 13px;

  font-weight: 600;

  color: var(--text-color);

}



.close-btn {

  width: 24px;

  height: 24px;

  display: flex;

  align-items: center;

  justify-content: center;

  border-radius: 4px;

  font-size: 12px;

  transition: background 0.15s;

}

.close-btn:hover {

  background: var(--border-color);

}



.settings-body {

  padding: 0 16px 16px;

  display: flex;

  flex-wrap: wrap;

  gap: 14px 20px;

}



.setting-group {

  min-width: 170px;

}



.setting-group label {

  display: block;

  font-size: 11px;

  color: var(--text-color);

  opacity: 0.6;

  margin-bottom: 6px;

}



.setting-group input[type="range"] {

  width: 100%;

  height: 4px;

  -webkit-appearance: none;

  appearance: none;

  background: var(--border-color);

  border-radius: 2px;

  outline: none;

  cursor: pointer;

}

.setting-group input[type="range"]::-webkit-slider-thumb {

  -webkit-appearance: none;

  width: 14px;

  height: 14px;

  border-radius: 50%;

  background: var(--accent-color);

  cursor: pointer;

}



.range-labels {

  display: flex;

  justify-content: space-between;

  font-size: 10px;

  color: var(--text-color);

  opacity: 0.35;

  margin-top: 2px;

}



.setting-group select {

  width: 100%;

  padding: 4px 8px;

  border: 1px solid var(--border-color);

  border-radius: 4px;

  background: var(--bg-color);

  color: var(--text-color);

  font-size: 12px;

  cursor: pointer;

}



.theme-toggles {

  display: flex;

  gap: 4px;

}

.theme-toggles button {

  flex: 1;

  padding: 4px 8px;

  border: 1px solid var(--border-color);

  border-radius: 4px;

  font-size: 12px;

  transition: all 0.15s;

}

.theme-toggles button.active {

  border-color: var(--accent-color);

  color: var(--accent-color);

  background: color-mix(in srgb, var(--accent-color) 8%, transparent);

}



.grid-toggle-row {

  display: flex;

  align-items: center;

  justify-content: space-between;

  gap: 8px;

}



.toggle-label {

  display: flex !important;

  align-items: center;

  gap: 8px;

  cursor: pointer;

  margin-bottom: 0 !important;

  opacity: 1 !important;

}

.toggle-label input[type="checkbox"] {

  width: auto;

  accent-color: var(--accent-color);

}



.grid-info {

  font-size: 10px;

  color: var(--accent-color);

  opacity: 0.7;

  white-space: nowrap;

}

</style>