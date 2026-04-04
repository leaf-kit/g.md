use crate::output::{print_result, SearchResult};
use crate::parser::markdown::{parse_markdown, MdElement};
use crate::parser::walker::{walk_markdown_files, MdFile};
use std::collections::HashMap;
use std::path::Path;

pub fn run_tag(path: &Path, sub: Option<&str>, name: Option<&str>) {
    let files = walk_markdown_files(path);

    match sub {
        Some("list") => tag_list(&files, path),
        Some("top") => tag_top(&files),
        _ => {
            if let Some(n) = name.or(sub) {
                tag_find(&files, path, n);
            } else {
                tag_list(&files, path);
            }
        }
    }
}

fn collect_tags(files: &[MdFile]) -> HashMap<String, usize> {
    let mut tags: HashMap<String, usize> = HashMap::new();
    for file in files {
        let elements = parse_markdown(&file.content);
        for el in &elements {
            if let MdElement::Tag { name, .. } = el {
                *tags.entry(name.clone()).or_insert(0) += 1;
            }
        }
    }
    tags
}

fn tag_list(files: &[MdFile], _root: &Path) {
    let tags = collect_tags(files);
    if tags.is_empty() {
        println!("No tags found.");
        return;
    }
    let mut sorted: Vec<_> = tags.iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(b.0));
    for (tag, count) in &sorted {
        println!("  #{:<30} ({})", tag, count);
    }

    // Also show which files contain tags
    println!("\n{} unique tag(s) found.", sorted.len());
}

fn tag_top(files: &[MdFile]) {
    let tags = collect_tags(files);
    if tags.is_empty() {
        println!("No tags found.");
        return;
    }
    let mut sorted: Vec<_> = tags.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    let top = sorted.iter().take(20);
    println!("  {:<5} {:<30} Count", "Rank", "Tag");
    println!("  {}", "-".repeat(45));
    for (i, (tag, count)) in top.enumerate() {
        println!("  {:<5} #{:<29} {}", i + 1, tag, count);
    }
}

fn tag_find(files: &[MdFile], root: &Path, name: &str) {
    let name_lower = name.to_lowercase();
    let mut count = 0;
    for file in files {
        let elements = parse_markdown(&file.content);
        for el in &elements {
            if let MdElement::Tag {
                name: tag_name,
                line,
            } = el
            {
                if tag_name.to_lowercase() == name_lower
                    || tag_name.to_lowercase().contains(&name_lower)
                {
                    let rel = file.path.strip_prefix(root).unwrap_or(&file.path);
                    print_result(&SearchResult {
                        file: rel.display().to_string(),
                        line: *line,
                        content: format!("#{}", tag_name),
                    });
                    count += 1;
                }
            }
        }
    }
    if count == 0 {
        println!("No files found with tag #{}.", name);
    } else {
        println!("\n{} occurrence(s) of #{} found.", count, name);
    }
}
