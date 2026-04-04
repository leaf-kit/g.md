use regex::Regex;
use std::sync::LazyLock;

#[derive(Debug, Clone, PartialEq)]
pub enum MdElement {
    Heading {
        level: u8,
        text: String,
        line: usize,
    },
    Checkbox {
        checked: bool,
        text: String,
        line: usize,
    },
    CodeBlock {
        lang: String,
        content: String,
        start_line: usize,
        end_line: usize,
    },
    Link {
        text: String,
        url: String,
        line: usize,
    },
    WikiLink {
        target: String,
        line: usize,
    },
    Image {
        alt: String,
        url: String,
        line: usize,
    },
    Bold {
        text: String,
        line: usize,
    },
    Quote {
        text: String,
        line: usize,
    },
    Tag {
        name: String,
        line: usize,
    },
    Text {
        content: String,
        line: usize,
    },
}

static RE_HEADING: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(#{1,6})\s+(.+)$").unwrap());
static RE_CHECKBOX_UNCHECKED: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[-*]\s+\[ \]\s+(.+)$").unwrap());
static RE_CHECKBOX_CHECKED: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[-*]\s+\[x\]\s+(.+)$").unwrap());
static RE_LINK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[([^\]]*)\]\(([^)]+)\)").unwrap());
static RE_WIKILINK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[\[([^\]]+)\]\]").unwrap());
static RE_IMAGE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"!\[([^\]]*)\]\(([^)]+)\)").unwrap());
static RE_BOLD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\*\*([^*]+)\*\*").unwrap());
static RE_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?:^|\s)#([a-zA-Z0-9_\-/\u{AC00}-\u{D7AF}]+)").unwrap());
static RE_CODE_FENCE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^```(\w*)").unwrap());

pub fn parse_markdown(content: &str) -> Vec<MdElement> {
    let mut elements = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        let line_num = i + 1;
        let trimmed = line.trim();

        // Code block
        if let Some(caps) = RE_CODE_FENCE.captures(trimmed) {
            let lang = caps.get(1).map_or("", |m| m.as_str()).to_string();
            let start = line_num;
            let mut code_lines = Vec::new();
            i += 1;
            while i < lines.len() {
                if lines[i].trim().starts_with("```") {
                    break;
                }
                code_lines.push(lines[i]);
                i += 1;
            }
            let end = i + 1;
            elements.push(MdElement::CodeBlock {
                lang,
                content: code_lines.join("\n"),
                start_line: start,
                end_line: end,
            });
            i += 1;
            continue;
        }

        // Heading
        if let Some(caps) = RE_HEADING.captures(trimmed) {
            let level = caps.get(1).unwrap().as_str().len() as u8;
            let text = caps.get(2).unwrap().as_str().to_string();
            elements.push(MdElement::Heading {
                level,
                text,
                line: line_num,
            });
        }

        // Checkbox unchecked
        if let Some(caps) = RE_CHECKBOX_UNCHECKED.captures(trimmed) {
            let text = caps.get(1).unwrap().as_str().to_string();
            elements.push(MdElement::Checkbox {
                checked: false,
                text,
                line: line_num,
            });
        }

        // Checkbox checked
        if let Some(caps) = RE_CHECKBOX_CHECKED.captures(trimmed) {
            let text = caps.get(1).unwrap().as_str().to_string();
            elements.push(MdElement::Checkbox {
                checked: true,
                text,
                line: line_num,
            });
        }

        // Quote
        if let Some(quote_text) = trimmed.strip_prefix("> ") {
            let text = quote_text.to_string();
            elements.push(MdElement::Quote {
                text,
                line: line_num,
            });
        }

        // Images (before links to avoid overlap)
        for caps in RE_IMAGE.captures_iter(line) {
            elements.push(MdElement::Image {
                alt: caps.get(1).unwrap().as_str().to_string(),
                url: caps.get(2).unwrap().as_str().to_string(),
                line: line_num,
            });
        }

        // Links (exclude images)
        let no_images = RE_IMAGE.replace_all(line, "");
        for caps in RE_LINK.captures_iter(&no_images) {
            elements.push(MdElement::Link {
                text: caps.get(1).unwrap().as_str().to_string(),
                url: caps.get(2).unwrap().as_str().to_string(),
                line: line_num,
            });
        }

        // Wiki links
        for caps in RE_WIKILINK.captures_iter(line) {
            elements.push(MdElement::WikiLink {
                target: caps.get(1).unwrap().as_str().to_string(),
                line: line_num,
            });
        }

        // Bold
        for caps in RE_BOLD.captures_iter(line) {
            elements.push(MdElement::Bold {
                text: caps.get(1).unwrap().as_str().to_string(),
                line: line_num,
            });
        }

        // Tags (skip heading lines like "# Title", "## Sub", but allow "#tag")
        let is_heading_line = RE_HEADING.is_match(trimmed);
        if !is_heading_line {
            for caps in RE_TAG.captures_iter(line) {
                let name = caps.get(1).unwrap().as_str().to_string();
                elements.push(MdElement::Tag {
                    name,
                    line: line_num,
                });
            }
        }

        // Plain text (for general search)
        if !trimmed.is_empty() {
            elements.push(MdElement::Text {
                content: line.to_string(),
                line: line_num,
            });
        }

        i += 1;
    }

    elements
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_heading() {
        let elements = parse_markdown("# Hello World\n## Sub heading");
        let headings: Vec<_> = elements
            .iter()
            .filter(|e| matches!(e, MdElement::Heading { .. }))
            .collect();
        assert_eq!(headings.len(), 2);
        if let MdElement::Heading { level, text, .. } = &headings[0] {
            assert_eq!(*level, 1);
            assert_eq!(text, "Hello World");
        }
    }

    #[test]
    fn test_parse_checkbox() {
        let elements = parse_markdown("- [ ] Todo item\n- [x] Done item");
        let checks: Vec<_> = elements
            .iter()
            .filter(|e| matches!(e, MdElement::Checkbox { .. }))
            .collect();
        assert_eq!(checks.len(), 2);
        if let MdElement::Checkbox { checked, text, .. } = &checks[0] {
            assert!(!checked);
            assert_eq!(text, "Todo item");
        }
        if let MdElement::Checkbox { checked, text, .. } = &checks[1] {
            assert!(checked);
            assert_eq!(text, "Done item");
        }
    }

    #[test]
    fn test_parse_code_block() {
        let content = "```rust\nfn main() {}\n```";
        let elements = parse_markdown(content);
        let codes: Vec<_> = elements
            .iter()
            .filter(|e| matches!(e, MdElement::CodeBlock { .. }))
            .collect();
        assert_eq!(codes.len(), 1);
        if let MdElement::CodeBlock { lang, content, .. } = &codes[0] {
            assert_eq!(lang, "rust");
            assert_eq!(content, "fn main() {}");
        }
    }

    #[test]
    fn test_parse_link_and_image() {
        let content = "Check [Google](https://google.com) and ![logo](img.png)";
        let elements = parse_markdown(content);
        let links: Vec<_> = elements
            .iter()
            .filter(|e| matches!(e, MdElement::Link { .. }))
            .collect();
        let images: Vec<_> = elements
            .iter()
            .filter(|e| matches!(e, MdElement::Image { .. }))
            .collect();
        assert_eq!(links.len(), 1);
        assert_eq!(images.len(), 1);
    }

    #[test]
    fn test_parse_bold() {
        let elements = parse_markdown("This is **important** text");
        let bolds: Vec<_> = elements
            .iter()
            .filter(|e| matches!(e, MdElement::Bold { .. }))
            .collect();
        assert_eq!(bolds.len(), 1);
    }

    #[test]
    fn test_parse_quote() {
        let elements = parse_markdown("> This is a quote");
        let quotes: Vec<_> = elements
            .iter()
            .filter(|e| matches!(e, MdElement::Quote { .. }))
            .collect();
        assert_eq!(quotes.len(), 1);
    }

    #[test]
    fn test_parse_tag() {
        let elements = parse_markdown("Some text #rust #cli");
        let tags: Vec<_> = elements
            .iter()
            .filter(|e| matches!(e, MdElement::Tag { .. }))
            .collect();
        assert_eq!(tags.len(), 2);
    }

    #[test]
    fn test_parse_wikilink() {
        let elements = parse_markdown("See [[Other Page]] for details");
        let wikilinks: Vec<_> = elements
            .iter()
            .filter(|e| matches!(e, MdElement::WikiLink { .. }))
            .collect();
        assert_eq!(wikilinks.len(), 1);
    }
}
