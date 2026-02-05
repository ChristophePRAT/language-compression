pub fn preprocess_text(text: &str) -> String {
    text.to_lowercase().chars().collect()
}
