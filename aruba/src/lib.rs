pub mod cucumber_parameters;

pub mod text {
    pub fn sanitize_text(text: &str) -> String {
        //DO WE NEED THESE?
        //let text = unescape_text(text);
        //let text = extract_text(text) if aruba.config.remove_ansi_escape_sequences
        text.trim_end().to_owned()
    }
}

