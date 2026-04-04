use crate::parser::walker::walk_markdown_files;
use std::path::Path;

pub fn run_prompt(path: &Path, query: &str) {
    let files = walk_markdown_files(path);
    let mut output = String::new();

    output.push_str("You are analyzing a collection of Markdown documents.\n");
    output.push_str(&format!("Search query: \"{}\"\n\n", query));
    output.push_str("---\n\n");
    output.push_str("## Relevant Context\n\n");

    let q_lower = query.to_lowercase();
    let mut found = false;

    for file in &files {
        let lines: Vec<&str> = file.content.lines().collect();
        let mut matched_ranges: Vec<(usize, usize)> = Vec::new();

        for (i, line) in lines.iter().enumerate() {
            if line.to_lowercase().contains(&q_lower) {
                let start = i.saturating_sub(3);
                let end = (i + 4).min(lines.len());
                matched_ranges.push((start, end));
            }
        }

        if matched_ranges.is_empty() {
            continue;
        }

        // Merge overlapping ranges
        matched_ranges.sort_by_key(|r| r.0);
        let mut merged = vec![matched_ranges[0]];
        for &(start, end) in &matched_ranges[1..] {
            let last = merged.last_mut().unwrap();
            if start <= last.1 {
                last.1 = last.1.max(end);
            } else {
                merged.push((start, end));
            }
        }

        found = true;
        let rel = file.path.strip_prefix(path).unwrap_or(&file.path);
        output.push_str(&format!("### File: {}\n\n", rel.display()));

        for (start, end) in &merged {
            output.push_str("```\n");
            for (i, line) in lines.iter().enumerate().take(*end).skip(*start) {
                let marker = if line.to_lowercase().contains(&q_lower) {
                    ">>>"
                } else {
                    "   "
                };
                output.push_str(&format!("{} L{}: {}\n", marker, i + 1, line));
            }
            output.push_str("```\n\n");
        }
    }

    if !found {
        output.push_str("No matching content found for the given query.\n");
    }

    output.push_str("---\n\n");
    output.push_str(
        "Based on the above context from the Markdown documents, please provide your analysis.\n",
    );

    print!("{}", output);
}
