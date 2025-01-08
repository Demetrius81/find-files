use std::ffi::OsStr;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

const FILE_SIZE_BASE: f64 = 1e6;

fn get_input(query: &str) -> std::io::Result<String> {
    println!("{}", query);
    std::io::stdout().flush()?;
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_owned())
}

fn get_search_data() -> Option<(String, String, Vec<String>)> {
    let search_path = match get_input("Enter path to the dir to search for file in: ") {
        Ok(path) => path,
        Err(_) => return None,
    };

    let search_filename = match get_input("Enter filename to search (without extension): ") {
        Ok(path) => path,
        Err(_) => return None,
    };

    let search_extensions = match get_input("Enter file extensions separated by spaces: ") {
        Ok(extensions_string) => get_extensions(&extensions_string),
        Err(_) => return None,
    };

    if search_path.is_empty() || (search_filename.is_empty() && search_extensions.is_empty()) {
        println!("Enter something");
        return None;
    }
    Some((
        search_path.to_lowercase(),
        search_filename.to_lowercase(),
        search_extensions,
    ))
}

fn search_files(
    search_path: &str,
    filename: &str,
    extensions: &Vec<String>,
    now: &Instant,
    results_count: &mut i32,
) {
    let is_no_extension = extensions.is_empty();
    let is_empty_file_name = filename.is_empty();

    let files = match std::fs::read_dir(search_path) {
        Ok(files) => files,
        Err(_) => return,
    };

    for entry in files {
        if let Ok(entry) = entry {
            let path = entry.path();
            let file_name = convert_os_string(path.file_stem());
            let file_extension = convert_os_string(path.extension());

            if path.is_dir() {
                if is_no_extension && file_name.contains(filename) {
                    file_found(&path, now, results_count);
                }

                search_files(
                    path.to_str().unwrap_or_default(),
                    filename,
                    extensions,
                    now,
                    results_count,
                );
            } else if is_empty_file_name && extensions.contains(&file_extension) {
                file_found(&path, now, results_count);
            } else if path.is_file() && file_name.contains(filename) {
                if (!is_no_extension && extensions.contains(&file_extension)) || is_no_extension {
                    file_found(&path, now, results_count);
                }
            }
        }
    }
}


fn get_extensions(extensions_string: &str) -> Vec<String> {
    extensions_string
        .split_whitespace()
        .map(|word| word.to_lowercase())
        .collect()
}

fn file_found(path: &PathBuf, now: &Instant, results_count: &mut i32) {
    *results_count += 1;
    print_path_info(path, now);
}

fn print_path_info(path: &PathBuf, now: &Instant) {
    print!(
        "{} - Found in {} seconds",
        path.display(),
        now.elapsed().as_secs_f64()
    );

    match std::fs::metadata(path) {
        Ok(metadata) => {
            print!(" - {} MB\n", metadata.len() as f64 / FILE_SIZE_BASE);
        }
        Err(_) => println!(),
    }
}

fn convert_os_string(os_str: Option<&OsStr>) -> String {
    os_str
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_lowercase()
}

fn main() {
    loop {
        let (path, file_name, extensions) = match get_search_data() {
            Some(search_data) => search_data,
            None => continue,
        };

        println!();

        let now = Instant::now();
        let mut result_count = 0;

        search_files(&path, &file_name, &extensions, &now, &mut result_count);

        println!(
            "\nTotal time: {} seconds\n{}matches\n",
            now.elapsed().as_secs_f64(),
            file_name
        );
    }
}
