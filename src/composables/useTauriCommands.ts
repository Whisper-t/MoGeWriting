import { invoke } from '@tauri-apps/api/core'
import type {
  WorkspaceSummary,
  BookIndex,
  DirEntry,
  SaveRequest,
  CreateChapterRequest,
  CreateVolumeRequest,
  RenameRequest,
  ReorderRequest,
  HistorySnapshot,
  ExportRequest,
  RestoreRequest,
} from '@/types'

// ========== 工作区管理 ==========

export function listWorkspaces(rootPath: string): Promise<WorkspaceSummary[]> {
  return invoke('list_workspaces', { rootPath })
}

export function createWorkspace(
  rootPath: string,
  name: string,
  workType: string,
  structureType: string,
  description?: string,
  author?: string
): Promise<WorkspaceSummary> {
  return invoke('create_workspace', {
    rootPath,
    name,
    workType,
    structureType,
    description: description || null,
    author: author || null,
  })
}

export function readBookIndex(bookPath: string): Promise<BookIndex> {
  return invoke('read_book_index', { bookPath })
}

export function writeBookIndex(bookPath: string, book: BookIndex): Promise<void> {
  return invoke('write_book_index', { bookPath, book })
}

export function createChapter(request: CreateChapterRequest): Promise<BookIndex> {
  return invoke('create_chapter', { request })
}

export function createVolume(request: CreateVolumeRequest): Promise<BookIndex> {
  return invoke('create_volume', { request })
}

export function renameEntry(request: RenameRequest): Promise<BookIndex> {
  return invoke('rename_entry', { request })
}

export function deleteChapter(bookPath: string, chapterId: string): Promise<BookIndex> {
  return invoke('delete_chapter', { bookPath, chapterId })
}

export function deleteVolume(bookPath: string, volumeId: string): Promise<BookIndex> {
  return invoke('delete_volume', { bookPath, volumeId })
}

export function reorder(request: ReorderRequest): Promise<BookIndex> {
  return invoke('reorder', { request })
}

export function listDirectory(path: string): Promise<DirEntry[]> {
  return invoke('list_directory', { path })
}

export function deleteWorkspace(path: string): Promise<void> {
  return invoke('delete_workspace', { path })
}

// ========== 文件 I/O ==========

export function readFile(path: string): Promise<string> {
  return invoke('read_file', { path })
}

export function writeFile(request: SaveRequest): Promise<void> {
  return invoke('write_file', { request })
}

export function writeFiles(requests: SaveRequest[]): Promise<void> {
  return invoke('write_files', { requests })
}

export function getFileInfo(path: string): Promise<DirEntry> {
  return invoke('get_file_info', { path })
}

export function setCoverImage(bookPath: string, imagePath: string): Promise<string> {
  return invoke('set_cover_image', { bookPath, imagePath })
}

// ========== 历史快照 ==========

export function listHistory(filePath: string): Promise<HistorySnapshot[]> {
  return invoke('list_history', { filePath })
}

export function readHistorySnapshot(snapshotPath: string): Promise<string> {
  return invoke('read_history_snapshot', { snapshotPath })
}

export function restoreSnapshot(snapshotPath: string, targetPath: string): Promise<void> {
  return invoke('restore_snapshot', { snapshotPath, targetPath })
}

// ========== 导出与恢复 ==========

export function exportTxt(request: ExportRequest): Promise<string> {
  return invoke('export_txt', { request })
}

export function exportZip(request: ExportRequest): Promise<string> {
  return invoke('export_zip', { request })
}

export function restoreFromZip(request: RestoreRequest): Promise<void> {
  return invoke('restore_from_zip', { request })
}

export function countWords(bookPath: string): Promise<number> {
  return invoke('count_words', { bookPath })
}