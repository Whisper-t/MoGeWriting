use std::fs;
use std::io::Write;
use std::path::Path;
use crate::models::*;

/// 导出为 EPUB 电子书
pub fn export_epub(request: &ExportRequest, book: &BookIndex, book_path: &Path) -> Result<String, String> {
    let chapters = collect_chapters_in_order(book);
    let output_path = Path::new(&request.output_path);
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let file = fs::File::create(output_path).map_err(|e| e.to_string())?;
    let mut zip = zip::ZipWriter::new(file);

    // 1. mimetype（必须第一个，不压缩）
    let store = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
    zip.start_file("mimetype", store).map_err(|e| e.to_string())?;
    zip.write_all(b"application/epub+zip").map_err(|e| e.to_string())?;

    let deflate = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    // 2. META-INF/container.xml
    zip.start_file("META-INF/container.xml", deflate).map_err(|e| e.to_string())?;
    zip.write_all(CONTAINER_XML.as_bytes()).map_err(|e| e.to_string())?;

    // 3. 生成章节 XHTML
    let mut chapter_ids: Vec<String> = Vec::new();
    let mut chapter_titles: Vec<String> = Vec::new();
    for (i, ch) in chapters.iter().enumerate() {
        let id = format!("chapter_{}", i + 1);
        let filename = format!("OEBPS/{}.xhtml", id);
        let md_path = book_path.join(&ch.file);
        let content = fs::read_to_string(&md_path).unwrap_or_default();
        let xhtml = markdown_to_xhtml(&content, &ch.title);

        zip.start_file(&filename, deflate).map_err(|e| e.to_string())?;
        zip.write_all(xhtml.as_bytes()).map_err(|e| e.to_string())?;

        chapter_ids.push(id);
        chapter_titles.push(ch.title.clone());
    }

    // 4. OEBPS/content.opf
    let opf = generate_opf(book, &chapter_ids, &chapter_titles);
    zip.start_file("OEBPS/content.opf", deflate).map_err(|e| e.to_string())?;
    zip.write_all(opf.as_bytes()).map_err(|e| e.to_string())?;

    // 5. OEBPS/toc.ncx
    let ncx = generate_ncx(book, &chapter_ids, &chapter_titles);
    zip.start_file("OEBPS/toc.ncx", deflate).map_err(|e| e.to_string())?;
    zip.write_all(ncx.as_bytes()).map_err(|e| e.to_string())?;

    zip.finish().map_err(|e| e.to_string())?;
    Ok(request.output_path.to_string())
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

fn markdown_to_xhtml(markdown: &str, title: &str) -> String {
    let mut html = markdown.to_string();
    // 移除灵感注释
    let re = regex_lite::Regex::new(r"<!--inspiration:[\s\S]*?-->[\s\S]*?<!--/inspiration-->").unwrap();
    html = re.replace_all(&html, "").to_string();
    let re2 = regex_lite::Regex::new(r"<!--[\s\S]*?-->").unwrap();
    html = re2.replace_all(&html, "").to_string();
    // 转义 HTML 实体（先转义 & 避免重复转义）
    html = html.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace("\"", "&quot;");
    // 标题
    html = regex_lite::Regex::new(r"(?m)^#### (.+)$").unwrap().replace_all(&html, "<h4>$1</h4>").to_string();
    html = regex_lite::Regex::new(r"(?m)^### (.+)$").unwrap().replace_all(&html, "<h3>$1</h3>").to_string();
    html = regex_lite::Regex::new(r"(?m)^## (.+)$").unwrap().replace_all(&html, "<h2>$1</h2>").to_string();
    html = regex_lite::Regex::new(r"(?m)^# (.+)$").unwrap().replace_all(&html, "<h1>$1</h1>").to_string();
    // 粗体/斜体
    html = regex_lite::Regex::new(r"\*\*\*(.+?)\*\*\*").unwrap().replace_all(&html, "<strong><em>$1</em></strong>").to_string();
    html = regex_lite::Regex::new(r"\*\*(.+?)\*\*").unwrap().replace_all(&html, "<strong>$1</strong>").to_string();
    html = regex_lite::Regex::new(r"\*(.+?)\*").unwrap().replace_all(&html, "<em>$1</em>").to_string();
    // 段落
    let lines: Vec<&str> = html.lines().collect();
    let mut result = Vec::new();
    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            result.push(String::new());
        } else if trimmed.starts_with('<') {
            result.push(trimmed.to_string());
        } else {
            result.push(format!("<p>{}</p>", trimmed));
        }
    }
    html = result.join("\n");

    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <!DOCTYPE html>\n\
         <html xmlns=\"http://www.w3.org/1999/xhtml\">\n\
         <head><title>{}</title></head>\n\
         <body>\n<h1>{}</h1>\n{}</body>\n</html>",
        escape_xml(title),
        escape_xml(title),
        html,
    )
}

fn generate_opf(book: &BookIndex, chapter_ids: &[String], _chapter_titles: &[String]) -> String {
    let author = book.author.as_deref().unwrap_or("佚名");
    let now = chrono::Local::now().format("%Y-%m-%d").to_string();

    let mut manifest = String::new();
    let mut spine = String::new();

    for id in chapter_ids.iter() {
        manifest.push_str(&format!(
            "    <item id=\"{}\" href=\"{}.xhtml\" media-type=\"application/xhtml+xml\"/>\n",
            id, id
        ));
        spine.push_str(&format!("    <itemref idref=\"{}\"/>\n", id));
    }

    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <package xmlns=\"http://www.idpf.org/2007/opf\" version=\"2.0\" unique-identifier=\"book-id\">\n\
         <metadata xmlns:dc=\"http://purl.org/dc/elements/1.1/\">\n\
           <dc:title>{}</dc:title>\n\
           <dc:creator>{}</dc:creator>\n\
           <dc:language>zh-CN</dc:language>\n\
           <dc:date>{}</dc:date>\n\
           <dc:identifier id=\"book-id\">urn:uuid:{}</dc:identifier>\n\
         </metadata>\n\
         <manifest>\n\
           <item id=\"ncx\" href=\"toc.ncx\" media-type=\"application/x-dtbncx+xml\"/>\n\
           {}\
         </manifest>\n\
         <spine toc=\"ncx\">\n\
           {}\
         </spine>\n\
         </package>",
        escape_xml(&book.name),
        escape_xml(author),
        now,
        uuid::Uuid::new_v4(),
        manifest,
        spine,
    )
}

fn generate_ncx(book: &BookIndex, chapter_ids: &[String], chapter_titles: &[String]) -> String {
    let mut nav_points = String::new();
    for (i, id) in chapter_ids.iter().enumerate() {
        nav_points.push_str(&format!(
            "    <navPoint id=\"nav_{}\" playOrder=\"{}\">\n\
               <navLabel><text>{}</text></navLabel>\n\
               <content src=\"{}.xhtml\"/>\n\
             </navPoint>\n",
            id,
            i + 1,
            escape_xml(&chapter_titles[i]),
            id,
        ));
    }

    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <ncx xmlns=\"http://www.daisy.org/z3986/2005/ncx/\" version=\"2005-1\">\n\
         <head>\n\
           <meta name=\"dtb:uid\" content=\"urn:uuid:{}\"/>\n\
         </head>\n\
         <docTitle><text>{}</text></docTitle>\n\
         <navMap>\n\
           {}\
         </navMap>\n\
         </ncx>",
        uuid::Uuid::new_v4(),
        escape_xml(&book.name),
        nav_points,
    )
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace("\"", "&quot;")
        .replace('\'', "&apos;")
}

const CONTAINER_XML: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
<container version=\"1.0\" xmlns=\"urn:oasis:names:tc:opendocument:xmlns:container\">\n\
  <rootfiles>\n\
    <rootfile full-path=\"OEBPS/content.opf\" media-type=\"application/oebps-package+xml\"/>\n\
  </rootfiles>\n\
</container>";