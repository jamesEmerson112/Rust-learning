fn normalize_word(word: &str) -> String {
    word.trim_matches(|c: char| !c.is_alphanumeric())
        .to_lowercase()
}

fn main() {
    for raw in ["Hello,", "...Rust!!!", "42nd", "???"] {
        println!("{raw:?} -> {:?}", normalize_word(raw));
    }
}
