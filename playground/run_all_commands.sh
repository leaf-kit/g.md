#!/usr/bin/env bash
# ==============================================================================
#  gmd — Run All Commands
#
#  Executes every gmd command against the playground/ sample documents.
#  Designed for quick validation after build or install.
#
#  Usage:
#    cd playground/
#    ./run_all_commands.sh
#
#  Or from project root:
#    ./playground/run_all_commands.sh
# ==============================================================================

set -euo pipefail

# Resolve script directory so it works from anywhere
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
GMD="${GMD:-gmd}"

# Verify gmd is available
if ! command -v "$GMD" &>/dev/null; then
    echo "Error: '$GMD' not found. Install with: brew tap leaf-kit/gmd && brew install gmd"
    echo "Or set GMD env var: GMD=./target/release/gmd ./playground/run_all_commands.sh"
    exit 1
fi

passed=0
failed=0
total=0

run() {
    local label="$1"
    shift
    total=$((total + 1))
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "  [$total] $label"
    echo "  \$ $*"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    if eval "$@" 2>&1; then
        passed=$((passed + 1))
        echo ""
        echo "  -> PASS"
    else
        failed=$((failed + 1))
        echo ""
        echo "  -> FAIL (exit code: $?)"
    fi
}

echo "================================================================"
echo "  gmd — Run All Commands"
echo "  Version: $($GMD --version)"
echo "  Target:  $SCRIPT_DIR"
echo "================================================================"

# --------------------------------------------------------------------------
#  1. Version & Help
# --------------------------------------------------------------------------
run "Version" \
    "$GMD --version"

run "Help" \
    "$GMD --help"

# --------------------------------------------------------------------------
#  2. Task Management (todo)
# --------------------------------------------------------------------------
run "Todo: list pending tasks" \
    "$GMD todo -p $SCRIPT_DIR"

run "Todo: list completed tasks" \
    "$GMD todo done -p $SCRIPT_DIR"

run "Todo: search by keyword (migration)" \
    "$GMD todo migration -p $SCRIPT_DIR"

run "Todo: today's tasks" \
    "$GMD todo today -p $SCRIPT_DIR"

run "Todo: this week's tasks" \
    "$GMD todo week -p $SCRIPT_DIR"

# --------------------------------------------------------------------------
#  3. Precision Search (find)
# --------------------------------------------------------------------------
run "Find: full-text search (Rust)" \
    "$GMD find Rust -p $SCRIPT_DIR"

run "Find: headings only (API)" \
    "$GMD find head API -p $SCRIPT_DIR"

run "Find: code blocks only (query)" \
    "$GMD find code query -p $SCRIPT_DIR"

run "Find: links (api)" \
    "$GMD find link api -p $SCRIPT_DIR"

run "Find: bold text (Rust)" \
    "$GMD find bold Rust -p $SCRIPT_DIR"

run "Find: quotes (safety)" \
    "$GMD find quote safety -p $SCRIPT_DIR"

run "Find: full-text with --full-path" \
    "$GMD find Rust --full-path -p $SCRIPT_DIR"

run "Find: full-text with -F (short)" \
    "$GMD find Rust -F -p $SCRIPT_DIR"

run "Find: full-text with --relative-path" \
    "$GMD find Rust --relative-path -p $SCRIPT_DIR"

run "Find: full-text with -R (short)" \
    "$GMD find Rust -R -p $SCRIPT_DIR"

run "Find: headings with --full-path" \
    "$GMD find head API --full-path -p $SCRIPT_DIR"

run "Find: headings with --relative-path" \
    "$GMD find head API --relative-path -p $SCRIPT_DIR"

# --------------------------------------------------------------------------
#  4. Asset & Link Check (img, link)
# --------------------------------------------------------------------------
run "Img: list all images" \
    "$GMD img list -p $SCRIPT_DIR"

run "Img: find broken images" \
    "$GMD img broken -p $SCRIPT_DIR"

run "Img: find orphan images" \
    "$GMD img orphan -p $SCRIPT_DIR"

run "Link: list all links" \
    "$GMD link list -p $SCRIPT_DIR"

run "Link: find broken links" \
    "$GMD link broken -p $SCRIPT_DIR"

run "Link: backlinks to architecture" \
    "$GMD link back architecture -p $SCRIPT_DIR"

# --------------------------------------------------------------------------
#  5. Tag & Metadata (tag, meta)
# --------------------------------------------------------------------------
run "Tag: list all tags" \
    "$GMD tag list -p $SCRIPT_DIR"

run "Tag: top tags by frequency" \
    "$GMD tag top -p $SCRIPT_DIR"

run "Tag: find files with #rust" \
    "$GMD tag rust -p $SCRIPT_DIR"

run "Meta: list frontmatter fields" \
    "$GMD meta list -p $SCRIPT_DIR"

run "Meta: find missing metadata" \
    "$GMD meta missing -p $SCRIPT_DIR"

# --------------------------------------------------------------------------
#  6. Export & AI Prompt (export, prompt, path)
# --------------------------------------------------------------------------
run "Export: markdown merge" \
    "$GMD export md -p $SCRIPT_DIR | head -30"

run "Export: JSON" \
    "$GMD export json -p $SCRIPT_DIR | head -30"

run "Prompt: generate AI context (monitoring)" \
    "$GMD prompt monitoring -p $SCRIPT_DIR"

run "Path: list file paths only" \
    "$GMD path -p $SCRIPT_DIR"

# --------------------------------------------------------------------------
#  7. Statistics (stat)
# --------------------------------------------------------------------------
run "Stat: overview" \
    "$GMD stat -p $SCRIPT_DIR"

run "Stat: reading time & activity" \
    "$GMD stat time -p $SCRIPT_DIR"

# --------------------------------------------------------------------------
#  8. AI Agent Scanner (claude, codex, agents)
# --------------------------------------------------------------------------
run "Agent: scan Claude Code config" \
    "$GMD claude -p $SCRIPT_DIR"

run "Agent: scan OpenAI Codex config" \
    "$GMD codex -p $SCRIPT_DIR"

run "Agent: scan all AI agents" \
    "$GMD agents -p $SCRIPT_DIR"

# --------------------------------------------------------------------------
#  9. Pipe Integration
# --------------------------------------------------------------------------
run "Pipe: find + grep (filter by file)" \
    "$GMD find Rust -p $SCRIPT_DIR | grep meeting"

run "Pipe: todo count with grep -c" \
    "$GMD todo -p $SCRIPT_DIR | grep -c '\- \[ \]'"

run "Pipe: group tasks by file" \
    "$GMD todo -p $SCRIPT_DIR | grep '\- \[ \]' | awk -F: '{print \$1}' | sort | uniq -c | sort -rn"

run "Pipe: count matches per file" \
    "$GMD find Rust -p $SCRIPT_DIR | grep -v match | awk -F: '{print \$1}' | sort | uniq -c | sort -rn"

run "Pipe: line count per document" \
    "$GMD path -p $SCRIPT_DIR 2>/dev/null | sed 's|^|$SCRIPT_DIR/|' | xargs wc -l"

run "Pipe: extract broken image paths" \
    "$GMD img broken -p $SCRIPT_DIR 2>&1 | grep BROKEN | sed 's/.*!\\[.*\\](//;s/).*//' "

run "Pipe: filter broken internal links" \
    "$GMD link broken -p $SCRIPT_DIR 2>&1 | grep BROKEN"

run "Pipe: export JSON file list" \
    "$GMD export json -p $SCRIPT_DIR | python3 -c \"import sys,json; [print(d['file']) for d in json.load(sys.stdin)]\""

# --------------------------------------------------------------------------
#  Summary
# --------------------------------------------------------------------------
echo ""
echo "================================================================"
echo "  Results: $passed passed, $failed failed, $total total"
echo "================================================================"

if [ "$failed" -gt 0 ]; then
    echo "  Some commands failed. Review output above."
    exit 1
else
    echo "  All commands passed!"
    exit 0
fi
