use ignore::WalkBuilder;

pub fn get_file_paths(repo_path: &str) -> Vec<String> {
    WalkBuilder::new(repo_path)
        .build()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .map(|e| e.path().display().to_string())
        .collect()
}
