use crate::parser::markdown::{parse_markdown, MdElement};
use crate::parser::walker::walk_markdown_files;
use std::path::Path;

pub fn run_export(path: &Path, format: &str, query: Option<&str>) {
    let files = walk_markdown_files(path);

    match format {
        "md" => export_md(&files, path, query),
        "json" => export_json(&files, path, query),
        _ => {
            eprintln!("Unknown format: {}. Use: md, json", format);
        }
    }
}

fn export_md(files: &[crate::parser::walker::MdFile], root: &Path, query: Option<&str>) {
    let mut output = String::new();
    output.push_str("# gmd Export\n\n");

    for file in files {
        let rel = file.path.strip_prefix(root).unwrap_or(&file.path);
        let mut matched_lines = Vec::new();

        if let Some(q) = query {
            let q_lower = q.to_lowercase();
            for (i, line) in file.content.lines().enumerate() {
                if line.to_lowercase().contains(&q_lower) {
                    matched_lines.push((i + 1, line));
                }
            }
            if matched_lines.is_empty() {
                continue;
            }
        }

        output.push_str(&format!("## {}\n\n", rel.display()));

        if query.is_some() {
            for (line_num, line) in &matched_lines {
                output.push_str(&format!("- L{}: {}\n", line_num, line));
            }
        } else {
            output.push_str(&file.content);
        }
        output.push_str("\n\n---\n\n");
    }

    print!("{}", output);
}

fn export_json(files: &[crate::parser::walker::MdFile], root: &Path, query: Option<&str>) {
    let mut results: Vec<serde_json::Value> = Vec::new();

    for file in files {
        let rel = file.path.strip_prefix(root).unwrap_or(&file.path);
        let elements = parse_markdown(&file.content);

        let mut file_data = serde_json::json!({
            "file": rel.display().to_string(),
        });

        if let Some(q) = query {
            let q_lower = q.to_lowercase();
            let matches: Vec<serde_json::Value> = file
                .content
                .lines()
                .enumerate()
                .filter(|(_, line)| line.to_lowercase().contains(&q_lower))
                .map(|(i, line)| {
                    serde_json::json!({
                        "line": i + 1,
                        "content": line,
                    })
                })
                .collect();
            if matches.is_empty() {
                continue;
            }
            file_data["matches"] = serde_json::Value::Array(matches);
        }

        let headings: Vec<serde_json::Value> = elements
            .iter()
            .filter_map(|el| {
                if let MdElement::Heading { level, text, line } = el {
                    Some(serde_json::json!({
                        "level": level,
                        "text": text,
                        "line": line,
                    }))
                } else {
                    None
                }
            })
            .collect();
        file_data["headings"] = serde_json::Value::Array(headings);

        let links: Vec<serde_json::Value> = elements
            .iter()
            .filter_map(|el| {
                if let MdElement::Link { text, url, line } = el {
                    Some(serde_json::json!({
                        "text": text,
                        "url": url,
                        "line": line,
                    }))
                } else {
                    None
                }
            })
            .collect();
        file_data["links"] = serde_json::Value::Array(links);

        results.push(file_data);
    }

    let output = serde_json::to_string_pretty(&results).unwrap_or_default();
    println!("{}", output);
}

pub fn run_path_only(path: &Path) {
    let files = walk_markdown_files(path);
    for file in &files {
        let rel = file.path.strip_prefix(path).unwrap_or(&file.path);
        println!("{}", rel.display());
    }
    eprintln!("\n{} file(s).", files.len());
}
