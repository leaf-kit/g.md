use crate::output::{print_result, SearchResult};
use crate::parser::markdown::{parse_markdown, MdElement};
use crate::parser::walker::{walk_markdown_files, MdFile};
use std::path::Path;

/// Controls how file paths are displayed in search results.
pub enum PathMode {
    /// Default: strip root prefix (e.g. `notes.md`)
    Default,
    /// Full absolute path (e.g. `/home/user/docs/notes.md`)
    Full,
    /// Relative with `./` prefix (e.g. `./notes.md`)
    Relative,
}

fn resolve_display_path(file: &Path, root: &Path, mode: &PathMode) -> String {
    match mode {
        PathMode::Full => {
            // Return canonical absolute path
            file.canonicalize()
                .unwrap_or_else(|_| file.to_path_buf())
                .display()
                .to_string()
        }
        PathMode::Relative => {
            let rel = file.strip_prefix(root).unwrap_or(file);
            format!("./{}", rel.display())
        }
        PathMode::Default => {
            let rel = file.strip_prefix(root).unwrap_or(file);
            rel.display().to_string()
        }
    }
}

pub fn run_find(path: &Path, sub: Option<&str>, query: &str, mode: PathMode) {
    let files = walk_markdown_files(path);

    match sub {
        Some("head") => find_in_headings(&files, path, query, &mode),
        Some("code") => find_in_code(&files, path, query, &mode),
        Some("link") => find_in_links(&files, path, query, &mode),
        Some("bold") => find_in_bold(&files, path, query, &mode),
        Some("quote") => find_in_quotes(&files, path, query, &mode),
        _ => find_all(&files, path, query, &mode),
    }
}

fn find_all(files: &[MdFile], root: &Path, query: &str, mode: &PathMode) {
    let q = query.to_lowercase();
    let mut count = 0;
    for file in files {
        for (i, line) in file.content.lines().enumerate() {
            if line.to_lowercase().contains(&q) {
                print_result(&SearchResult {
                    file: resolve_display_path(&file.path, root, mode),
                    line: i + 1,
                    content: line.to_string(),
                });
                count += 1;
            }
        }
    }
    print_count(count);
}

fn find_in_headings(files: &[MdFile], root: &Path, query: &str, mode: &PathMode) {
    let q = query.to_lowercase();
    let mut count = 0;
    for file in files {
        let elements = parse_markdown(&file.content);
        for el in &elements {
            if let MdElement::Heading { text, line, level } = el {
                if text.to_lowercase().contains(&q) {
                    print_result(&SearchResult {
                        file: resolve_display_path(&file.path, root, mode),
                        line: *line,
                        content: format!("{} {}", "#".repeat(*level as usize), text),
                    });
                    count += 1;
                }
            }
        }
    }
    print_count(count);
}

fn find_in_code(files: &[MdFile], root: &Path, query: &str, mode: &PathMode) {
    let q = query.to_lowercase();
    let mut count = 0;
    for file in files {
        let elements = parse_markdown(&file.content);
        for el in &elements {
            if let MdElement::CodeBlock {
                content,
                start_line,
                lang,
                ..
            } = el
            {
                for (i, code_line) in content.lines().enumerate() {
                    if code_line.to_lowercase().contains(&q) {
                        print_result(&SearchResult {
                            file: resolve_display_path(&file.path, root, mode),
                            line: start_line + 1 + i,
                            content: format!("[{}] {}", lang, code_line),
                        });
                        count += 1;
                    }
                }
            }
        }
    }
    print_count(count);
}

fn find_in_links(files: &[MdFile], root: &Path, query: &str, mode: &PathMode) {
    let q = query.to_lowercase();
    let mut count = 0;
    for file in files {
        let elements = parse_markdown(&file.content);
        for el in &elements {
            match el {
                MdElement::Link { text, url, line } => {
                    if text.to_lowercase().contains(&q) || url.to_lowercase().contains(&q) {
                        print_result(&SearchResult {
                            file: resolve_display_path(&file.path, root, mode),
                            line: *line,
                            content: format!("[{}]({})", text, url),
                        });
                        count += 1;
                    }
                }
                MdElement::WikiLink { target, line } => {
                    if target.to_lowercase().contains(&q) {
                        print_result(&SearchResult {
                            file: resolve_display_path(&file.path, root, mode),
                            line: *line,
                            content: format!("[[{}]]", target),
                        });
                        count += 1;
                    }
                }
                _ => {}
            }
        }
    }
    print_count(count);
}

fn find_in_bold(files: &[MdFile], root: &Path, query: &str, mode: &PathMode) {
    let q = query.to_lowercase();
    let mut count = 0;
    for file in files {
        let elements = parse_markdown(&file.content);
        for el in &elements {
            if let MdElement::Bold { text, line } = el {
                if text.to_lowercase().contains(&q) {
                    print_result(&SearchResult {
                        file: resolve_display_path(&file.path, root, mode),
                        line: *line,
                        content: format!("**{}**", text),
                    });
                    count += 1;
                }
            }
        }
    }
    print_count(count);
}

fn find_in_quotes(files: &[MdFile], root: &Path, query: &str, mode: &PathMode) {
    let q = query.to_lowercase();
    let mut count = 0;
    for file in files {
        let elements = parse_markdown(&file.content);
        for el in &elements {
            if let MdElement::Quote { text, line } = el {
                if text.to_lowercase().contains(&q) {
                    print_result(&SearchResult {
                        file: resolve_display_path(&file.path, root, mode),
                        line: *line,
                        content: format!("> {}", text),
                    });
                    count += 1;
                }
            }
        }
    }
    print_count(count);
}

fn print_count(count: usize) {
    if count == 0 {
        println!("No matches found.");
    } else {
        println!("\n{} match(es) found.", count);
    }
}
