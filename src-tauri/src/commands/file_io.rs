use std::fs;
use std::io::Read;
use std::path::Path;
use std::io::Write;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use tauri::command;
use base64::Engine;
use crate::models::*;

/// 在默认浏览器中打开 HTML 文件（用于 PDF 打印）
#[command]
pub fn open_in_browser(path: String) -> Result<(), String> {
    open_path(&path)
}

/// 在资源管理器中打开文件夹
#[command]
pub fn open_in_explorer(path: String) -> Result<(), String> {
    open_path(&path)
}

fn open_path(path: &str) -> Result<(), String> {
    let p = Path::new(path);
    if !p.exists() {
        return Err(format!("路径不存在: {}", path));
    }
    open::that(path).map_err(|e| e.to_string())
}

/// 读取文件并返回 base64 编码（用于图片嵌入 PDF）
#[command]
pub fn read_file_base64(path: String) -> Result<String, String> {
    let file_path = Path::new(&path);
    if !file_path.exists() {
        return Err(format!("文件不存在: {}", path));
    }
    let mut buf = Vec::new();
    fs::File::open(file_path)
        .map_err(|e| e.to_string())?
        .read_to_end(&mut buf)
        .map_err(|e| e.to_string())?;
    Ok(base64::engine::general_purpose::STANDARD.encode(&buf))
}

/// 读取 .md 文件内容
#[command]
pub fn read_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| e.to_string())
}

/// 写入 .md 文件（原子化 + 历史快照）
#[command]
pub fn write_file(request: SaveRequest) -> Result<(), String> {
    let file_path = Path::new(&request.path);

    // 确保父目录存在
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    // 原子化写入：先写临时文件，再重命名
    atomic_write(file_path, request.content.as_bytes())?;

    // 触发历史快照
    save_history_snapshot(file_path)?;

    Ok(())
}

/// 批量保存
#[command]
pub fn write_files(requests: Vec<SaveRequest>) -> Result<(), String> {
    for req in requests {
        let file_path = Path::new(&req.path);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        atomic_write(file_path, req.content.as_bytes())?;
        save_history_snapshot(file_path)?;
    }
    Ok(())
}

/// 原子化写入：先写 .tmp 文件，再重命名替换原文件
/// POSIX rename() 是原子操作，保证不会出现半写状态
fn atomic_write(target: &Path, data: &[u8]) -> Result<(), String> {
    let tmp_path = target.with_extension("tmp");
    fs::write(&tmp_path, data).map_err(|e| e.to_string())?;
    fs::rename(&tmp_path, target).map_err(|e| {
        // 重命名失败时清理临时文件
        let _ = fs::remove_file(&tmp_path);
        e.to_string()
    })
}

/// 获取文件信息
#[command]
pub fn get_file_info(path: String) -> Result<DirEntry, String> {
    let file_path = Path::new(&path);
    let metadata = fs::metadata(file_path).map_err(|e| e.to_string())?;
    Ok(DirEntry {
        name: file_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
        path: path.clone(),
        is_dir: metadata.is_dir(),
        size: if metadata.is_file() { Some(metadata.len()) } else { None },
        modified_at: metadata.modified().ok().map(|t| {
            chrono::DateTime::<chrono::Local>::from(t)
                .format("%Y-%m-%dT%H:%M:%S")
                .to_string()
        }),
    })
}

// ========== 历史快照 ==========

/// 将当前文件保存到 .history 目录
fn save_history_snapshot(file_path: &Path) -> Result<(), String> {
    // 只对 .md 文件做快照
    if file_path.extension().and_then(|e| e.to_str()) != Some("md") {
        return Ok(());
    }

    // 确定 .history 目录位置（在书籍根目录下）
    let book_root = find_book_root(file_path);
    let history_dir = book_root.join(".history");

    // 创建 .history 目录
    fs::create_dir_all(&history_dir).map_err(|e| e.to_string())?;

    // 生成快照文件名
    let stem = file_path.file_stem().unwrap_or_default().to_string_lossy();
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let snapshot_name = format!("{}_{}.md.gz", stem, timestamp);
    let snapshot_path = history_dir.join(&snapshot_name);

    // 读取原文件并压缩写入
    let content = fs::read(file_path).map_err(|e| e.to_string())?;
    let out_file = fs::File::create(&snapshot_path).map_err(|e| e.to_string())?;
    let mut encoder = GzEncoder::new(out_file, Compression::default());
    encoder.write_all(&content).map_err(|e| e.to_string())?;
    encoder.finish().map_err(|e| e.to_string())?;

    Ok(())
}

/// 查找文件所属的书籍根目录（包含 book.json 的目录）
fn find_book_root(file_path: &Path) -> std::path::PathBuf {
    let mut current = file_path.parent().map(|p| p.to_path_buf());
    while let Some(dir) = current {
        if dir.join("book.json").exists() {
            return dir;
        }
        current = dir.parent().map(|p| p.to_path_buf());
    }
    // 回退到文件所在目录
    file_path.parent().unwrap_or(Path::new(".")).to_path_buf()
}

/// 列出历史快照
#[command]
pub fn list_history(file_path: String) -> Result<Vec<HistorySnapshot>, String> {
    let file_path = Path::new(&file_path);
    let book_root = find_book_root(file_path);
    let history_dir = book_root.join(".history");

    if !history_dir.exists() {
        return Ok(vec![]);
    }

    let stem = file_path.file_stem().unwrap_or_default().to_string_lossy();
    let mut snapshots = Vec::new();

    for entry in fs::read_dir(&history_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().to_string();
        // 匹配以章节名开头的快照文件
        if name.starts_with(&format!("{}_", stem)) && (name.ends_with(".md") || name.ends_with(".md.gz")) {
            let metadata = entry.metadata().map_err(|e| e.to_string())?;
            // 从文件名中提取时间戳
            let ext = if name.ends_with(".md.gz") { ".md.gz" } else { ".md" };
            let ts_part = name
                .strip_prefix(&format!("{}_", stem))
                .unwrap_or("")
                .strip_suffix(ext)
                .unwrap_or("");
            snapshots.push(HistorySnapshot {
                filename: name.clone(),
                path: entry.path().to_string_lossy().to_string(),
                timestamp: format_timestamp(ts_part),
                size: metadata.len(),
            });
        }
    }

    // 按时间戳降序排列（最新的在前）
    snapshots.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    Ok(snapshots)
}

/// 读取历史快照内容
#[command]
pub fn read_history_snapshot(snapshot_path: String) -> Result<String, String> {
    if snapshot_path.ends_with(".gz") {
        let file = fs::File::open(&snapshot_path).map_err(|e| e.to_string())?;
        let mut decoder = GzDecoder::new(file);
        let mut string = String::new();
        decoder.read_to_string(&mut string).map_err(|e| e.to_string())?;
        Ok(string)
    } else {
        fs::read_to_string(&snapshot_path).map_err(|e| e.to_string())
    }
}

/// 恢复历史快照（覆盖当前文件）
#[command]
pub fn restore_snapshot(snapshot_path: String, target_path: String) -> Result<(), String> {
    let snapshot = Path::new(&snapshot_path);
    let target = Path::new(&target_path);

    if !snapshot.exists() {
        return Err("快照文件不存在".to_string());
    }

    // 先保存当前版本的快照
    if target.exists() {
        save_history_snapshot(target)?;
    }

    // 复制快照覆盖目标文件 (或解压)
    if snapshot_path.ends_with(".gz") {
        let file = fs::File::open(snapshot).map_err(|e| e.to_string())?;
        let mut decoder = GzDecoder::new(file);
        let mut content = Vec::new();
        decoder.read_to_end(&mut content).map_err(|e| e.to_string())?;
        fs::write(target, &content).map_err(|e| e.to_string())?;
    } else {
        fs::copy(snapshot, target).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn format_timestamp(raw: &str) -> String {
    // raw 格式: YYYYMMDD_HHMMSS → YYYY-MM-DD HH:MM:SS
    if raw.len() == 15 && raw.chars().nth(8) == Some('_') {
        format!(
            "{}-{}-{} {}:{}:{}",
            &raw[0..4],
            &raw[4..6],
            &raw[6..8],
            &raw[9..11],
            &raw[11..13],
            &raw[13..15]
        )
    } else {
        raw.to_string()
    }
}

/// 设置封面图片并将其复制到工作区内
#[command]
pub fn set_cover_image(book_path: String, image_path: String) -> Result<String, String> {
    let book_dir = Path::new(&book_path);
    let src = Path::new(&image_path);

    if !src.exists() {
        return Err("选择的图片不存在".to_string());
    }

    let ext = src.extension().and_then(|e| e.to_str()).unwrap_or("png");
    let cover_filename = format!("cover.{}", ext);
    let dest = book_dir.join(&cover_filename);

    fs::copy(src, &dest).map_err(|e| e.to_string())?;

    Ok(cover_filename)
}