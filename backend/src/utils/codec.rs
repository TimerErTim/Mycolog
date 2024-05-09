pub fn json_encoded_to_utf8(encoded: &str) -> Result<String, serde_json::Error> {
    serde_json::from_str::<String>(&format!("\"{encoded}\""))
}

pub fn utf8_to_json_encoded(string: &str) -> Result<String, serde_json::Error> {
    serde_json_fmt::JsonFormat::new()
        .ascii(true)
        .format_to_string(string)
}
