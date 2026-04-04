use crate::output::{print_result, SearchResult};
use crate::parser::markdown::{parse_markdown, MdElement};
use crate::parser::walker::{walk_markdown_files, MdFile};
use colored::Colorize;
use std::collections::HashMap;
use std::path::Path;

pub fn run_link(path: &Path, sub: &str, target: Option<&str>) {
    let files = walk_markdown_files(path);

    match sub {
        "list" => link_list(&files, path),
        "broken" => link_broken(&files, path),
        "back" => {
            if let Some(t) = target {
                link_backlinks(&files, path, t);
            } else {
                eprintln!("Usage: gmd link back <FILE>");
            }
        }
        _ => {
            eprintln!("Unknown subcommand: {}. Use: list, broken, back", sub);
        }
    }
}

fn link_list(files: &[MdFile], root: &Path) {
    let mut count = 0;
    for file in files {
        let elements = parse_markdown(&file.content);
        for el in &elements {
            match el {
                MdElement::Link { text, url, line } => {
                    let rel = file.path.strip_prefix(root).unwrap_or(&file.path);
                    let kind = if url.starts_with("http") {
                        "EXT"
                    } else {
                        "INT"
                    };
                    print_result(&SearchResult {
                        file: rel.display().to_string(),
                        line: *line,
                        content: format!("[{}] [{}]({})", kind, text, url),
                    });
                    count += 1;
                }
                MdElement::WikiLink { target, line } => {
                    let rel = file.path.strip_prefix(root).unwrap_or(&file.path);
                    print_result(&SearchResult {
                        file: rel.display().to_string(),
                        line: *line,
                        content: format!("[WIKI] [[{}]]", target),
                    });
                    count += 1;
                }
                _ => {}
            }
        }
    }
    println!("\n{} link(s) found.", count);
}

fn link_broken(files: &[MdFile], root: &Path) {
    let mut count = 0;

    let md_files: HashMap<String, bool> = files
        .iter()
        .map(|f| {
            let rel = f.path.strip_prefix(root).unwrap_or(&f.path);
            (rel.display().to_string(), true)
        })
        .collect();

    for file in files {
        let elements = parse_markdown(&file.content);
        let file_dir = file.path.parent().unwrap_or(root);

        for el in &elements {
            match el {
                MdElement::Link { text, url, line } => {
                    if url.starts_with("http://") || url.starts_with("https://") {
                        match check_url(url) {
                            Ok(false) => {
                                let rel = file.path.strip_prefix(root).unwrap_or(&file.path);
                                print_result(&SearchResult {
                                    file: rel.display().to_string(),
                                    line: *line,
                                    content: format!("{} [{}]({})", "BROKEN".red(), text, url),
                                });
                                count += 1;
                            }
                            Err(_) => {
                                let rel = file.path.strip_prefix(root).unwrap_or(&file.path);
                                print_result(&SearchResult {
                                    file: rel.display().to_string(),
                                    line: *line,
                                    content: format!("{} [{}]({})", "ERROR".red(), text, url),
                                });
                                count += 1;
                            }
                            _ => {}
                        }
                    } else if !url.starts_with('#') {
                        let link_path = file_dir.join(url.split('#').next().unwrap_or(url));
                        if !link_path.exists() {
                            let rel = file.path.strip_prefix(root).unwrap_or(&file.path);
                            print_result(&SearchResult {
                                file: rel.display().to_string(),
                                line: *line,
                                content: format!("{} [{}]({})", "BROKEN".red(), text, url),
                            });
                            count += 1;
                        }
                    }
                }
                MdElement::WikiLink { target, line } => {
                    let found = md_files.keys().any(|p| p.contains(target.as_str()));
                    if !found {
                        let rel = file.path.strip_prefix(root).unwrap_or(&file.path);
                        print_result(&SearchResult {
                            file: rel.display().to_string(),
                            line: *line,
                            content: format!("{} [[{}]]", "BROKEN".red(), target),
                        });
                        count += 1;
                    }
                }
                _ => {}
            }
        }
    }

    if count == 0 {
        println!("{}", "No broken links found.".green());
    } else {
        println!("\n{} broken link(s) found.", count);
    }
}

fn link_backlinks(files: &[MdFile], root: &Path, target: &str) {
    let mut count = 0;
    let target_lower = target.to_lowercase();

    for file in files {
        let elements = parse_markdown(&file.content);
        for el in &elements {
            match el {
                MdElement::Link { url, line, .. } => {
                    if url.to_lowercase().contains(&target_lower) {
                        let rel = file.path.strip_prefix(root).unwrap_or(&file.path);
                        print_result(&SearchResult {
                            file: rel.display().to_string(),
                            line: *line,
                            content: format!("links to: {}", url),
                        });
                        count += 1;
                    }
                }
                MdElement::WikiLink { target: t, line } => {
                    if t.to_lowercase().contains(&target_lower) {
                        let rel = file.path.strip_prefix(root).unwrap_or(&file.path);
                        print_result(&SearchResult {
                            file: rel.display().to_string(),
                            line: *line,
                            content: format!("links to: [[{}]]", t),
                        });
                        count += 1;
                    }
                }
                _ => {}
            }
        }
    }

    if count == 0 {
        println!("No backlinks found for '{}'.", target);
    } else {
        println!("\n{} backlink(s) found.", count);
    }
}

fn check_url(url: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    let resp = client.head(url).send()?;
    Ok(resp.status().is_success() || resp.status().is_redirection())
}
