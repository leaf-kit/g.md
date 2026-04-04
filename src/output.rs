use colored::Colorize;

pub struct SearchResult {
    pub file: String,
    pub line: usize,
    pub content: String,
}

pub fn print_result(result: &SearchResult) {
    println!(
        "  {}:{} {}",
        result.file.cyan(),
        result.line.to_string().yellow(),
        result.content
    );
}
