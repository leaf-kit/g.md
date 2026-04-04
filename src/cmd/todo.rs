use crate::output::{print_result, SearchResult};
use crate::parser::markdown::{parse_markdown, MdElement};
use crate::parser::walker::{walk_markdown_files, MdFile};
use chrono::{Datelike, Local, NaiveDate};
use std::path::Path;

pub fn run_todo(path: &Path, sub: Option<&str>, query: Option<&str>) {
    let files = walk_markdown_files(path);

    match sub {
        Some("done") => show_checkboxes(&files, path, true, query),
        Some("today") => show_today_todos(&files, path),
        Some("week") => show_week_todos(&files, path),
        _ => {
            if let Some(q) = query.or(sub) {
                show_checkboxes_with_query(&files, path, q);
            } else {
                show_checkboxes(&files, path, false, None);
            }
        }
    }
}

fn show_checkboxes(files: &[MdFile], root: &Path, done: bool, query: Option<&str>) {
    let mut count = 0;
    for file in files {
        let elements = parse_markdown(&file.content);
        for el in &elements {
            if let MdElement::Checkbox {
                checked,
                text,
                line,
            } = el
            {
                if *checked == done {
                    if let Some(q) = query {
                        if !text.to_lowercase().contains(&q.to_lowercase()) {
                            continue;
                        }
                    }
                    let rel = file.path.strip_prefix(root).unwrap_or(&file.path);
                    print_result(&SearchResult {
                        file: rel.display().to_string(),
                        line: *line,
                        content: format!("- [{}] {}", if *checked { "x" } else { " " }, text),
                    });
                    count += 1;
                }
            }
        }
    }
    if count == 0 {
        println!(
            "No {} tasks found.",
            if done { "completed" } else { "pending" }
        );
    } else {
        println!("\n{} task(s) found.", count);
    }
}

fn show_checkboxes_with_query(files: &[MdFile], root: &Path, query: &str) {
    show_checkboxes(files, root, false, Some(query));
}

fn show_today_todos(files: &[MdFile], root: &Path) {
    let today = Local::now().date_naive();
    show_date_filtered_todos(files, root, today, today);
}

fn show_week_todos(files: &[MdFile], root: &Path) {
    let today = Local::now().date_naive();
    let weekday = today.weekday().num_days_from_monday();
    let week_start = today - chrono::Duration::days(weekday as i64);
    let week_end = week_start + chrono::Duration::days(6);
    show_date_filtered_todos(files, root, week_start, week_end);
}

fn show_date_filtered_todos(files: &[MdFile], root: &Path, start: NaiveDate, end: NaiveDate) {
    let mut count = 0;
    for file in files {
        if let Some(modified) = file.modified {
            let file_date = modified.date();
            if file_date >= start && file_date <= end {
                let elements = parse_markdown(&file.content);
                for el in &elements {
                    if let MdElement::Checkbox {
                        checked,
                        text,
                        line,
                    } = el
                    {
                        if !checked {
                            let rel = file.path.strip_prefix(root).unwrap_or(&file.path);
                            print_result(&SearchResult {
                                file: rel.display().to_string(),
                                line: *line,
                                content: format!("- [ ] {}", text),
                            });
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    if count == 0 {
        println!("No tasks found for this period.");
    } else {
        println!("\n{} task(s) found.", count);
    }
}
