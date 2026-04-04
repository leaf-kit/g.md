use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Frontmatter {
    pub fields: HashMap<String, serde_yaml::Value>,
    #[allow(dead_code)]
    pub raw: String,
}

pub fn parse_frontmatter(content: &str) -> (Option<Frontmatter>, &str) {
    let trimmed = content.trim_start_matches('\u{feff}');
    if !trimmed.starts_with("---") {
        return (None, content);
    }

    let after_first = &trimmed[3..];
    let newline_pos = match after_first.find('\n') {
        Some(pos) => pos,
        None => return (None, content),
    };

    let rest = &after_first[newline_pos + 1..];
    if let Some(end) = rest.find("\n---") {
        let yaml_str = &rest[..end];
        let body = &rest[end + 4..];
        let body = body.strip_prefix('\n').unwrap_or(body);

        let fields: HashMap<String, serde_yaml::Value> =
            serde_yaml::from_str(yaml_str).unwrap_or_default();

        let fm = Frontmatter {
            fields,
            raw: yaml_str.to_string(),
        };
        (Some(fm), body)
    } else {
        (None, content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter() {
        let content = "---\ntitle: Hello\ntags: [rust, cli]\n---\n# Body here";
        let (fm, body) = parse_frontmatter(content);
        assert!(fm.is_some());
        let fm = fm.unwrap();
        assert_eq!(
            fm.fields.get("title").and_then(|v| v.as_str()),
            Some("Hello")
        );
        assert_eq!(body, "# Body here");
    }

    #[test]
    fn test_no_frontmatter() {
        let content = "# Just a heading\nSome text";
        let (fm, body) = parse_frontmatter(content);
        assert!(fm.is_none());
        assert_eq!(body, content);
    }

    #[test]
    fn test_bom_frontmatter() {
        let content = "\u{feff}---\nkey: value\n---\nBody";
        let (fm, body) = parse_frontmatter(content);
        assert!(fm.is_some());
        assert_eq!(body, "Body");
    }
}
