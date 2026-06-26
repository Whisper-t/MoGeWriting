use std::fs;
use std::path::Path;
use tauri::command;
use crate::models::*;

/// 获取用户主目录
#[command]
pub fn get_home_dir() -> Result<String, String> {
    dirs::home_dir()
        .map(|p| p.to_string_lossy().to_string())
        .ok_or_else(|| "无法获取用户主目录".to_string())
}

/// 扫描一个根目录，列出所有有效的书籍/随笔集工作区
#[command]
pub fn list_workspaces(root_path: String) -> Result<Vec<WorkspaceSummary>, String> {
    let root = Path::new(&root_path);
    if !root.exists() {
        fs::create_dir_all(root).map_err(|e| e.to_string())?;
        return Ok(vec![]);
    }

    let mut workspaces = Vec::new();

    for entry in fs::read_dir(root).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            let book_json_path = path.join("book.json");
            if book_json_path.exists() {
                if let Ok(content) = fs::read_to_string(&book_json_path) {
                    if let Ok(book) = serde_json::from_str::<BookIndex>(&content) {
                        let total_chapters = match &book.volumes {
                            Some(vols) => vols.iter().map(|v| v.chapters.len()).sum(),
                            None => book.chapters.as_ref().map(|c| c.len()).unwrap_or(0),
                        };
                        let total_words = count_words_in_book(&path, &book);
                        workspaces.push(WorkspaceSummary {
                            path: path.to_string_lossy().to_string(),
                            name: book.name,
                            work_type: book.work_type,
                            description: book.description.clone(),
                            cover_image: book.cover_image.clone(),
                            total_chapters,
                            total_words,
                            modified_at: book.modified_at,
                        });
                    }
                }
            }
        }
    }

    // 按修改时间降序排列
    workspaces.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
    Ok(workspaces)
}

/// 创建新工作区
#[command]
pub fn create_workspace(
    root_path: String,
    name: String,
    work_type: String,
    structure_type: String,
    description: Option<String>,
    author: Option<String>,
) -> Result<WorkspaceSummary, String> {
    // 生成安全的目录名
    let dir_name = sanitize_dirname(&name);
    let work_path = Path::new(&root_path).join(&dir_name);

    if work_path.exists() {
        return Err(format!("目录已存在: {}", work_path.display()));
    }

    fs::create_dir_all(&work_path).map_err(|e| e.to_string())?;

    let is_flat_type = structure_type == "flat";
    
    let mut book = if is_flat_type {
        BookIndex::new_essay(&name, description.as_deref(), author.as_deref())
    } else {
        BookIndex::new_novel(&name, description.as_deref(), author.as_deref())
    };
    book.work_type = work_type.clone();

    let book_json = serde_json::to_string_pretty(&book).map_err(|e| e.to_string())?;
    fs::write(work_path.join("book.json"), &book_json).map_err(|e| e.to_string())?;

    Ok(WorkspaceSummary {
        path: work_path.to_string_lossy().to_string(),
        name: book.name,
        work_type: book.work_type,
        description: book.description,
        cover_image: book.cover_image,
        total_chapters: 0,
        total_words: 0,
        modified_at: book.modified_at,
    })
}

/// 读取 book.json
#[command]
pub fn read_book_index(book_path: String) -> Result<BookIndex, String> {
    let json_path = Path::new(&book_path).join("book.json");
    let content = fs::read_to_string(&json_path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

/// 写入 book.json
#[command]
pub fn write_book_index(book_path: String, book: BookIndex) -> Result<(), String> {
    let mut book = book;
    book.modified_at = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let json_path = Path::new(&book_path).join("book.json");
    let content = serde_json::to_string_pretty(&book).map_err(|e| e.to_string())?;
    fs::write(&json_path, &content).map_err(|e| e.to_string())
}

/// 创建新章节（小说模式）
#[command]
pub fn create_chapter(request: CreateChapterRequest) -> Result<BookIndex, String> {
    let book_path = Path::new(&request.book_path);
    let mut book = read_book_index_internal(book_path)?;

    let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let chapter_id = uuid::Uuid::new_v4().to_string();
    let filename = sanitize_filename(&request.title);
    let file_path = format!("{}.md", filename);

    let chapter = ChapterEntry {
        id: chapter_id,
        title: request.title.clone(),
        file: file_path.clone(),
        order: 0,
        word_count: Some(0),
        created_at: Some(now.clone()),
        modified_at: Some(now),
    };

    // 创建空的 .md 文件
    let md_path = book_path.join(&file_path);
    let initial_content = String::new();
    fs::write(&md_path, initial_content).map_err(|e| e.to_string())?;

    match &mut book.volumes {
        Some(volumes) => {
            if let Some(vol_id) = &request.volume_id {
                if let Some(vol) = volumes.iter_mut().find(|v| &v.id == vol_id) {
                    let order = vol.chapters.len() as u32;
                    let mut chapter = chapter;
                    chapter.order = order;
                    vol.chapters.push(chapter);
                } else {
                    // 卷不存在则删除文件
                    let _ = fs::remove_file(&md_path);
                    return Err("指定的卷不存在".to_string());
                }
            } else {
                // 没有指定卷，创建到默认卷
                if volumes.is_empty() {
                    let vol_id = uuid::Uuid::new_v4().to_string();
                    volumes.push(VolumeEntry {
                        id: vol_id,
                        title: "默认卷".to_string(),
                        order: 0,
                        chapters: vec![chapter],
                    });
                } else {
                    let order = volumes[0].chapters.len() as u32;
                    let mut chapter = chapter;
                    chapter.order = order;
                    volumes[0].chapters.push(chapter);
                }
            }
        }
        None => {
            // 随笔模式
            let chapters = book.chapters.get_or_insert(vec![]);
            let order = chapters.len() as u32;
            let mut chapter = chapter;
            chapter.order = order;
            chapters.push(chapter);
        }
    }

    write_book_index_internal(book_path, &mut book)?;
    Ok(book)
}

/// 创建新分卷
#[command]
pub fn create_volume(request: CreateVolumeRequest) -> Result<BookIndex, String> {
    let book_path = Path::new(&request.book_path);
    let mut book = read_book_index_internal(book_path)?;

    let vol_id = uuid::Uuid::new_v4().to_string();
    let volumes = book.volumes.get_or_insert(vec![]);
    let order = volumes.len() as u32;

    volumes.push(VolumeEntry {
        id: vol_id,
        title: request.title,
        order,
        chapters: vec![],
    });

    write_book_index_internal(book_path, &mut book)?;
    Ok(book)
}

/// 重命名章节或分卷
#[command]
pub fn rename_entry(request: RenameRequest) -> Result<BookIndex, String> {
    let book_path = Path::new(&request.book_path);
    let mut book = read_book_index_internal(book_path)?;

    // 重命名文件系统中的文件
    let old_path = book_path.join(&request.old_path);
    let parent = old_path.parent().unwrap_or(book_path);
    let new_filename = format!("{}.md", sanitize_filename(&request.new_name));
    let new_path = parent.join(&new_filename);

    if old_path.exists() {
        fs::rename(&old_path, &new_path).map_err(|e| e.to_string())?;
    }

    // 更新 book.json 中的记录
    if let Some(volumes) = &mut book.volumes {
        for vol in volumes.iter_mut() {
            for ch in vol.chapters.iter_mut() {
                if ch.file == request.old_path {
                    ch.title = request.new_name.clone();
                    ch.file = new_filename.clone();
                }
            }
        }
    }
    if let Some(chapters) = &mut book.chapters {
        for ch in chapters.iter_mut() {
            if ch.file == request.old_path {
                ch.title = request.new_name.clone();
                ch.file = new_filename.clone();
            }
        }
    }

    write_book_index_internal(book_path, &mut book)?;
    Ok(book)
}

/// 删除章节（软删除：移动到 .trash 回收站）
#[command]
pub fn delete_chapter(book_path: String, chapter_id: String) -> Result<BookIndex, String> {
    let book_path = Path::new(&book_path);
    let mut book = read_book_index_internal(book_path)?;

    // 查找并软删除文件
    let file_to_delete = if let Some(volumes) = &book.volumes {
        volumes.iter().flat_map(|v| &v.chapters).find(|c| c.id == chapter_id).map(|c| c.file.clone())
    } else {
        book.chapters.as_ref().and_then(|ch| ch.iter().find(|c| c.id == chapter_id).map(|c| c.file.clone()))
    };

    if let Some(file) = file_to_delete {
        let md_path = book_path.join(&file);
        if md_path.exists() {
            move_to_trash(&md_path, book_path)?;
        }
    }

    // 从 book.json 中移除
    if let Some(volumes) = &mut book.volumes {
        for vol in volumes.iter_mut() {
            vol.chapters.retain(|c| c.id != chapter_id);
            // 重新排序
            for (i, ch) in vol.chapters.iter_mut().enumerate() {
                ch.order = i as u32;
            }
        }
    }
    if let Some(chapters) = &mut book.chapters {
        chapters.retain(|c| c.id != chapter_id);
        for (i, ch) in chapters.iter_mut().enumerate() {
            ch.order = i as u32;
        }
    }

    write_book_index_internal(book_path, &mut book)?;
    Ok(book)
}

/// 删除分卷（软删除：章节文件移动到 .trash）
#[command]
pub fn delete_volume(book_path: String, volume_id: String) -> Result<BookIndex, String> {
    let book_path = Path::new(&book_path);
    let mut book = read_book_index_internal(book_path)?;

    if let Some(volumes) = &mut book.volumes {
        // 软删除卷内所有章节文件
        if let Some(vol) = volumes.iter().find(|v| v.id == volume_id) {
            for ch in &vol.chapters {
                let md_path = book_path.join(&ch.file);
                if md_path.exists() {
                    let _ = move_to_trash(&md_path, book_path);
                }
            }
        }
        volumes.retain(|v| v.id != volume_id);
        for (i, vol) in volumes.iter_mut().enumerate() {
            vol.order = i as u32;
        }
    }

    write_book_index_internal(book_path, &mut book)?;
    Ok(book)
}

/// 重新排序（拖拽后调用）
#[command]
pub fn reorder(request: ReorderRequest) -> Result<BookIndex, String> {
    let book_path = Path::new(&request.book_path);
    let mut book = read_book_index_internal(book_path)?;

    if let Some(volumes) = request.volumes {
        book.volumes = Some(volumes);
    }
    if let Some(chapters) = request.chapters {
        book.chapters = Some(chapters);
    }

    write_book_index_internal(book_path, &mut book)?;
    Ok(book)
}

/// 列出目录内容
#[command]
pub fn list_directory(path: String) -> Result<Vec<DirEntry>, String> {
    let dir = Path::new(&path);
    if !dir.exists() {
        return Ok(vec![]);
    }

    let mut entries = Vec::new();
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let metadata = entry.metadata().map_err(|e| e.to_string())?;

        // 跳过隐藏文件/目录
        if entry.file_name().to_string_lossy().starts_with('.') {
            continue;
        }

        entries.push(DirEntry {
            name: entry.file_name().to_string_lossy().to_string(),
            path: path.to_string_lossy().to_string(),
            is_dir: metadata.is_dir(),
            size: if metadata.is_file() { Some(metadata.len()) } else { None },
            modified_at: metadata.modified().ok().map(|t| {
                chrono::DateTime::<chrono::Local>::from(t)
                    .format("%Y-%m-%dT%H:%M:%S")
                    .to_string()
            }),
        });
    }

    entries.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir)
        } else {
            a.name.cmp(&b.name)
        }
    });

    Ok(entries)
}

/// 删除工作区（软删除：移动到 .trash 回收站）
#[command]
pub fn delete_workspace(path: String) -> Result<(), String> {
    let work_path = Path::new(&path);
    if !work_path.exists() {
        return Err("工作区不存在".to_string());
    }
    // 软删除：移动到父目录下的 .trash
    let parent = work_path.parent().unwrap_or(Path::new("."));
    move_to_trash(work_path, parent)
}

// ========== 内部辅助函数 ==========

fn read_book_index_internal(book_path: &Path) -> Result<BookIndex, String> {
    let json_path = book_path.join("book.json");
    let content = fs::read_to_string(&json_path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

fn write_book_index_internal(book_path: &Path, book: &mut BookIndex) -> Result<(), String> {
    book.modified_at = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let json_path = book_path.join("book.json");
    let content = serde_json::to_string_pretty(book).map_err(|e| e.to_string())?;
    fs::write(&json_path, &content).map_err(|e| e.to_string())
}

/// 软删除：将文件/目录移动到 .trash 回收站（带时间戳防重名）
fn move_to_trash(source: &Path, root: &Path) -> Result<(), String> {
    let trash_dir = root.join(".trash");
    fs::create_dir_all(&trash_dir).map_err(|e| e.to_string())?;

    let name = source.file_name().unwrap_or_default().to_string_lossy();
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let trash_name = format!("{}_{}", name, timestamp);
    let dest = trash_dir.join(&trash_name);

    fs::rename(source, &dest).map_err(|e| e.to_string())
}

fn sanitize_dirname(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect()
}

fn sanitize_filename(name: &str) -> String {
    sanitize_dirname(name)
}

fn count_words_in_book(book_path: &Path, book: &BookIndex) -> usize {
    let mut total = 0;
    let chapters: Vec<&ChapterEntry> = match &book.volumes {
        Some(vols) => vols.iter().flat_map(|v| &v.chapters).collect(),
        None => book.chapters.as_ref().map(|c| c.iter().collect()).unwrap_or_default(),
    };
    for ch in chapters {
        let md_path = book_path.join(&ch.file);
        if let Ok(content) = fs::read_to_string(&md_path) {
            total += count_chinese_words(&content);
        }
    }
    total
}

fn count_chinese_words(text: &str) -> usize {
    // 统计中文字符数 + 英文单词数
    let chinese_chars = text.chars().filter(|c| c >= &'\u{4e00}' && c <= &'\u{9fff}').count();
    let english_words = text
        .split(|c: char| c.is_whitespace() || (c >= '\u{4e00}' && c <= '\u{9fff}'))
        .filter(|s| !s.is_empty())
        .count();
    chinese_chars + english_words
}