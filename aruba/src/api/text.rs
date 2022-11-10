use lazy_static::lazy_static;
use regex::Regex;

pub fn sanitize_command(text: &str) -> String {
    //DO WE NEED THESE?
    //let text = unescape_text(text);
    //let text = extract_text(text) if aruba.config.remove_ansi_escape_sequences
    text.trim().to_owned()
}

pub fn sanitize_temp_dir(prefix: &str) -> String {
    lazy_static! { static ref RE_NON_WORDS: Regex = Regex::new(r"\W").unwrap(); }
    RE_NON_WORDS.replace_all(prefix.to_lowercase().as_str(), "-").to_string()
}

/// WORKAROUND: Rust Gherkin `Step::docstring()` always starts with a newline
pub fn trim_prefix_single_newline(docstring: &String) -> &str {
    docstring
        .strip_prefix("\r\n")
        .or_else(|| docstring.strip_prefix('\n'))
        .unwrap_or(docstring)
}
