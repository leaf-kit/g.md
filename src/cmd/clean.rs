use crate::parser::walker::walk_markdown_files;
use colored::Colorize;
use std::collections::HashMap;
use std::path::Path;

pub fn run_clean(path: &Path) {
    let files = walk_markdown_files(path);
    let mut issues = 0;

    println!("{}", "  gmd Clean Report".bold());
    println!("  {}", "=".repeat(50));

    // 1. Empty files
    println!("\n  {}", "Empty files:".bold());
    let mut empty_count = 0;
    for file in &files {
        if file.content.trim().is_empty() {
            let rel = file.path.strip_prefix(path).unwrap_or(&file.path);
            println!("    {} {}", "EMPTY".red(), rel.display());
            empty_count += 1;
            issues += 1;
        }
    }
    if empty_count == 0 {
        println!("    {}", "None found.".green());
    }

    // 2. Duplicate content detection (by content hash)
    println!("\n  {}", "Potential duplicates (same content):".bold());
    let mut content_map: HashMap<u64, Vec<&Path>> = HashMap::new();
    for file in &files {
        if file.content.trim().is_empty() {
            continue;
        }
        let hash = simple_hash(&file.content);
        content_map.entry(hash).or_default().push(&file.path);
    }
    let mut dup_count = 0;
    for paths in content_map.values() {
        if paths.len() > 1 {
            for p in paths {
                let rel = p.strip_prefix(path).unwrap_or(p);
                println!("    {} {}", "DUP".yellow(), rel.display());
                issues += 1;
            }
            dup_count += 1;
            println!();
        }
    }
    if dup_count == 0 {
        println!("    {}", "None found.".green());
    }

    // 3. Very short files (< 50 chars of actual content)
    println!("  {}", "Very short files (< 50 chars):".bold());
    let mut short_count = 0;
    for file in &files {
        let trimmed = file.content.trim();
        if !trimmed.is_empty() && trimmed.len() < 50 {
            let rel = file.path.strip_prefix(path).unwrap_or(&file.path);
            println!(
                "    {} {} ({} chars)",
                "SHORT".yellow(),
                rel.display(),
                trimmed.len()
            );
            short_count += 1;
            issues += 1;
        }
    }
    if short_count == 0 {
        println!("    {}", "None found.".green());
    }

    println!("\n  {}", "-".repeat(50));
    if issues == 0 {
        println!("  {}", "All clean! No issues found.".green());
    } else {
        println!("  {} issue(s) found for review.", issues);
    }
}

fn simple_hash(content: &str) -> u64 {
    let mut hash: u64 = 5381;
    for byte in content.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }
    hash
}
