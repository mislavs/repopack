use std::{error::Error, fs, path::PathBuf};
use tiktoken_rs::{cl100k_base, CoreBPE};

use ignore::WalkBuilder;

pub fn get_file_metadata(repo_path: &str) -> impl Iterator<Item = FileMetadata> {
    let bpe = cl100k_base().ok();
    
    WalkBuilder::new(repo_path)
        .build()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .map(move |e| {
            let path = e.path().to_path_buf();
            let token_count = bpe.as_ref()
                .and_then(|tokenizer| calculate_token_size_for_file(&path, tokenizer).ok());
            let line_count = fs::read_to_string(&path).unwrap_or_default().lines().count();
            
            FileMetadata {
                path,
                token_count,
                line_count
            }
        })
}

fn calculate_token_size_for_file(file_path: &PathBuf, bpe: &CoreBPE) -> Result<usize, Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;
    Ok(bpe.encode_with_special_tokens(&content).len())
}

pub struct FileMetadata {
    pub path: PathBuf,
    pub token_count: Option<usize>,
    pub line_count: usize,
}
