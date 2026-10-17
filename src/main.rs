use std::fmt::format;

mod colors;

fn print(entries: std::fs::ReadDir, path: String) {
    println!(
        "{}Информация о папке (путь: {path:?})\n{}",
        colors::SUCCESS,
        colors::RESET
    );

    for entry in entries {
        let Ok(file) = entry else {
            continue;
        };
        let Ok(ft) = file.file_type() else {
            continue;
        };

        let size: String = if ft.is_dir() {
            format!("{}<DIR>{}", colors::VALUE, colors::RESET)
        } else {
            let Ok(meta) = file.metadata() else {
                continue;
            };
            if meta.len() < 1024 {
                format!("{}{} Б{}", colors::VALUE, meta.len(), colors::RESET)
            } else {
                format!("{}{} КБ{}", colors::VALUE, meta.len() / 1024, colors::RESET)
            }
        };

        let file_type = if ft.is_dir() {
            format!("📁 {}[dir]{} ", colors::FILE_TYPE, colors::RESET)
        } else {
            format!("📃 {}[file]{}", colors::FILE_TYPE, colors::RESET)
        };

        println!(
            "{:<4} {:<50} {:<10}",
            file_type,
            format!(
                "{}name: {}{:?}{}",
                colors::NOTE,
                colors::VALUE,
                file.file_name(),
                colors::RESET
            ),
            format!(
                "{}size: {}{}{}",
                colors::NOTE,
                colors::VALUE,
                size,
                colors::RESET
            )
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
