use lazy_static::lazy_static;
use regex::Regex;

pub fn sanitize_command(text: &str) -> String {
    text.trim().to_owned()
}

pub fn sanitize_temp_dir(prefix: &str) -> String {
    lazy_static! {
        static ref RE_NON_WORDS: Regex = Regex::new(r"\W").unwrap();
    }
    RE_NON_WORDS
        .replace_all(prefix.to_lowercase().as_str(), "-")
        .to_string()
}

/// WORKAROUND: Rust Gherkin `Step::docstring()` always starts with a newline
pub fn trim_docstring_prefix_newline(docstring: &String) -> &str {
    docstring
        .strip_prefix("\r\n")
        .or_else(|| docstring.strip_prefix('\n'))
        .unwrap_or(docstring)
}
