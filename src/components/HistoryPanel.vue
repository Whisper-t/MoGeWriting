<script setup lang="ts">
import { ref, watch, computed } from "vue";

import { useEditorStore } from "@/stores/editor";

import { useWorkspaceStore } from "@/stores/workspace";

import * as api from "@/composables/useTauriCommands";

import type { HistorySnapshot } from "@/types";

defineEmits<{
  close: [];
}>();

const editor = useEditorStore();

const workspace = useWorkspaceStore();

const snapshots = ref<HistorySnapshot[]>([]);

const loading = ref(false);

const previewContent = ref("");

const previewingSnapshot = ref<HistorySnapshot | null>(null);

const restoring = ref(false);

const statusMsg = ref("");

// 当前文件路径

const currentFilePath = computed(() => editor.currentFilePath);

// 加载历史列表

async function loadHistory() {
  if (!currentFilePath.value) return;

  loading.value = true;

  try {
    snapshots.value = await api.listHistory(currentFilePath.value);
  } catch (e) {
    console.error("Failed to load history:", e);

    snapshots.value = [];
  } finally {
    loading.value = false;
  }
}

// 预览快照
async function previewSnapshot(snap: HistorySnapshot) {
  previewingSnapshot.value = snap;
  try {
    previewContent.value = await api.readHistorySnapshot(snap.path);
  } catch (e) {
    previewContent.value = `✗ 加载失败：${e}`;
  }
}

// 恢复快照
async function restoreSnapshot(snap: HistorySnapshot) {
  if (!currentFilePath.value) return;
  restoring.value = true;
  statusMsg.value = "";
  try {
    await editor.restoreFromSnapshot(snap.path);
    statusMsg.value = "✓ 已恢复到该版本";
    previewingSnapshot.value = null;
    previewContent.value = "";
    await loadHistory();
  } catch (e) {
    statusMsg.value = `✗ 恢复失败：${e}`;
  } finally {
    restoring.value = false;
  }
}

function closePreview() {
  previewingSnapshot.value = null;

  previewContent.value = "";
}

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;

  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;

  return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
}

function formatTimeAgo(timestamp: string): string {
  const t = new Date(timestamp.replace(" ", "T"));

  const now = new Date();

  const diffMs = now.getTime() - t.getTime();

  const diffMin = Math.floor(diffMs / 60000);

  if (diffMin < 1) return "刚刚";

  if (diffMin < 60) return `${diffMin} 分钟前`;

  const diffHour = Math.floor(diffMin / 60);

  if (diffHour < 24) return `${diffHour} 小时前`;

  const diffDay = Math.floor(diffHour / 24);

  if (diffDay < 30) return `${diffDay} 天前`;

  return timestamp;
}

// 初始加载

watch(
  currentFilePath,
  () => {
    loadHistory();

    closePreview();
  },
  { immediate: true },
);
</script>

<template>
  <div class="history-panel-overlay" @click.self="$emit('close')">
    <div class="history-panel">
      <div class="panel-header">
        <h2>📜 历史版本</h2>

        <span class="chapter-name">{{
          workspace.selectedChapter?.title || ""
        }}</span>

        <button class="close-btn" @click="$emit('close')">✕</button>
      </div>

      <div class="panel-body">
        <!-- 左侧：快照列表 -->

        <div class="snapshot-list">
          <div v-if="loading" class="loading-state">加载中…</div>

          <div v-else-if="snapshots.length === 0" class="empty-state">
            <p class="empty-icon">🕐</p>

            <p class="empty-text">暂无历史版本</p>

            <p class="empty-hint">每次保存都会自动生成历史快照</p>
          </div>

          <div
            v-for="snap in snapshots"
            :key="snap.filename"
            class="snapshot-card"
            :class="{ active: previewingSnapshot?.filename === snap.filename }"
            @click="previewSnapshot(snap)"
          >
            <div class="snap-time">{{ snap.timestamp }}</div>

            <div class="snap-meta">
              <span class="snap-ago">{{ formatTimeAgo(snap.timestamp) }}</span>

              <span class="snap-size">{{ formatSize(snap.size) }}</span>
            </div>
          </div>
        </div>

        <!-- 右侧：预览区 -->

        <div class="preview-area">
          <div v-if="!previewingSnapshot" class="preview-empty">
            <p>👈 选择一个历史版本进行预览</p>
          </div>

          <template v-else>
            <div class="preview-header">
              <span class="preview-time">{{
                previewingSnapshot.timestamp
              }}</span>

              <button
                class="btn-restore"
                :disabled="restoring"
                @click="restoreSnapshot(previewingSnapshot!)"
              >
                {{ restoring ? "恢复中…" : "↻ 恢复此版本" }}
              </button>
            </div>

            <pre class="preview-content">{{ previewContent }}</pre>
          </template>

          <div
            v-if="statusMsg"
            class="status-msg"
            :class="{ error: statusMsg.startsWith('✗') }"
          >
            {{ statusMsg }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.history-panel-overlay {
  position: fixed;

  inset: 0;

  background: rgba(0, 0, 0, 0.35);

  display: flex;

  align-items: center;

  justify-content: center;

  z-index: 400;
}

.history-panel {
  background: var(--surface-color);

  border: 1px solid var(--border-color);

  border-radius: 12px;

  width: 800px;

  max-width: 92vw;

  height: 560px;

  max-height: 85vh;

  display: flex;

  flex-direction: column;

  overflow: hidden;
}

.panel-header {
  display: flex;

  align-items: center;

  gap: 12px;

  padding: 16px 20px;

  border-bottom: 1px solid var(--border-color);

  flex-shrink: 0;
}

.panel-header h2 {
  font-size: 16px;

  font-weight: 600;

  color: var(--text-color);

  white-space: nowrap;
}

.chapter-name {
  font-size: 13px;

  color: var(--text-color);

  opacity: 0.5;

  flex: 1;

  overflow: hidden;

  text-overflow: ellipsis;

  white-space: nowrap;
}

.close-btn {
  width: 28px;

  height: 28px;

  display: flex;

  align-items: center;

  justify-content: center;

  border-radius: 4px;

  font-size: 14px;

  color: var(--text-color);

  flex-shrink: 0;

  transition: background 0.15s;
}

.close-btn:hover {
  background: var(--border-color);
}

.panel-body {
  flex: 1;

  display: flex;

  overflow: hidden;
}

/* 左侧快照列表 */

.snapshot-list {
  width: 260px;

  flex-shrink: 0;

  border-right: 1px solid var(--border-color);

  overflow-y: auto;

  padding: 12px;
}

.loading-state,
.empty-state {
  text-align: center;

  padding: 40px 16px;

  color: var(--text-color);

  opacity: 0.5;
}

.empty-icon {
  font-size: 32px;

  margin-bottom: 8px;
}

.empty-text {
  font-size: 14px;

  margin-bottom: 6px;
}

.empty-hint {
  font-size: 12px;

  opacity: 0.6;
}

.snapshot-card {
  padding: 10px 12px;

  border: 1px solid var(--border-color);

  border-radius: 6px;

  margin-bottom: 6px;

  cursor: pointer;

  transition: all 0.15s;
}

.snapshot-card:hover {
  border-color: var(--accent-color);

  background: color-mix(in srgb, var(--accent-color) 3%, transparent);
}

.snapshot-card.active {
  border-color: var(--accent-color);

  background: color-mix(in srgb, var(--accent-color) 8%, transparent);
}

.snap-time {
  font-size: 13px;

  font-weight: 500;

  color: var(--text-color);

  margin-bottom: 4px;
}

.snap-meta {
  display: flex;

  justify-content: space-between;

  font-size: 11px;

  color: var(--text-color);

  opacity: 0.4;
}

/* 右侧预览 */

.preview-area {
  flex: 1;

  display: flex;

  flex-direction: column;

  overflow: hidden;
}

.preview-empty {
  height: 100%;

  display: flex;

  align-items: center;

  justify-content: center;

  color: var(--text-color);

  opacity: 0.35;

  font-size: 14px;
}

.preview-header {
  display: flex;

  align-items: center;

  justify-content: space-between;

  padding: 10px 16px;

  border-bottom: 1px solid var(--border-color);

  flex-shrink: 0;
}

.preview-time {
  font-size: 13px;

  font-weight: 500;

  color: var(--text-color);
}

.btn-restore {
  padding: 5px 14px;

  background: var(--accent-color);

  color: #fff;

  border-radius: 4px;

  font-size: 12px;

  transition: background 0.15s;
}

.btn-restore:hover:not(:disabled) {
  background: var(--accent-hover);
}

.btn-restore:disabled {
  opacity: 0.5;

  cursor: default;
}

.preview-content {
  flex: 1;

  overflow-y: auto;

  padding: 16px;

  font-size: 13px;

  line-height: 1.8;

  color: var(--text-color);

  font-family: "Noto Serif SC", "Source Han Serif SC", "SimSun", "宋体", serif;

  white-space: pre-wrap;

  word-break: break-all;

  background: var(--bg-color);
}

.status-msg {
  padding: 8px 16px;

  font-size: 12px;

  color: var(--accent-color);

  background: color-mix(in srgb, var(--accent-color) 6%, transparent);

  flex-shrink: 0;
}

.status-msg.error {
  color: var(--danger-color);

  background: rgba(192, 57, 43, 0.06);
}
</style>
