mod agent_scan;
mod cmd;
mod output;
mod parser;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// gmd — Grep Markdown
///
/// A blazingly fast CLI tool for searching, analyzing, and managing
/// Markdown documents. Built with Rust for speed and safety.
#[derive(Parser)]
#[command(
    name = "gmd",
    version = env!("CARGO_PKG_VERSION"),
    about = "gmd — Grep Markdown",
    long_about = "gmd — Grep Markdown\n\nA blazingly fast CLI tool for searching, analyzing, and managing\nMarkdown documents. Built with Rust for speed and safety.\n\ngmd helps you search todos, find content, check assets,\nmanage tags, export data, and scan AI agent configurations.",
    after_help = "Discussion:\n    gmd is your markdown companion for navigating large document\n    collections. It parses frontmatter, tracks todos, validates links\n    and images, and can even prepare context for AI prompts.\n\n    Get started with `gmd stat` to see an overview of your documents,\n    or `gmd find <QUERY>` to search across all markdown files."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Set working directory (default: current directory)
    #[arg(short, long, global = true)]
    path: Option<PathBuf>,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Suppress non-essential output
    #[arg(short, long, global = true)]
    quiet: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage markdown checkboxes and tasks
    ///
    /// Search and filter todo items (- [ ] and - [x]) across all
    /// markdown files in the target directory.
    Todo {
        /// Subcommand: done, today, week, or search query
        sub: Option<String>,
        /// Search query for filtering tasks
        query: Option<String>,
    },

    /// Search markdown content with precision filters
    ///
    /// Search across all markdown files with optional filters
    /// for headings, code blocks, links, bold text, or quotes.
    Find {
        /// Filter: head, code, link, bold, quote (or omit for full text)
        filter: String,
        /// Search query
        query: Option<String>,
    },

    /// Check and manage images in documents
    ///
    /// List all images, find broken references, or detect
    /// orphan image files not used by any document.
    Img {
        /// Subcommand: list, broken, orphan
        sub: String,
    },

    /// Analyze and validate links
    ///
    /// List all links, detect broken URLs (404), or find
    /// backlinks pointing to a specific document.
    Link {
        /// Subcommand: list, broken, back
        sub: String,
        /// Target file for backlink search
        target: Option<String>,
    },

    /// Manage hashtags across documents
    ///
    /// List all tags with frequency, show top tags, or find
    /// all files containing a specific tag.
    Tag {
        /// Subcommand: list, top, or tag name
        sub: Option<String>,
        /// Tag name to search
        name: Option<String>,
    },

    /// Inspect frontmatter metadata
    ///
    /// List all YAML frontmatter fields across documents,
    /// or find documents with missing metadata.
    Meta {
        /// Subcommand: list, missing
        sub: String,
    },

    /// Export search results in various formats
    ///
    /// Combine search results into a single markdown file
    /// or export as JSON for programmatic use.
    Export {
        /// Format: md, json
        format: String,
        /// Optional search query to filter exported content
        query: Option<String>,
    },

    /// Generate AI-ready prompt context
    ///
    /// Search for content and wrap matching results with
    /// surrounding context, formatted for AI prompt input.
    Prompt {
        /// Search query
        query: String,
    },

    /// List file paths only (no content)
    ///
    /// Output just the paths of all markdown files found,
    /// one per line.
    Path,

    /// Show document statistics
    ///
    /// Display file counts, total size, word counts, and
    /// reading time estimates.
    Stat {
        /// Subcommand: time (for reading time analysis)
        sub: Option<String>,
    },

    /// Scan AI coding agent configurations
    ///
    /// Detect and display AI agent prompt files (CLAUDE.md,
    /// AGENTS.md, .cursorrules, etc.) in tree format with
    /// status indicators.
    #[command(name = "claude")]
    Claude,

    /// Scan OpenAI Codex agent configuration
    #[command(name = "codex")]
    Codex,

    /// Scan all AI agent configurations
    ///
    /// Comprehensive scan of all known AI coding agent
    /// configurations including Claude, Codex, Copilot,
    /// Cursor, Windsurf, Aider, Continue, Cline, and more.
    #[command(name = "agents")]
    Agents,

    /// Interactive TUI exploration mode
    Ui,
}

fn main() {
    let cli = Cli::parse();
    let path = cli.path.unwrap_or_else(|| std::env::current_dir().unwrap());

    if !path.exists() {
        eprintln!("Error: path '{}' does not exist.", path.display());
        std::process::exit(1);
    }

    match cli.command {
        Commands::Todo { sub, query } => {
            cmd::todo::run_todo(&path, sub.as_deref(), query.as_deref());
        }
        Commands::Find { filter, query } => {
            let known_filters = ["head", "code", "link", "bold", "quote"];
            if known_filters.contains(&filter.as_str()) {
                let q = query.unwrap_or_default();
                if q.is_empty() {
                    eprintln!("Usage: gmd find {} <QUERY>", filter);
                    std::process::exit(1);
                }
                cmd::find::run_find(&path, Some(&filter), &q);
            } else {
                // filter is actually the query for full-text search
                cmd::find::run_find(&path, None, &filter);
            }
        }
        Commands::Img { sub } => {
            cmd::img::run_img(&path, &sub);
        }
        Commands::Link { sub, target } => {
            cmd::link::run_link(&path, &sub, target.as_deref());
        }
        Commands::Tag { sub, name } => {
            cmd::tag::run_tag(&path, sub.as_deref(), name.as_deref());
        }
        Commands::Meta { sub } => {
            cmd::meta::run_meta(&path, &sub);
        }
        Commands::Export { format, query } => {
            cmd::export::run_export(&path, &format, query.as_deref());
        }
        Commands::Prompt { query } => {
            cmd::prompt_cmd::run_prompt(&path, &query);
        }
        Commands::Path => {
            cmd::export::run_path_only(&path);
        }
        Commands::Stat { sub } => {
            cmd::stat::run_stat(&path, sub.as_deref());
        }
        Commands::Claude => {
            agent_scan::scan_agent(&path, Some("claude"));
        }
        Commands::Codex => {
            agent_scan::scan_agent(&path, Some("codex"));
        }
        Commands::Agents => {
            agent_scan::scan_agent(&path, None);
        }
        Commands::Ui => {
            eprintln!("TUI mode is coming in v0.2.0. Stay tuned!");
            std::process::exit(0);
        }
    }
}
