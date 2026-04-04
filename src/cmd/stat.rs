use crate::parser::walker::walk_markdown_files;
use colored::Colorize;
use std::path::Path;

pub fn run_stat(path: &Path, sub: Option<&str>) {
    let files = walk_markdown_files(path);

    match sub {
        Some("time") => stat_time(&files, path),
        _ => stat_overview(&files, path),
    }
}

fn stat_overview(files: &[crate::parser::walker::MdFile], _root: &Path) {
    let total_files = files.len();
    let mut total_bytes: u64 = 0;
    let mut total_words: u64 = 0;
    let mut total_lines: u64 = 0;
    let mut total_chars: u64 = 0;

    for file in files {
        total_bytes += file.content.len() as u64;
        total_lines += file.content.lines().count() as u64;
        total_words += file.content.split_whitespace().count() as u64;
        total_chars += file.content.chars().count() as u64;
    }

    println!("{}", "  gmd Statistics".bold());
    println!("  {}", "=".repeat(40));
    println!("  {:<25} {}", "Documents:", total_files);
    println!("  {:<25} {}", "Total size:", format_bytes(total_bytes));
    println!("  {:<25} {}", "Total lines:", format_number(total_lines));
    println!("  {:<25} {}", "Total words:", format_number(total_words));
    println!(
        "  {:<25} {}",
        "Total characters:",
        format_number(total_chars)
    );

    if total_files > 0 {
        println!("  {}", "-".repeat(40));
        println!(
            "  {:<25} {}",
            "Avg words/doc:",
            total_words / total_files as u64
        );
        println!(
            "  {:<25} {}",
            "Avg lines/doc:",
            total_lines / total_files as u64
        );
    }
}

fn stat_time(files: &[crate::parser::walker::MdFile], root: &Path) {
    println!("{}", "  Reading Time & Activity".bold());
    println!("  {}", "=".repeat(50));

    let wpm = 200u64; // average reading speed

    let mut file_stats: Vec<(&crate::parser::walker::MdFile, u64)> = files
        .iter()
        .map(|f| {
            let words = f.content.split_whitespace().count() as u64;
            (f, words)
        })
        .collect();

    file_stats.sort_by(|a, b| b.1.cmp(&a.1));

    let total_words: u64 = file_stats.iter().map(|(_, w)| w).sum();
    let total_minutes = total_words / wpm;

    println!(
        "  {:<35} {} min ({} words)",
        "Total reading time:".bold(),
        total_minutes,
        format_number(total_words)
    );
    println!();

    println!("  {:<40} {:>8} {:>8}", "File", "Words", "Time");
    println!("  {}", "-".repeat(60));

    for (file, words) in file_stats.iter().take(20) {
        let rel = file.path.strip_prefix(root).unwrap_or(&file.path);
        let minutes = words / wpm;
        let display = rel.display().to_string();
        let truncated = if display.len() > 38 {
            format!("...{}", &display[display.len() - 35..])
        } else {
            display
        };
        println!("  {:<40} {:>8} {:>5} min", truncated, words, minutes);
    }

    // Activity by day of week
    println!();
    println!("  {}", "Activity by modification date:".bold());
    let mut by_weekday = [0u32; 7];
    for file in files {
        if let Some(modified) = file.modified {
            let wd = modified
                .and_utc()
                .format("%u")
                .to_string()
                .parse::<usize>()
                .unwrap_or(1)
                - 1;
            by_weekday[wd] += 1;
        }
    }
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let max_count = *by_weekday.iter().max().unwrap_or(&1) as f64;
    for (i, day) in days.iter().enumerate() {
        let bar_len = if max_count > 0.0 {
            (by_weekday[i] as f64 / max_count * 20.0) as usize
        } else {
            0
        };
        println!(
            "  {} {} {} ({})",
            day,
            "█".repeat(bar_len),
            " ".repeat(20 - bar_len),
            by_weekday[i]
        );
    }
}

fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}
