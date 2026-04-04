use std::fs;
use std::process::Command;

fn gmd_bin() -> String {
    env!("CARGO_BIN_EXE_gmd").to_string()
}

fn setup_test_dir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("gmd_test_{}", name));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    fs::write(
        dir.join("notes.md"),
        r#"---
title: My Notes
tags: [rust, cli]
---
# My Notes

## Tasks
- [ ] Buy groceries
- [x] Write tests
- [ ] Deploy application

## Links
Check [Rust](https://www.rust-lang.org) for details.
See [[Other Page]] for more info.

## Code
```rust
fn main() {
    println!("Hello, world!");
}
```

## Quotes
> This is an important quote

Some text with **bold testing here** in it.

Some text with #rust and #programming tags.

![logo](images/logo.png)
"#,
    )
    .unwrap();

    fs::write(dir.join("empty.md"), "").unwrap();

    fs::write(
        dir.join("project.md"),
        r#"---
title: Project
author:
---
# Project Overview

This project uses **Rust** for performance.

- [ ] Setup CI/CD
- [x] Initialize repository

> Always test before deploying

[Documentation](./notes.md)
![diagram](./arch.png)

#project #rust
"#,
    )
    .unwrap();

    dir
}

#[test]
fn test_todo_list() {
    let dir = setup_test_dir("todo_list");
    let output = Command::new(gmd_bin())
        .args(["todo", "-p", dir.to_str().unwrap()])
        .output()
        .expect("failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Buy groceries"), "stdout: {}", stdout);
    assert!(stdout.contains("Deploy application"), "stdout: {}", stdout);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_todo_done() {
    let dir = setup_test_dir("todo_done");
    let output = Command::new(gmd_bin())
        .args(["todo", "done", "-p", dir.to_str().unwrap()])
        .output()
        .expect("failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Write tests"), "stdout: {}", stdout);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_find_text() {
    let dir = setup_test_dir("find_text");
    let output = Command::new(gmd_bin())
        .args(["find", "Rust", "-p", dir.to_str().unwrap()])
        .output()
        .expect("failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Rust"), "stdout: {}", stdout);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_find_head() {
    let dir = setup_test_dir("find_head");
    let output = Command::new(gmd_bin())
        .args(["find", "head", "Notes", "-p", dir.to_str().unwrap()])
        .output()
        .expect("failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("My Notes"), "stdout: {}", stdout);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_find_code() {
    let dir = setup_test_dir("find_code");
    let output = Command::new(gmd_bin())
        .args(["find", "code", "println", "-p", dir.to_str().unwrap()])
        .output()
        .expect("failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("println"), "stdout: {}", stdout);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_find_bold() {
    let dir = setup_test_dir("find_bold");
    let output = Command::new(gmd_bin())
        .args(["find", "bold", "bold", "-p", dir.to_str().unwrap()])
        .output()
        .expect("failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("bold"), "stdout: {}", stdout);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_find_quote() {
    let dir = setup_test_dir("find_quote");
    let output = Command::new(gmd_bin())
        .args(["find", "quote", "important", "-p", dir.to_str().unwrap()])
        .output()
        .expect("failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("important"), "stdout: {}", stdout);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_find_link() {
    let dir = setup_test_dir("find_link");
    let output = Command::new(gmd_bin())
        .args(["find", "link", "rust-lang", "-p", dir.to_str().unwrap()])
        .output()
        .expect("failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("rust-lang"), "stdout: {}", stdout);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_img_list() {
    let dir = setup_test_dir("img_list");
    let output = Command::new(gmd_bin())
        .args(["img", "list", "-p", dir.to_str().unwrap()])
        .output()
        .expect("failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("logo.png"), "stdout: {}", stdout);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_img_broken() {
    let dir = setup_test_dir("img_broken");
    let output = Command::new(gmd_bin())
        .args(["img", "broken", "-p", dir.to_str().unwrap()])
        .output()
        .expect("failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    // The BROKEN indicator might have ANSI color codes
    assert!(
        stdout.contains("logo.png") || stdout.contains("arch.png"),
        "stdout: {}",
        stdout
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_link_list() {
    let dir = setup_test_dir("link_list");
    let output = Command::new(gmd_bin())
        .args(["link", "list", "-p", dir.to_str().unwrap()])
        .output()
        .expect("failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("rust-lang.org"), "stdout: {}", stdout);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_meta_list() {
    let dir = setup_test_dir("meta_list");
    let output = Command::new(gmd_bin())
        .args(["meta", "list", "-p", dir.to_str().unwrap()])
        .output()
        .expect("failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"), "stdout: {}", stdout);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_meta_missing() {
    let dir = setup_test_dir("meta_missing");
    let output = Command::new(gmd_bin())
        .args(["meta", "missing", "-p", dir.to_str().unwrap()])
        .output()
        .expect("failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("empty.md") || stdout.contains("NO META"),
        "stdout: {}",
        stdout
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_stat() {
    let dir = setup_test_dir("stat");
    let output = Command::new(gmd_bin())
        .args(["stat", "-p", dir.to_str().unwrap()])
        .output()
        .expect("failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Documents:"), "stdout: {}", stdout);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_export_json() {
    let dir = setup_test_dir("export_json");
    let output = Command::new(gmd_bin())
        .args(["export", "json", "-p", dir.to_str().unwrap()])
        .output()
        .expect("failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"file\""), "stdout: {}", stdout);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_path_only() {
    let dir = setup_test_dir("path_only");
    let output = Command::new(gmd_bin())
        .args(["path", "-p", dir.to_str().unwrap()])
        .output()
        .expect("failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stdout.contains("notes.md") || stderr.contains("notes.md"),
        "stdout: {}, stderr: {}",
        stdout,
        stderr
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_prompt() {
    let dir = setup_test_dir("prompt");
    let output = Command::new(gmd_bin())
        .args(["prompt", "Rust", "-p", dir.to_str().unwrap()])
        .output()
        .expect("failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Relevant Context"), "stdout: {}", stdout);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_agents_scan() {
    let dir = setup_test_dir("agents_scan");
    fs::write(dir.join("CLAUDE.md"), "# Project Instructions").unwrap();

    let output = Command::new(gmd_bin())
        .args(["claude", "-p", dir.to_str().unwrap()])
        .output()
        .expect("failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Claude Code"), "stdout: {}", stdout);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_help() {
    let output = Command::new(gmd_bin())
        .args(["--help"])
        .output()
        .expect("failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("gmd"), "stdout: {}", stdout);
    assert!(stdout.contains("todo"), "stdout: {}", stdout);
    assert!(stdout.contains("find"), "stdout: {}", stdout);
}

#[test]
fn test_version() {
    let output = Command::new(gmd_bin())
        .args(["--version"])
        .output()
        .expect("failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0.1.0"), "stdout: {}", stdout);
}
