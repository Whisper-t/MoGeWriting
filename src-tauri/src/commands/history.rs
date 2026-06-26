use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use tauri::command;
use crate::models::*;

/// 导出为纯 TXT 文件
/// - 按 book.json 顺序合并章节
/// - 清洗所有 HTML 注释（灵感标记）
/// - 清洗 Markdown 标记符号
/// - 输出格式兼容主流小说阅读器的章节自动识别
#[command]
pub fn export_txt(request: ExportRequest) -> Result<String, String> {
    let book_path = Path::new(&request.book_path);
    let book = read_book_index_for_export(book_path)?;

    let mut output = String::new();

    // 添加书名
    output.push_str(&format!("《{}》\n", book.name));
    if let Some(author) = &book.author {
        if !author.is_empty() {
            output.push_str(&format!("作者：{}\n", author));
        }
    }
    output.push('\n');
    if let Some(desc) = &book.description {
        if !desc.is_empty() {
            output.push_str(&format!("简介：{}\n", desc));
        }
    }
    output.push_str(&"=".repeat(50));
    output.push_str("\n\n");

    // 按卷/章结构输出，保留卷标题以便其他阅读器识别
    match &book.volumes {
        Some(volumes) => {
            let mut vols = volumes.clone();
            vols.sort_by_key(|v| v.order);
            for vol in &vols {
                // 输出卷标题（独立一行，前后空行）
                output.push_str(&format!("\n{}\n\n", vol.title));

                let mut chs = vol.chapters.clone();
                chs.sort_by_key(|c| c.order);
                for ch in &chs {
                    let md_path = book_path.join(&ch.file);
                    if let Ok(content) = fs::read_to_string(&md_path) {
                        // 章节标题独立一行，前后留空行 — 这是阅读器识别章节的通用格式
                        output.push_str(&format!("\n{}\n\n", ch.title));
                        let cleaned = clean_markdown_for_txt(&content);
                        output.push_str(&cleaned);
                        output.push('\n');
                    }
                }
            }
        }
        None => {
            let mut chapters = book.chapters.clone().unwrap_or_default();
            chapters.sort_by_key(|c| c.order);
            for ch in &chapters {
                let md_path = book_path.join(&ch.file);
                if let Ok(content) = fs::read_to_string(&md_path) {
                    output.push_str(&format!("\n{}\n\n", ch.title));
                    let cleaned = clean_markdown_for_txt(&content);
                    output.push_str(&cleaned);
                    output.push('\n');
                }
            }
        }
    }

    // 确保输出目录存在
    if let Some(parent) = Path::new(&request.output_path).parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    fs::write(&request.output_path, &output).map_err(|e| e.to_string())?;
    Ok(request.output_path)
}

/// 导出为 ZIP 压缩包（全量备份）
#[command]
pub fn export_zip(request: ExportRequest) -> Result<String, String> {
    let book_path = Path::new(&request.book_path);
    if !book_path.exists() {
        return Err("书籍目录不存在".to_string());
    }

    let output_path = Path::new(&request.output_path);
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let file = fs::File::create(output_path).map_err(|e| e.to_string())?;
    let mut zip_writer = zip::ZipWriter::new(file);
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    // 递归添加目录中的所有文件
    add_dir_to_zip(&mut zip_writer, book_path, book_path, &options)?;

    zip_writer.finish().map_err(|e| e.to_string())?;
    Ok(request.output_path.to_string())
}

/// 从 ZIP 恢复
#[command]
pub fn restore_from_zip(request: RestoreRequest) -> Result<(), String> {
    let zip_path = Path::new(&request.zip_path);
    let target_path = Path::new(&request.target_path);

    if !zip_path.exists() {
        return Err("ZIP 文件不存在".to_string());
    }

    let file = fs::File::open(zip_path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).map_err(|e| e.to_string())?;
        let entry_path = target_path.join(entry.name());

        if entry.is_dir() {
            fs::create_dir_all(&entry_path).map_err(|e| e.to_string())?;
        } else {
            if let Some(parent) = entry_path.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            let mut outfile = fs::File::create(&entry_path).map_err(|e| e.to_string())?;
            let mut buffer = Vec::new();
            entry.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
            outfile.write_all(&buffer).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

/// 导出为 EPUB 电子书
#[command]
pub fn export_epub(request: ExportRequest) -> Result<String, String> {
    let book_path = Path::new(&request.book_path);
    let book = read_book_index_for_export(book_path)?;
    crate::commands::epub::export_epub(&request, &book, book_path)
}

/// 获取字数统计
#[command]
pub fn count_words(book_path: String) -> Result<usize, String> {
    let book_path = Path::new(&book_path);
    let book = read_book_index_for_export(book_path)?;
    let chapters = collect_chapters_in_order(&book);
    let mut total = 0usize;
    for ch in &chapters {
        let md_path = book_path.join(&ch.file);
        if let Ok(content) = fs::read_to_string(&md_path) {
            total += count_chinese_words(&content);
        }
    }
    Ok(total)
}

// ========== 内部辅助函数 ==========

fn read_book_index_for_export(book_path: &Path) -> Result<BookIndex, String> {
    let json_path = book_path.join("book.json");
    let content = fs::read_to_string(&json_path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

fn collect_chapters_in_order(book: &BookIndex) -> Vec<ChapterEntry> {
    match &book.volumes {
        Some(volumes) => {
            let mut vols = volumes.clone();
            vols.sort_by_key(|v| v.order);
            let mut result = Vec::new();
            for vol in &vols {
                let mut chs = vol.chapters.clone();
                chs.sort_by_key(|c| c.order);
                result.extend(chs);
            }
            result
        }
        None => {
            let mut chapters = book.chapters.clone().unwrap_or_default();
            chapters.sort_by_key(|c| c.order);
            chapters
        }
    }
}

/// 清洗 Markdown 内容，转为纯文本
fn clean_markdown_for_txt(content: &str) -> String {
    let mut text = content.to_string();

    // 1. 移除 HTML 注释（灵感标记）: <!-- ... -->
    text = remove_html_comments(&text);

    // 2. 移除 Markdown 标题标记 (#)
    text = remove_markdown_headers(&text);

    // 3. 移除加粗/斜体标记
    text = remove_markdown_emphasis(&text);

    // 4. 移除链接，保留文字
    text = remove_markdown_links(&text);

    // 5. 移除图片标记
    text = remove_markdown_images(&text);

    // 6. 移除代码块标记
    text = remove_markdown_code_blocks(&text);

    // 7. 移除多余空行（保留最多一个空行）
    text = normalize_blank_lines(&text);

    text
}

fn remove_html_comments(text: &str) -> String {
    let re = regex_lite::Regex::new(r"<!--[\s\S]*?-->").unwrap();
    re.replace_all(text, "").to_string()
}

fn remove_markdown_headers(text: &str) -> String {
    let re = regex_lite::Regex::new(r"(?m)^#{1,6}\s+").unwrap();
    re.replace_all(text, "").to_string()
}

fn remove_markdown_emphasis(text: &str) -> String {
    let mut result = text.to_string();
    // 粗体 + 斜体
    let re_bold = regex_lite::Regex::new(r"\*\*\*(.+?)\*\*\*").unwrap();
    result = re_bold.replace_all(&result, "$1").to_string();
    // 粗体
    let re_bold2 = regex_lite::Regex::new(r"\*\*(.+?)\*\*").unwrap();
    result = re_bold2.replace_all(&result, "$1").to_string();
    // 斜体
    let re_italic = regex_lite::Regex::new(r"\*(.+?)\*").unwrap();
    result = re_italic.replace_all(&result, "$1").to_string();
    result
}

fn remove_markdown_links(text: &str) -> String {
    let re = regex_lite::Regex::new(r"\[([^\]]+)\]\([^)]+\)").unwrap();
    re.replace_all(text, "$1").to_string()
}

fn remove_markdown_images(text: &str) -> String {
    let re = regex_lite::Regex::new(r"!\[([^\]]*)\]\([^)]+\)").unwrap();
    re.replace_all(text, "").to_string()
}

fn remove_markdown_code_blocks(text: &str) -> String {
    let mut result = text.to_string();
    // 围栏代码块
    let re_fence = regex_lite::Regex::new(r"(?s)```[^\n]*\n.*?```").unwrap();
    result = re_fence.replace_all(&result, "").to_string();
    // 行内代码
    let re_inline = regex_lite::Regex::new(r"`([^`]+)`").unwrap();
    result = re_inline.replace_all(&result, "$1").to_string();
    result
}

fn normalize_blank_lines(text: &str) -> String {
    let re = regex_lite::Regex::new(r"\n{3,}").unwrap();
    re.replace_all(text, "\n\n").to_string()
}

fn count_chinese_words(text: &str) -> usize {
    let cleaned = remove_html_comments(text);
    let chinese_chars = cleaned
        .chars()
        .filter(|c| c >= &'\u{4e00}' && c <= &'\u{9fff}')
        .count();
    let english_words = cleaned
        .split(|c: char| c.is_whitespace() || (c >= '\u{4e00}' && c <= '\u{9fff}'))
        .filter(|s| !s.is_empty())
        .count();
    chinese_chars + english_words
}

fn add_dir_to_zip(
    zip_writer: &mut zip::ZipWriter<fs::File>,
    root: &Path,
    current: &Path,
    options: &zip::write::FileOptions,
) -> Result<(), String> {
    for entry in fs::read_dir(current).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let relative = path.strip_prefix(root).map_err(|e| e.to_string())?;
        let name = relative.to_string_lossy().to_string();

        if path.is_dir() {
            let dir_name = if name.ends_with('/') || name.ends_with('\\') {
                name
            } else {
                format!("{}/", name)
            };
            zip_writer
                .add_directory(&dir_name, *options)
                .map_err(|e| e.to_string())?;
            add_dir_to_zip(zip_writer, root, &path, options)?;
        } else {
            zip_writer
                .start_file(&name, *options)
                .map_err(|e| e.to_string())?;
            let mut file = fs::File::open(&path).map_err(|e| e.to_string())?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
            zip_writer.write_all(&buffer).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}