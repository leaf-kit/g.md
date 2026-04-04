<p align="center">
  <img src="images/logo.png" alt="gmd logo" width="240" />
</p>

# gmd — **G**rep **M**ark**d**own

[![Release](https://img.shields.io/github/v/release/leaf-kit/g.md?label=release)](https://github.com/leaf-kit/g.md/releases/latest)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Homebrew](https://img.shields.io/badge/homebrew-leaf--kit%2Fgmd-yellow.svg)](https://github.com/leaf-kit/homebrew-gmd)
[![Homebrew install](https://img.shields.io/badge/brew%20install-gmd-success.svg)](https://github.com/leaf-kit/homebrew-gmd)
[![pipe](https://img.shields.io/badge/pipe-grep%20%7C%20awk%20%7C%20jq-blue.svg)]()
[![AI Agent](https://img.shields.io/badge/agent--scan-claude%20%7C%20codex%20%7C%20copilot%20%7C%20cursor-purple.svg)]()

Fast, structure-aware Markdown search & analysis tool for the terminal.

> **v0.1.0 Released** — [GitHub Release](https://github.com/leaf-kit/g.md/releases/tag/v0.1.0) | [Homebrew Tap](https://github.com/leaf-kit/homebrew-gmd)
>
> ```bash
> brew tap leaf-kit/gmd && brew install gmd
> ```

**gmd** searches across Markdown documents with precision — filter by headings, code blocks, links, bold text, or quotes. It tracks todos, validates images and links, analyzes tags and frontmatter, and generates AI-ready prompt context. All from a single CLI.

A must-have terminal tool for **prompt engineers** managing large Markdown knowledge bases, **developers** maintaining documentation, and **writers** organizing notes across hundreds of files.

## Why "gmd"?

**gmd** stands for **Grep Markdown**. Just like `grep` searches text, **gmd** searches Markdown — but understands its structure. It knows the difference between a heading, a code block, a link, and a tag. Search with precision, not just patterns.

**gmd** is a friend of **leaf** and **bark**. While bark renders Markdown beautifully in the terminal, gmd digs into the content and finds exactly what you need.

> *Don't just search text. Search meaning.*

## Features

- Full **Markdown-aware search** — filter by headings, code blocks, links, bold, quotes
- **Todo management** — track `- [ ]` / `- [x]` checkboxes across all documents
- **Asset validation** — find broken images and orphan files
- **Link checking** — detect 404 URLs and find backlinks
- **Tag analytics** — list, rank, and search `#hashtags` across documents
- **Frontmatter inspection** — audit YAML metadata completeness
- **AI prompt generation** — extract context with surrounding lines for AI input
- **AI agent scanner** — detect Claude, Codex, Copilot, Cursor, Windsurf, Aider, Cline, and more
- **Export** — output as Markdown or JSON for integration
- **Statistics** — word counts, reading time, activity patterns
- Fast startup — written in Rust, optimized with LTO

## Installation

### Homebrew (macOS)

```bash
brew tap leaf-kit/gmd
brew install gmd
```

### Build from Source

```bash
git clone https://github.com/leaf-kit/g.md.git
cd g.md
cargo build --release
cp target/release/gmd /usr/local/bin/
```

Or use the build script:

```bash
./build.sh
```

## Update

### Homebrew

```bash
brew upgrade gmd
```

### From Source

```bash
git pull origin main
cargo build --release
cp target/release/gmd /usr/local/bin/
```

## Uninstall

### Homebrew

```bash
brew uninstall gmd
brew untap leaf-kit/gmd
```

### Manual (source install)

```bash
rm /usr/local/bin/gmd
```

## Usage

```
% gmd
A blazingly fast CLI tool for searching, analyzing, and managing
Markdown documents. Built with Rust for speed and safety.

gmd helps you search todos, find content, check assets,
manage tags, export data, and scan AI agent configurations.

Usage: gmd [OPTIONS] <COMMAND>

Commands:
  todo    Manage markdown checkboxes and tasks
  find    Search markdown content with precision filters
  img     Check and manage images in documents
  link    Analyze and validate links
  tag     Manage hashtags across documents
  meta    Inspect frontmatter metadata
  export  Export search results in various formats
  prompt  Generate AI-ready prompt context
  path    List file paths only (no content)
  stat    Show document statistics
  claude  Scan AI coding agent configurations
  codex   Scan OpenAI Codex agent configuration
  agents  Scan all AI agent configurations
  ui      Interactive TUI exploration mode
  clean   Find and suggest cleanup for duplicates and empty files
  help    Print this message or the help of the given subcommand(s)

Options:
  -p, --path <PATH>  Set working directory (default: current directory)
  -v, --verbose      Enable verbose output
  -q, --quiet        Suppress non-essential output
  -h, --help         Print help (see a summary with '-h')
  -V, --version      Print version

Discussion:
    gmd is your markdown companion for navigating large document
    collections. It parses frontmatter, tracks todos, validates links
    and images, and can even prepare context for AI prompts.

    Get started with `gmd stat` to see an overview of your documents,
    or `gmd find <QUERY>` to search across all markdown files.
```

## Commands & Output Examples

All examples below are actual outputs from running `gmd` against the included `tree/` sample documents (10 Markdown files across `docs/`, `notes/`, and `projects/`).

### 1. Task Management (`todo`)

Search and filter Markdown checkboxes (`- [ ]` / `- [x]`) across all documents.

```
% gmd todo
  projects/sprint-backlog.md:18 - [ ] Migrate user service to Rust
  projects/sprint-backlog.md:19 - [ ] Write integration tests for API gateway
  projects/sprint-backlog.md:20 - [ ] Set up **Prometheus** monitoring
  projects/sprint-backlog.md:21 - [ ] Fix broken image paths in documentation
  docs/deployment.md:51 - [ ] Update Kubernetes manifests for v2.2
  docs/deployment.md:52 - [ ] Configure auto-scaling rules
  notes/meeting-2026-03.md:20 - [ ] Alice: Draft migration plan by March 12
  notes/meeting-2026-03.md:21 - [ ] Bob: Evaluate gRPC frameworks
  notes/ideas.md:10 - [ ] Fuzzy search with Levenshtein distance
  notes/daily-log.md:23 - [ ] Optimize for large files (>10MB)
  ...

37 task(s) found.
```

```
% gmd todo done
  projects/sprint-backlog.md:22 - [x] Create database migration scripts
  projects/sprint-backlog.md:23 - [x] Deploy staging environment
  projects/tech-debt.md:18 - [x] Migrate from `moment.js` to `chrono`
  docs/deployment.md:54 - [x] Write deployment documentation
  notes/meeting-2026-03.md:23 - [x] All: Review architecture document
  notes/ideas.md:13 - [x] Case-insensitive search
  notes/daily-log.md:21 - [x] Implement code block parser
  ...

19 task(s) found.
```

```
% gmd todo migration
  projects/sprint-backlog.md:36 - [ ] Write blog post about migration
  notes/meeting-2026-03.md:20 - [ ] Alice: Draft migration plan by March 12

2 task(s) found.
```

| Command | Description |
|---------|-------------|
| `gmd todo` | List all pending tasks |
| `gmd todo done` | Show completed tasks only |
| `gmd todo today` | Tasks from today's modified files |
| `gmd todo week` | Tasks from this week's modified files |
| `gmd todo <QUERY>` | Search tasks by keyword |

### 2. Precision Search (`find`)

Search document content with structure-aware filters.

```
% gmd find Rust
  projects/sprint-backlog.md:10 - Complete **Rust migration** for core services
  projects/sprint-backlog.md:18 - [ ] Migrate user service to Rust
  docs/architecture.md:22 | Backend | Rust (Actix-web) |
  README.md:21 - [Rust Language](https://www.rust-lang.org)
  notes/meeting-2026-03.md:14 - Migrate to **Rust** backend by end of Q2
  notes/meeting-2026-03.md:46 > "Rust gives us the safety guarantees we need for production." — Bob
  CLAUDE.md:3 This is a Rust project for a Markdown search CLI tool.
  ...

13 match(es) found.
```

```
% gmd find head API
  docs/api-guide.md:7 # API Guide

1 match(es) found.
```

```
% gmd find code query
  notes/daily-log.md:12 [rust] fn search_code_blocks(content: &str, query: &str) -> Vec<Match> {
  notes/daily-log.md:15 [rust]         .filter(|b| b.content.contains(query))

2 match(es) found.
```

```
% gmd find link api
  projects/sprint-backlog.md:28 [API Guide](../docs/api-guide.md)
  docs/architecture.md:46 [API Guide](api-guide.md)
  README.md:13 [API Documentation](docs/api-guide.md)
  notes/meeting-2026-03.md:55 [[API Guide]]

4 match(es) found.
```

```
% gmd find bold Rust
  projects/sprint-backlog.md:10 **Rust migration**
  notes/meeting-2026-03.md:14 **Rust**

2 match(es) found.
```

```
% gmd find quote safety
  notes/meeting-2026-03.md:46 > "Rust gives us the safety guarantees we need for production." — Bob

1 match(es) found.
```

| Command | Description |
|---------|-------------|
| `gmd find <QUERY>` | Full-text search across all files |
| `gmd find head <QUERY>` | Search within headings (`# Heading`) |
| `gmd find code <QUERY>` | Search inside code blocks only |
| `gmd find link <QUERY>` | Search link URLs and wiki links |
| `gmd find bold <QUERY>` | Search bold text (`**bold**`) |
| `gmd find quote <QUERY>` | Search blockquotes (`> quote`) |

### 3. Asset & Link Check (`img`, `link`)

Validate images and links for integrity.

```
% gmd img list
  projects/sprint-backlog.md:51 ![Burndown Chart](../images/burndown.png)
  projects/sprint-backlog.md:52 ![Velocity Graph](../images/velocity.png)
  docs/architecture.md:50 ![System Diagram](../images/architecture.png)
  README.md:27 ![Project Logo](images/logo.png)
  README.md:28 ![Missing Banner](images/banner.png)

5 image(s) found.
```

```
% gmd img broken
  projects/sprint-backlog.md:51 BROKEN ![Burndown Chart](../images/burndown.png)
  projects/sprint-backlog.md:52 BROKEN ![Velocity Graph](../images/velocity.png)
  docs/architecture.md:50 BROKEN ![System Diagram](../images/architecture.png)
  README.md:28 BROKEN ![Missing Banner](images/banner.png)

4 broken image(s) found.
```

```
% gmd img orphan
  ORPHAN images/orphan-unused.png

1 orphan image(s) found.
```

```
% gmd link list
  projects/sprint-backlog.md:28 [INT] [API Guide](../docs/api-guide.md)
  projects/sprint-backlog.md:31 [INT] [Architecture Overview](../docs/architecture.md)
  projects/tech-debt.md:32 [WIKI] [[Sprint Backlog]]
  docs/architecture.md:46 [INT] [API Guide](api-guide.md)
  docs/architecture.md:48 [WIKI] [[Sprint Backlog]]
  README.md:17 [INT] [Broken Link Example](docs/nonexistent.md)
  README.md:21 [EXT] [Rust Language](https://www.rust-lang.org)
  README.md:23 [EXT] [Broken External](https://example.invalid/broken-page)
  notes/meeting-2026-03.md:55 [WIKI] [[API Guide]]
  ...

19 link(s) found.
```

```
% gmd link broken
  projects/tech-debt.md:32 BROKEN [[Sprint Backlog]]
  docs/architecture.md:48 BROKEN [[Sprint Backlog]]
  docs/api-guide.md:73 BROKEN [[Architecture Overview]]
  README.md:17 BROKEN [Broken Link Example](docs/nonexistent.md)
  README.md:23 ERROR [Broken External](https://example.invalid/broken-page)
  notes/meeting-2026-03.md:55 BROKEN [[API Guide]]
  notes/ideas.md:38 BROKEN [[Daily Log]]

7 broken link(s) found.
```

```
% gmd link back architecture
  projects/sprint-backlog.md:31 links to: ../docs/architecture.md
  docs/api-guide.md:73 links to: [[Architecture Overview]]
  README.md:14 links to: docs/architecture.md
  notes/meeting-2026-03.md:56 links to: ../docs/architecture.md

4 backlink(s) found.
```

| Command | Description |
|---------|-------------|
| `gmd img list` | List all images in documents |
| `gmd img broken` | Find broken image references |
| `gmd img orphan` | Find unused image files |
| `gmd link list` | List all internal/external links |
| `gmd link broken` | Detect broken URLs (404) |
| `gmd link back <FILE>` | Find backlinks to a document |

### 4. Tag & Metadata (`tag`, `meta`)

Manage hashtags and frontmatter metadata.

```
% gmd tag list
  #api                            (1)
  #architecture                   (1)
  #backlog                        (1)
  #brainstorm                     (1)
  #daily                          (1)
  #deployment                     (1)
  #development                    (1)
  #devops                         (1)
  #documentation                  (1)
  #feature                        (1)
  #gmd                            (1)
  #ideas                          (1)
  #kubernetes                     (1)
  #log                            (1)
  #meeting                        (1)
  #microservices                  (1)
  #planning                       (1)
  #project                        (1)
  #project-management             (1)
  #refactoring                    (1)
  #rest                           (1)
  #roadmap                        (1)
  #rust                           (2)
  #security                       (1)
  #sprint                         (1)
  #system-design                  (1)
  #tech-debt                      (1)
  #workspace                      (1)

28 unique tag(s) found.
```

```
% gmd tag top
  Rank  Tag                            Count
  ---------------------------------------------
  1     #rust                          2
  2     #security                      1
  3     #architecture                  1
  4     #api                           1
  5     #sprint                        1
  ...
```

```
% gmd tag rust
  projects/sprint-backlog.md:54 #rust
  notes/meeting-2026-03.md:58 #rust

2 occurrence(s) of #rust found.
```

```
% gmd meta list
  Field                          Count
  ----------------------------------------
  tags                           9
  title                          9
  author                         4
  version                        1
  date                           1
  priority                       1
  status                         1
  sprint                         1

8 unique field(s) across 10 file(s).
```

```
% gmd meta missing
  PARTIAL docs/deployment.md (empty: author)
  NO META CLAUDE.md

2 document(s) with missing/empty metadata.
```

| Command | Description |
|---------|-------------|
| `gmd tag list` | List all tags with frequency |
| `gmd tag top` | Show most used tags ranked |
| `gmd tag <NAME>` | Find files with a specific tag |
| `gmd meta list` | List all frontmatter fields |
| `gmd meta missing` | Find documents with missing metadata |

### 5. Export & AI Prompt (`export`, `prompt`)

Export data and generate AI-ready context.

```
% gmd prompt monitoring
You are analyzing a collection of Markdown documents.
Search query: "monitoring"

---

## Relevant Context

### File: projects/sprint-backlog.md

    L18: - [ ] Migrate user service to Rust
    L19: - [ ] Write integration tests for API gateway
>>> L20: - [ ] Set up **Prometheus** monitoring
    L21: - [ ] Fix broken image paths in documentation

### File: docs/deployment.md

    L51: - [ ] Update Kubernetes manifests for v2.2
    L52: - [ ] Configure auto-scaling rules
>>> L53: - [ ] Set up monitoring dashboards
    L54: - [x] Write deployment documentation

### File: notes/meeting-2026-03.md

    L14: - Migrate to **Rust** backend by end of Q2
    L15: - Adopt **gRPC** for inter-service communication
>>> L16: - Budget approved for new **monitoring** tools
    ...
>>> L22: - [ ] Charlie: Research monitoring solutions (Datadog vs Grafana)

---

Based on the above context from the Markdown documents, please provide your analysis.
```

```
% gmd export json | head -20
[
  {
    "file": "projects/sprint-backlog.md",
    "headings": [
      {
        "level": 1,
        "line": 6,
        "text": "Sprint Backlog — Q2 2026, Sprint 1"
      },
      {
        "level": 2,
        "line": 8,
        "text": "Goals"
      },
      ...
    ],
    "links": [ ... ]
  },
  ...
]
```

```
% gmd path
projects/sprint-backlog.md
projects/tech-debt.md
docs/architecture.md
docs/api-guide.md
docs/deployment.md
README.md
notes/meeting-2026-03.md
notes/ideas.md
notes/daily-log.md
CLAUDE.md

10 file(s).
```

| Command | Description |
|---------|-------------|
| `gmd export md` | Merge results into one Markdown file |
| `gmd export json` | Export as JSON for integration |
| `gmd prompt <QUERY>` | Generate AI prompt with context |
| `gmd path` | List file paths only |

### 6. System & Statistics (`stat`, `clean`)

Overview and maintenance.

```
% gmd stat
  gmd Statistics
  ========================================
  Documents:                10
  Total size:               10.2 KB
  Total lines:              470
  Total words:              1,532
  Total characters:         10,427
  ----------------------------------------
  Avg words/doc:            153
  Avg lines/doc:            47
```

```
% gmd stat time
  Reading Time & Activity
  ==================================================
  Total reading time:                 7 min (1,532 words)

  File                                        Words     Time
  ------------------------------------------------------------
  notes/meeting-2026-03.md                      211     1 min
  projects/sprint-backlog.md                    208     1 min
  docs/api-guide.md                             187     0 min
  notes/daily-log.md                            183     0 min
  notes/ideas.md                                172     0 min
  docs/architecture.md                          164     0 min
  projects/tech-debt.md                         159     0 min
  docs/deployment.md                            142     0 min
  README.md                                      72     0 min
  CLAUDE.md                                      34     0 min

  Activity by modification date:
  Mon                       (0)
  Tue                       (0)
  Wed                       (0)
  Thu                       (0)
  Fri                       (0)
  Sat ████████████████████  (10)
  Sun                       (0)
```

```
% gmd clean
  gmd Clean Report
  ==================================================

  Empty files:
    None found.

  Potential duplicates (same content):
    None found.
  Very short files (< 50 chars):
    None found.

  --------------------------------------------------
  All clean! No issues found.
```

| Command | Description |
|---------|-------------|
| `gmd stat` | Document count, size, word stats |
| `gmd stat time` | Reading time and activity analysis |
| `gmd clean` | Detect empty/duplicate/short files |

### 7. AI Agent Scanner (`claude`, `codex`, `agents`)

Detect and display AI coding agent configurations in tree format with status indicators.

```
% gmd claude
  AI Coding Agent Configuration Scanner
  ═══════════════════════════════════════════════════════

  Claude Code [READY]
  Anthropic Claude Code - AI coding assistant

    ├──  ✓ CLAUDE.md (205 B)
    ├──  ○ .claude/ [optional]
    ├──  ○ .claude/settings.json [optional]
    ├──  ○ .claude/commands/ [optional]
    └──  ○ .claude/commands.md [optional]
```

```
% gmd agents
  AI Coding Agent Configuration Scanner
  ═══════════════════════════════════════════════════════

  Claude Code [READY]
  Anthropic Claude Code - AI coding assistant

    ├──  ✓ CLAUDE.md (205 B)
    ├──  ○ .claude/ [optional]
    ├──  ○ .claude/settings.json [optional]
    ├──  ○ .claude/commands/ [optional]
    └──  ○ .claude/commands.md [optional]

  OpenAI Codex [NOT FOUND]
  OpenAI Codex CLI - AI coding agent

    ├──  ✗ AGENTS.md [REQUIRED]
    ├──  ○ .codex/ [optional]
    ├──  ○ .codex/config.json [optional]
    └──  ○ codex.md [optional]

  GitHub Copilot [NOT FOUND]
  Cursor [NOT FOUND]
  Windsurf (Codeium) [NOT FOUND]
  Aider [NOT FOUND]
  Continue.dev [NOT FOUND]
  Cline [NOT FOUND]
  Amazon Q Developer [NOT FOUND]
  Gemini Code Assist [NOT FOUND]
```

| Command | Description |
|---------|-------------|
| `gmd claude` | Scan for Claude Code configuration |
| `gmd codex` | Scan for OpenAI Codex configuration |
| `gmd agents` | Scan all known AI agent configs |

Supported agents: Claude Code, OpenAI Codex, GitHub Copilot, Cursor, Windsurf (Codeium), Aider, Continue.dev, Cline, Amazon Q Developer, Gemini Code Assist.

## Pipe Integration (`|`)

gmd outputs plain text to stdout, so it works naturally with standard Unix tools via pipe. Below are tested, practical scenarios.

### Useful Pipe Recipes

**Filter search results with `grep`** — find "Rust" mentions only in meeting notes:

```
% gmd find Rust | grep "meeting"
  notes/meeting-2026-03.md:14 - Migrate to **Rust** backend by end of Q2
  notes/meeting-2026-03.md:37 Projected (Rust):
  notes/meeting-2026-03.md:46 > "Rust gives us the safety guarantees we need for production." — Bob
  notes/meeting-2026-03.md:58 #meeting #planning #rust
```

**Count pending tasks** — quick total with `grep -c`:

```
% gmd todo | grep -c "\- \[ \]"
37
```

**Group tasks by file** — see which files have the most todos:

```
% gmd todo | grep "\- \[ \]" | awk -F: '{print $1}' | sort | uniq -c | sort -rn
  10   projects/sprint-backlog.md
   9   notes/ideas.md
   8   projects/tech-debt.md
   5   notes/meeting-2026-03.md
   3   docs/deployment.md
   2   notes/daily-log.md
```

**Count matches per file** — see where a keyword is concentrated:

```
% gmd find Rust | grep -v "match" | awk -F: '{print $1}' | sort | uniq -c | sort -rn
   4   projects/sprint-backlog.md
   4   notes/meeting-2026-03.md
   2   CLAUDE.md
   1   README.md
   1   notes/daily-log.md
   1   docs/architecture.md
```

**Line count per document** — combine `gmd path` with `xargs wc`:

```
% gmd path 2>/dev/null | sed 's|^|./|' | xargs wc -l
      54 ./projects/sprint-backlog.md
      35 ./projects/tech-debt.md
      52 ./docs/architecture.md
      75 ./docs/api-guide.md
      57 ./docs/deployment.md
      32 ./README.md
      58 ./notes/meeting-2026-03.md
      40 ./notes/ideas.md
      58 ./notes/daily-log.md
       9 ./CLAUDE.md
     508 total
```

**Extract broken image paths only** — clean list for scripting:

```
% gmd img broken 2>&1 | grep "BROKEN" | sed 's/.*!\\[.*\\](//;s/).*//'
../images/burndown.png
../images/velocity.png
../images/architecture.png
images/banner.png
```

**Filter broken internal links only** (exclude external errors):

```
% gmd link broken | grep "BROKEN"
  projects/tech-debt.md:32 BROKEN [[Sprint Backlog]]
  docs/architecture.md:48 BROKEN [[Sprint Backlog]]
  docs/api-guide.md:73 BROKEN [[Architecture Overview]]
  README.md:17 BROKEN [Broken Link Example](docs/nonexistent.md)
  notes/meeting-2026-03.md:55 BROKEN [[API Guide]]
  notes/ideas.md:38 BROKEN [[Daily Log]]
```

**Copy AI prompt to clipboard** (macOS):

```
% gmd prompt "authentication" | pbcopy
```

**Filter JSON export** with `jq` or `python`:

```
% gmd export json | jq '.[].file'
"projects/sprint-backlog.md"
"projects/tech-debt.md"
"docs/architecture.md"
...
```

### Pipe Usage Notes

> **Important caveats when using gmd with pipes:**

1. **Colored output is auto-disabled in pipes.** gmd uses ANSI colors for terminal display, but most programs (grep, awk, etc.) strip or ignore them. The `colored` library auto-detects non-TTY output and disables colors, so pipe output is clean plain text.

2. **Summary lines go to stdout.** Lines like `37 task(s) found.` are included in stdout. Filter them out with `grep -v "found\|match"` or use `2>/dev/null` to suppress stderr-based summaries when applicable.

3. **`gmd path` outputs to both stdout and stderr.** File paths go to stdout, the count summary (`10 file(s).`) goes to stderr. Use `2>/dev/null` to get clean path lists for `xargs`.

4. **Use `-p` flag before piping.** Always set the working directory explicitly with `-p <PATH>` when combining with other tools, so relative paths in output are predictable.

5. **`gmd link broken` may be slow with external URLs.** Each external link is checked via HTTP HEAD request with a 10s timeout. Pipe only after the command completes. For large projects, consider filtering with `grep -v "EXT"` first.

6. **Encoding: gmd outputs UTF-8.** If piping to tools that expect ASCII, non-ASCII characters (Korean tags, etc.) may cause issues. Use `LC_ALL=en_US.UTF-8` if needed.

## Documentation

- [English README](README.md) — This document
- [Korean README](README_ko.md) — Korean documentation

## Feedback & Contributing

This is an early release and there is still room for improvement. We are committed to making gmd better with each update. If you encounter any issues or have suggestions, please open an issue at:

[github.com/leaf-kit/g.md/issues](https://github.com/leaf-kit/g.md/issues)

Your feedback helps us improve. Stay healthy and happy!

## License

[MIT](LICENSE)
