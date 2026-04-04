use colored::Colorize;
use std::path::Path;

struct AgentConfig {
    name: &'static str,
    description: &'static str,
    paths: &'static [&'static str],
    required: &'static [&'static str],
    #[allow(dead_code)]
    optional: &'static [&'static str],
}

const AGENTS: &[AgentConfig] = &[
    AgentConfig {
        name: "Claude Code",
        description: "Anthropic Claude Code - AI coding assistant",
        paths: &[
            "CLAUDE.md",
            ".claude/",
            ".claude/settings.json",
            ".claude/commands/",
            ".claude/commands.md",
        ],
        required: &["CLAUDE.md"],
        optional: &[
            ".claude/",
            ".claude/settings.json",
            ".claude/commands/",
            ".claude/commands.md",
        ],
    },
    AgentConfig {
        name: "OpenAI Codex",
        description: "OpenAI Codex CLI - AI coding agent",
        paths: &["AGENTS.md", ".codex/", ".codex/config.json", "codex.md"],
        required: &["AGENTS.md"],
        optional: &[".codex/", ".codex/config.json", "codex.md"],
    },
    AgentConfig {
        name: "GitHub Copilot",
        description: "GitHub Copilot - AI pair programmer",
        paths: &[
            ".github/copilot-instructions.md",
            ".copilot/",
            ".vscode/settings.json",
            "copilot-instructions.md",
        ],
        required: &[".github/copilot-instructions.md"],
        optional: &[
            ".copilot/",
            ".vscode/settings.json",
            "copilot-instructions.md",
        ],
    },
    AgentConfig {
        name: "Cursor",
        description: "Cursor AI IDE - rules and configuration",
        paths: &[
            ".cursor/rules/",
            ".cursorrules",
            ".cursor/",
            ".cursor/settings.json",
        ],
        required: &[],
        optional: &[
            ".cursor/rules/",
            ".cursorrules",
            ".cursor/",
            ".cursor/settings.json",
        ],
    },
    AgentConfig {
        name: "Windsurf (Codeium)",
        description: "Windsurf by Codeium - AI IDE",
        paths: &[".windsurfrules", ".windsurf/", ".codeium/"],
        required: &[],
        optional: &[".windsurfrules", ".windsurf/", ".codeium/"],
    },
    AgentConfig {
        name: "Aider",
        description: "Aider - AI pair programming in terminal",
        paths: &[".aider.conf.yml", ".aiderignore", "CONVENTIONS.md"],
        required: &[],
        optional: &[".aider.conf.yml", ".aiderignore", "CONVENTIONS.md"],
    },
    AgentConfig {
        name: "Continue.dev",
        description: "Continue - open-source AI code assistant",
        paths: &[
            ".continue/",
            ".continue/config.json",
            ".continue/config.ts",
            ".continuerules",
        ],
        required: &[],
        optional: &[
            ".continue/",
            ".continue/config.json",
            ".continue/config.ts",
            ".continuerules",
        ],
    },
    AgentConfig {
        name: "Cline",
        description: "Cline (formerly Claude Dev) - autonomous coding agent",
        paths: &[".clinerules", ".cline/", ".cline/settings.json"],
        required: &[],
        optional: &[".clinerules", ".cline/", ".cline/settings.json"],
    },
    AgentConfig {
        name: "Amazon Q Developer",
        description: "Amazon Q Developer (formerly CodeWhisperer)",
        paths: &[".amazonq/", ".amazonq/rules/", ".amazonq/settings.json"],
        required: &[],
        optional: &[".amazonq/", ".amazonq/rules/", ".amazonq/settings.json"],
    },
    AgentConfig {
        name: "Gemini Code Assist",
        description: "Google Gemini Code Assist",
        paths: &[".gemini/", ".gemini/settings.json", "GEMINI.md"],
        required: &[],
        optional: &[".gemini/", ".gemini/settings.json", "GEMINI.md"],
    },
];

pub fn scan_agent(path: &Path, agent_name: Option<&str>) {
    println!("\n{}", "  AI Coding Agent Configuration Scanner".bold());
    println!("  {}", "═".repeat(55));
    println!("  Scanning: {}\n", path.display().to_string().cyan());

    let agents_to_scan: Vec<&AgentConfig> = if let Some(name) = agent_name {
        let name_lower = name.to_lowercase();
        AGENTS
            .iter()
            .filter(|a| a.name.to_lowercase().contains(&name_lower))
            .collect()
    } else {
        AGENTS.iter().collect()
    };

    if agents_to_scan.is_empty() {
        println!(
            "  No matching agent found for '{}'.",
            agent_name.unwrap_or("")
        );
        println!("\n  Available agents:");
        for agent in AGENTS {
            println!("    - {}", agent.name);
        }
        return;
    }

    let mut found_any = false;

    for agent in &agents_to_scan {
        let mut existing = Vec::new();
        let mut missing_required = Vec::new();
        let mut missing_optional = Vec::new();

        for p in agent.paths {
            let full = path.join(p);
            if full.exists() {
                existing.push(*p);
            } else if agent.required.contains(p) {
                missing_required.push(*p);
            } else {
                missing_optional.push(*p);
            }
        }

        let has_any = !existing.is_empty();
        if has_any {
            found_any = true;
        }

        let status = if missing_required.is_empty() && has_any {
            "READY".green().bold()
        } else if has_any {
            "PARTIAL".yellow().bold()
        } else {
            "NOT FOUND".dimmed().bold()
        };

        println!("  {} [{}]", agent.name.bold(), status);
        println!("  {}", agent.description.dimmed());
        println!();

        // Tree-style output
        let all_paths = agent.paths;
        for (i, p) in all_paths.iter().enumerate() {
            let is_last = i == all_paths.len() - 1;
            let prefix = if is_last {
                "  └── "
            } else {
                "  ├── "
            };
            let full = path.join(p);

            if full.exists() {
                let size_info = if full.is_file() {
                    if let Ok(meta) = std::fs::metadata(&full) {
                        format!(" ({})", format_size(meta.len()))
                    } else {
                        String::new()
                    }
                } else {
                    " (dir)".to_string()
                };
                println!("  {} {} {}{}", prefix, "✓".green(), p, size_info.dimmed());
            } else if agent.required.contains(p) {
                println!("  {} {} {} {}", prefix, "✗".red(), p, "[REQUIRED]".red());
            } else {
                println!(
                    "  {} {} {} {}",
                    prefix,
                    "○".dimmed(),
                    p.dimmed(),
                    "[optional]".dimmed()
                );
            }
        }
        println!();
    }

    if !found_any && agent_name.is_none() {
        println!(
            "  {}",
            "No AI coding agent configurations detected.".yellow()
        );
        println!("  Consider adding CLAUDE.md or AGENTS.md to configure your AI assistant.\n");
    }
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}
