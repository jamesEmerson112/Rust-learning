use std::collections::HashMap;

fn normalize_word(word: &str) -> String {
    word.trim_matches(|c: char| !c.is_alphanumeric())
        .to_lowercase()
}

fn word_count(input: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for raw in input.split_whitespace() {
        let word = normalize_word(raw);
        if word.is_empty() {
            continue;
        }
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

fn main() {
    let counts: HashMap<String, usize> = word_count("Rust rust, ownership! Borrowing rust.");
    for (word, count) in &counts {
        println!("{word}: {count}");
    }
}
