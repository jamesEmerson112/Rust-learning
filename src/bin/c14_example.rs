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
    let input = "Rust rust, ownership! Borrowing rust.";
    let counts = word_count(input);

    let mut pairs: Vec<_> = counts.iter().collect();
    pairs.sort_by_key(|(word, _)| *word);

    for (word, count) in pairs {
        println!("{word}: {count}");
    }
}
