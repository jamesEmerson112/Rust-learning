pub fn normalize_word(word: &str) -> String {
    // TODO: Return the word with:
    //   1) leading/trailing non-alphanumeric characters trimmed
    //      (use .trim_matches(|c: char| !c.is_alphanumeric()))
    //   2) lowercased (.to_lowercase())
    word.trim_matches(|c: char| !c.is_alphanumeric())
    .to_lowercase()
}

fn main() {
    println!("{:?}", normalize_word("Hello,"));
    println!("{:?}", normalize_word("...Rust!!!"));
}
