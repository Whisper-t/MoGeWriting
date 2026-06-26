mod commands;
mod models;

use commands::workspace;
use commands::file_io;
use commands::history;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            // 系统
            workspace::get_home_dir,
            // 工作区管理
            workspace::list_workspaces,
            workspace::create_workspace,
            workspace::read_book_index,
            workspace::write_book_index,
            workspace::create_chapter,
            workspace::create_volume,
            workspace::rename_entry,
            workspace::delete_chapter,
            workspace::delete_volume,
            workspace::reorder,
            workspace::list_directory,
            workspace::delete_workspace,
            // 文件 I/O
            file_io::open_in_browser,
            file_io::open_in_explorer,
            file_io::read_file_base64,
            file_io::read_file,
            file_io::write_file,
            file_io::write_files,
            file_io::get_file_info,
            file_io::set_cover_image,
            // 历史快照
            file_io::list_history,
            file_io::read_history_snapshot,
            file_io::restore_snapshot,
            // 导出与恢复
            history::export_txt,
            history::export_zip,
            history::export_epub,
            history::restore_from_zip,
            history::count_words,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}