use std::{fs::FileType, os::windows::fs::FileTypeExt};

mod colors;

fn format_size(meta_len: u64) -> String {
    if meta_len < 1024 {
        format!("{}{:.1} Б{}", colors::VALUE, meta_len as f64, colors::RESET)
    } else if meta_len < 1024 * 1024 {
        format!(
            "{}{:.1} КБ{}",
            colors::VALUE,
            meta_len as f64 / 1024.0,
            colors::RESET
        )
    } else if meta_len < 1024 * 1024 * 1024 {
        format!(
            "{}{:.1} МБ{}",
            colors::VALUE,
            meta_len as f64 / (1024.0 * 1024.0),
            colors::RESET
        )
    } else {
        format!(
            "{}{:.1} ГБ{}",
            colors::VALUE,
            meta_len as f64 / (1024.0 * 1024.0 * 1024.0),
            colors::RESET
        )
    }
}

fn format_file_type(ft: FileType) -> String {
    if ft.is_dir() {
        format!("{}[dir]{}         ", colors::DIR, colors::RESET)
    } else if ft.is_file() {
        format!("{}[file]{}        ", colors::FILE, colors::RESET)
    } else if ft.is_symlink_dir() {
        format!("{}[symlink_dir]{} ", colors::DIR, colors::RESET)
    } else if ft.is_symlink_file() {
        format!("{}[symlink_file]{}", colors::FILE, colors::RESET)
    } else if ft.is_symlink() {
        format!("{}[symlink]{}     ", colors::FILE, colors::RESET)
    } else {
        format!("{}[unknown]{}     ", colors::FILE, colors::RESET)
    }
}

fn print(entries: std::fs::ReadDir, path: String) {
    println!(
        "{}Информация о директории (путь: {path:?})\n{}",
        colors::SUCCESS,
        colors::RESET
    );

    let start = std::time::Instant::now();
    let mut files: Vec<(String, String, String)> = Vec::new();
    let mut max_len: usize = 0;
    let mut file_count = 0;
    let mut directory_count = 0;
    let mut total_size: u64 = 0;

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

        let size: String = if ft.is_dir() || ft.is_symlink_dir() {
            directory_count += 1;
            format!("{}<DIR>{}", colors::VALUE, colors::RESET)
        } else if ft.is_symlink() {
            file_count += 1;
            format_size(meta.len())
        } else {
            file_count += 1;
            total_size += meta.len();
            format_size(meta.len())
        };

        let file_type = format_file_type(ft);
        files.push((file_type, name_str, size));
    }

    let total_size = format_size(total_size);

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
    println!(
        "\n{}количество файлов: {}{}\n{}количество директорий: {}{}\n{}общий размер файлов: {}{}",
        colors::NOTE,
        colors::VALUE,
        file_count,
        colors::NOTE,
        colors::VALUE,
        directory_count,
        colors::NOTE,
        total_size,
        colors::RESET
    );

    println!(
        "\n{}Выполнено за {}{:?}{}",
        colors::SUCCESS,
        colors::VALUE,
        start.elapsed(),
        colors::RESET
    );
}

fn main() {
    let mut args = std::env::args();
    let arg = match args.nth(1) {
        Some(arg) => arg,
        None => {
            eprintln!(
                "{}Ошибка: вы не указали аргумент при вызове утилиты.\n{}Помощь: вместо dir_info, напишите dir_info <путь> (либо какой-нибудь флаг, например: dir_info C:\\Windows).{}",
                colors::ERROR,
                colors::HELP,
                colors::RESET
            );
            std::process::exit(1);
        }
    };

    match arg.as_str() {
        "--version" | "-v" => {
            println!("{}", std::env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        }
        _ => (),
    }

    match std::fs::read_dir(&arg) {
        Ok(entries) => {
            print(entries, arg);
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
