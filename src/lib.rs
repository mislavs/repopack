use std::{error::Error, fs, path::{Path, PathBuf}};
use tiktoken_rs::cl100k_base;

use ignore::WalkBuilder;

pub fn get_file_metadata(repo_path: &str) -> impl Iterator<Item = FileMetadata> {
    get_file_paths(repo_path)
        .into_iter()
        .map(|path| FileMetadata {
            path: path.clone(),
            token_count: calculate_token_size_for_file(&path).ok(),
        })
}

fn calculate_token_size_for_file(file_path: &Path) -> Result<usize, Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;

    let bpe = cl100k_base()?;
    Ok(bpe.encode_with_special_tokens(&content).len())
}

fn get_file_paths(repo_path: &str) -> Vec<PathBuf> {
    WalkBuilder::new(repo_path)
        .build()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .map(|e| e.path().to_path_buf())
        .collect()
}

pub struct FileMetadata {
    pub path: PathBuf,
    pub token_count: Option<usize>,
}
