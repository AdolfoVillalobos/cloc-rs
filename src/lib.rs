use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use walkdir::WalkDir;


pub fn count_lines(file_path: &Path) -> io::Result<usize> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().count())
}

pub fn count_lines_in_repo(path: &Path, extension: Option<&str>) -> io::Result<usize> {
    let mut total_lines = 0;

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = extension {
                if path.extension().and_then(|s| s.to_str()) == Some(ext) {
                    total_lines += count_lines(path)?;
                }
            } else {
                total_lines += count_lines(path)?;
            }
        }
    }

    Ok(total_lines)
}
