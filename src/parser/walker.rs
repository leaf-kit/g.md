use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct MdFile {
    pub path: PathBuf,
    pub content: String,
    pub modified: Option<chrono::NaiveDateTime>,
}

pub fn walk_markdown_files(root: &Path) -> Vec<MdFile> {
    let mut files = Vec::new();

    for entry in WalkDir::new(root)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path
            .extension()
            .is_some_and(|ext| ext == "md" || ext == "markdown")
        {
            if let Ok(content) = std::fs::read_to_string(path) {
                let modified = entry
                    .metadata()
                    .ok()
                    .and_then(|m| m.modified().ok())
                    .map(|t| {
                        let duration = t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
                        chrono::DateTime::from_timestamp(duration.as_secs() as i64, 0)
                            .unwrap_or_default()
                            .naive_local()
                    });

                files.push(MdFile {
                    path: path.to_path_buf(),
                    content,
                    modified,
                });
            }
        }
    }

    files
}

pub fn walk_all_files(root: &Path) -> Vec<PathBuf> {
    WalkDir::new(root)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_walk_markdown_files() {
        let dir = std::env::temp_dir().join("gmd_test_walk");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("test.md"), "# Hello").unwrap();
        fs::write(dir.join("test.txt"), "not markdown").unwrap();

        let files = walk_markdown_files(&dir);
        assert_eq!(files.len(), 1);
        assert!(files[0].path.extension().unwrap() == "md");

        let _ = fs::remove_dir_all(&dir);
    }
}
