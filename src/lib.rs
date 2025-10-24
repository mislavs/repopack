use std::{error::Error, fs};
use tiktoken_rs::cl100k_base;

use ignore::WalkBuilder;

pub fn get_file_metadata(repo_path: &str) -> Vec<FileMetadata> {
    get_file_paths(repo_path)
        .iter()
        .map(|path| FileMetadata {
            path: path.to_string(),
            token_count: calculate_token_size_for_file(path).ok(),
        })
        .collect()
}

fn calculate_token_size_for_file(file_path: &str) -> Result<usize, Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;

    let bpe = cl100k_base().unwrap();
    Ok(bpe.encode_with_special_tokens(&content).len())
}

fn get_file_paths(repo_path: &str) -> Vec<String> {
    WalkBuilder::new(repo_path)
        .build()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .map(|e| e.path().display().to_string())
        .collect()
}

pub struct FileMetadata {
    pub path: String,
    pub token_count: Option<usize>,
}
