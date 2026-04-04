use crate::parser::frontmatter::parse_frontmatter;
use crate::parser::walker::{walk_markdown_files, MdFile};
use colored::Colorize;
use std::collections::HashMap;
use std::path::Path;

pub fn run_meta(path: &Path, sub: &str) {
    let files = walk_markdown_files(path);

    match sub {
        "list" => meta_list(&files, path),
        "missing" => meta_missing(&files, path),
        _ => {
            eprintln!("Unknown subcommand: {}. Use: list, missing", sub);
        }
    }
}

fn meta_list(files: &[MdFile], _root: &Path) {
    let mut all_fields: HashMap<String, usize> = HashMap::new();

    for file in files {
        let (fm, _) = parse_frontmatter(&file.content);
        if let Some(fm) = fm {
            for key in fm.fields.keys() {
                *all_fields.entry(key.clone()).or_insert(0) += 1;
            }
        }
    }

    if all_fields.is_empty() {
        println!("No frontmatter fields found.");
        return;
    }

    let mut sorted: Vec<_> = all_fields.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    println!("  {:<30} Count", "Field");
    println!("  {}", "-".repeat(40));
    for (field, count) in &sorted {
        println!("  {:<30} {}", field, count);
    }
    println!(
        "\n{} unique field(s) across {} file(s).",
        sorted.len(),
        files.len()
    );
}

fn meta_missing(files: &[MdFile], root: &Path) {
    let mut count = 0;
    for file in files {
        let (fm, _) = parse_frontmatter(&file.content);
        let rel = file.path.strip_prefix(root).unwrap_or(&file.path);
        match fm {
            None => {
                println!("  {} {}", "NO META".red(), rel.display());
                count += 1;
            }
            Some(fm) => {
                let empty_fields: Vec<_> = fm
                    .fields
                    .iter()
                    .filter(|(_, v)| {
                        v.is_null() || (v.is_string() && v.as_str().is_some_and(|s| s.is_empty()))
                    })
                    .map(|(k, _)| k.clone())
                    .collect();
                if !empty_fields.is_empty() {
                    println!(
                        "  {} {} (empty: {})",
                        "PARTIAL".yellow(),
                        rel.display(),
                        empty_fields.join(", ")
                    );
                    count += 1;
                }
            }
        }
    }
    if count == 0 {
        println!("{}", "All documents have complete metadata.".green());
    } else {
        println!("\n{} document(s) with missing/empty metadata.", count);
    }
}
