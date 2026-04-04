use crate::output::{print_result, SearchResult};
use crate::parser::markdown::{parse_markdown, MdElement};
use crate::parser::walker::{walk_all_files, walk_markdown_files, MdFile};
use colored::Colorize;
use std::collections::HashSet;
use std::path::Path;

pub fn run_img(path: &Path, sub: &str) {
    let files = walk_markdown_files(path);

    match sub {
        "list" => img_list(&files, path),
        "broken" => img_broken(&files, path),
        "orphan" => img_orphan(&files, path),
        _ => {
            eprintln!("Unknown subcommand: {}. Use: list, broken, orphan", sub);
        }
    }
}

fn img_list(files: &[MdFile], root: &Path) {
    let mut count = 0;
    for file in files {
        let elements = parse_markdown(&file.content);
        for el in &elements {
            if let MdElement::Image { alt, url, line } = el {
                let rel = file.path.strip_prefix(root).unwrap_or(&file.path);
                print_result(&SearchResult {
                    file: rel.display().to_string(),
                    line: *line,
                    content: format!("![{}]({})", alt, url),
                });
                count += 1;
            }
        }
    }
    println!("\n{} image(s) found.", count);
}

fn img_broken(files: &[MdFile], root: &Path) {
    let mut count = 0;
    for file in files {
        let elements = parse_markdown(&file.content);
        let file_dir = file.path.parent().unwrap_or(root);
        for el in &elements {
            if let MdElement::Image { alt, url, line } = el {
                if url.starts_with("http://") || url.starts_with("https://") {
                    continue;
                }
                let img_path = if Path::new(url).is_absolute() {
                    Path::new(url).to_path_buf()
                } else {
                    file_dir.join(url)
                };
                if !img_path.exists() {
                    let rel = file.path.strip_prefix(root).unwrap_or(&file.path);
                    print_result(&SearchResult {
                        file: rel.display().to_string(),
                        line: *line,
                        content: format!("{} ![{}]({})", "BROKEN".red(), alt, url),
                    });
                    count += 1;
                }
            }
        }
    }
    if count == 0 {
        println!("{}", "No broken images found.".green());
    } else {
        println!("\n{} broken image(s) found.", count);
    }
}

fn img_orphan(files: &[MdFile], root: &Path) {
    let image_extensions: HashSet<&str> =
        ["png", "jpg", "jpeg", "gif", "svg", "webp", "bmp", "ico"]
            .iter()
            .copied()
            .collect();

    let mut referenced: HashSet<String> = HashSet::new();
    for file in files {
        let elements = parse_markdown(&file.content);
        let file_dir = file.path.parent().unwrap_or(root);
        for el in &elements {
            if let MdElement::Image { url, .. } = el {
                if !url.starts_with("http://") && !url.starts_with("https://") {
                    let resolved = if Path::new(url).is_absolute() {
                        Path::new(url).to_path_buf()
                    } else {
                        file_dir.join(url)
                    };
                    if let Ok(canonical) = resolved.canonicalize() {
                        referenced.insert(canonical.display().to_string());
                    }
                }
            }
        }
    }

    let all_files = walk_all_files(root);
    let mut count = 0;
    for file_path in &all_files {
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            if image_extensions.contains(ext.to_lowercase().as_str()) {
                if let Ok(canonical) = file_path.canonicalize() {
                    if !referenced.contains(&canonical.display().to_string()) {
                        let rel = file_path.strip_prefix(root).unwrap_or(file_path);
                        println!("  {} {}", "ORPHAN".yellow(), rel.display());
                        count += 1;
                    }
                }
            }
        }
    }

    if count == 0 {
        println!("{}", "No orphan images found.".green());
    } else {
        println!("\n{} orphan image(s) found.", count);
    }
}
