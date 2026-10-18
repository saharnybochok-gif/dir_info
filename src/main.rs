use std::{fs::FileType, os::windows::fs::FileTypeExt};

mod colors;

fn format_size(meta_len: u64, ft: FileType) -> String {
    if ft.is_dir() || ft.is_symlink_dir() {
        format!("<DIR>")
    } else if meta_len < 1024 {
        format!("{:.1} Б", meta_len as f64)
    } else if meta_len < 1024 * 1024 {
        format!("{:.1} КБ", meta_len as f64 / 1024.0)
    } else if meta_len < 1024 * 1024 * 1024 {
        format!("{:.1} МБ", meta_len as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} ГБ", meta_len as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

fn format_file_type(ft: FileType) -> String {
    if ft.is_dir() {
        format!("{}[dir]{}         ", colors::FILE_TYPE, colors::RESET)
    } else if ft.is_file() {
        format!("{}[file]{}        ", colors::FILE_TYPE, colors::RESET)
    } else if ft.is_symlink_dir() {
        format!("{}[symlink_dir]{} ", colors::FILE_TYPE, colors::RESET)
    } else if ft.is_symlink_file() {
        format!("{}[symlink_file]{}", colors::FILE_TYPE, colors::RESET)
    } else if ft.is_symlink() {
        format!("{}[symlink]{}     ", colors::FILE_TYPE, colors::RESET)
    } else {
        format!("{}[unknown]{}     ", colors::FILE_TYPE, colors::RESET)
    }
}

fn print(entries: std::fs::ReadDir, path: String) {
    println!(
        "{}Информация о папке (путь: {path:?})\n{}",
        colors::SUCCESS,
        colors::RESET
    );

    let mut files: Vec<(String, String, String)> = Vec::new();
    let mut max_len: usize = 0;

    for entry in entries {
        let Ok(file) = entry else {
            continue;
        };
        let Ok(ft) = file.file_type() else {
            continue;
        };
        let Ok(meta) = file.metadata() else {
            continue;
        };

        let name_str = format!("{:?}", file.file_name());
        if name_str.len() > max_len {
            max_len = name_str.len();
        }

        let size: String = format_size(meta.len(), ft);

        let file_type = format_file_type(ft);
        files.push((file_type, name_str, size));
    }

    for (file_type, name, size) in files {
        println!(
            "{:<4} {}name:{} {:<width$} {}size: {}{}{}",
            file_type,
            colors::NOTE,
            colors::VALUE,
            name,
            colors::NOTE,
            colors::VALUE,
            size,
            colors::RESET,
            width = max_len + 2
        );
    }
}

fn main() {
    let mut args = std::env::args();
    let path = match args.nth(1) {
        Some(path) => path,
        None => {
            eprintln!(
                "{}Ошибка: вы не указали путь в аргументах при вызове утилиты.\n{}Помощь: вместо dir_info, напишите dir_info <путь> (например: dir_info C:\\Windows).{}",
                colors::ERROR,
                colors::HELP,
                colors::RESET
            );
            std::process::exit(1);
        }
    };

    match std::fs::read_dir(&path) {
        Ok(entries) => {
            print(entries, path);
        }
        Err(e) => {
            eprintln!(
                "{}Ошибка: произошла непредвиденная ошибка при попытке открыть директорию.\n{}текст ошибки: {e}.\n{}Помощь: убедитесь то что вы указали существующий путь, при присутствии пробелов в пути напишите в аргументах путь в кавычках.{}",
                colors::ERROR,
                colors::NOTE,
                colors::HELP,
                colors::RESET
            );
        }
    }
}
