export type WorkType = string

export interface ChapterEntry {
  id: string
  title: string
  file: string
  order: number
  word_count?: number
  created_at?: string
  modified_at?: string
}

export interface VolumeEntry {
  id: string
  title: string
  order: number
  chapters: ChapterEntry[]
}

export interface BookIndex {
  name: string
  type: WorkType
  description?: string
  author?: string
  cover_image?: string
  auto_numbering?: boolean
  created_at: string
  modified_at: string
  volumes?: VolumeEntry[]
  chapters?: ChapterEntry[]
}

export interface DirEntry {
  name: string
  path: string
  is_dir: boolean
  size?: number
  modified_at?: string
}

export interface WorkspaceSummary {
  path: string
  name: string
  work_type: WorkType
  description?: string
  cover_image?: string
  total_chapters: number
  total_words: number
  modified_at: string
}

export interface SaveRequest {
  path: string
  content: string
}

export interface CreateChapterRequest {
  book_path: string
  title: string
  volume_id?: string
}

export interface CreateVolumeRequest {
  book_path: string
  title: string
}

export interface RenameRequest {
  book_path: string
  old_path: string
  new_name: string
}

export interface ReorderRequest {
  book_path: string
  volumes?: VolumeEntry[]
  chapters?: ChapterEntry[]
}

export interface HistorySnapshot {
  filename: string
  path: string
  timestamp: string
  size: number
}

export interface ExportRequest {
  book_path: string
  output_path: string
  format: string
}

export interface RestoreRequest {
  zip_path: string
  target_path: string
}