use std::collections::HashMap;

pub fn word_count(input: &str) -> HashMap<String, usize> {
    // TODO: Normalize words to lowercase, trim punctuation,
    // and count frequency with a HashMap.
    let _ = input;
    HashMap::new()
}

fn main() {
    let input = "Hello, hello world!";
    let counts = word_count(input);
    println!("{counts:?}");
}
