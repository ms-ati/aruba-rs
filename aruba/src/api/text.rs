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
