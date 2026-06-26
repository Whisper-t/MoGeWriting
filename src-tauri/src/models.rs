use serde::{Deserialize, Serialize};

/// 书籍/随笔集的类型


/// 单章信息的索引条目
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChapterEntry {
    pub id: String,
    pub title: String,
    pub file: String,
    pub order: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
}

/// 分卷信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VolumeEntry {
    pub id: String,
    pub title: String,
    pub order: u32,
    pub chapters: Vec<ChapterEntry>,
}

/// book.json 的顶层结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BookIndex {
    pub name: String,
    #[serde(rename = "type")]
    pub work_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_numbering: Option<bool>,
    pub created_at: String,
    pub modified_at: String,
    /// 小说模式：分卷 + 章节
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<VolumeEntry>>,
    /// 随笔模式：平铺章节
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapters: Option<Vec<ChapterEntry>>,
}

impl BookIndex {
    /// 创建一本新小说
    pub fn new_novel(name: &str, description: Option<&str>, author: Option<&str>) -> Self {
        let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        Self {
            name: name.to_string(),
            work_type: "novel".to_string(),
            description: description.map(|s| s.to_string()),
            author: author.map(|s| s.to_string()),
            cover_image: None,
            auto_numbering: Some(false),
            created_at: now.clone(),
            modified_at: now,
            volumes: Some(vec![]),
            chapters: None,
        }
    }

    /// 创建一本新随笔集
    pub fn new_essay(name: &str, description: Option<&str>, author: Option<&str>) -> Self {
        let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        Self {
            name: name.to_string(),
            work_type: "essay".to_string(),
            description: description.map(|s| s.to_string()),
            author: author.map(|s| s.to_string()),
            cover_image: None,
            auto_numbering: Some(false),
            created_at: now.clone(),
            modified_at: now,
            volumes: None,
            chapters: Some(vec![]),
        }
    }
}

/// 文件/目录信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DirEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
}

/// 工作区概览（首页展示用）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkspaceSummary {
    pub path: String,
    pub name: String,
    pub work_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<String>,
    pub total_chapters: usize,
    pub total_words: usize,
    pub modified_at: String,
}

/// 保存文件的请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SaveRequest {
    pub path: String,
    pub content: String,
}

/// 创建章节的请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateChapterRequest {
    pub book_path: String,
    pub title: String,
    /// 如果是小说模式，需要指定 volume_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
}

/// 创建分卷的请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateVolumeRequest {
    pub book_path: String,
    pub title: String,
}

/// 重命名请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RenameRequest {
    pub book_path: String,
    pub old_path: String,
    pub new_name: String,
}

/// 重新排序请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReorderRequest {
    pub book_path: String,
    /// 新的 volumes 列表（完整替换）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<VolumeEntry>>,
    /// 新的 chapters 列表（随笔模式）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapters: Option<Vec<ChapterEntry>>,
}

/// 历史快照信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistorySnapshot {
    pub filename: String,
    pub path: String,
    pub timestamp: String,
    pub size: u64,
}

/// 导出请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExportRequest {
    pub book_path: String,
    pub output_path: String,
    /// "txt" 或 "pdf" 或 "zip"
    pub format: String,
}

/// 恢复请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestoreRequest {
    pub zip_path: String,
    pub target_path: String,
}