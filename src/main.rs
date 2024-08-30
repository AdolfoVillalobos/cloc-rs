use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use walkdir::WalkDir;

use cloc_rs::{count_lines, count_lines_in_repo};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path> [extension]", args[0]);
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    let extension = args.get(2).map(|s| s.as_str());

    match count_lines_in_repo(path, extension) {
        Ok(lines) => println!("Total lines: {}", lines),
        Err(e) => eprintln!("Error: {}", e),
    }
}
