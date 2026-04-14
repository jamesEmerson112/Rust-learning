use std::collections::HashMap;

pub fn normalise(input: &str) -> String {
    input.trim_matches(|c: char | !c.is_alphanumeric()).to_lowercase()
}

pub fn word_count(input: &str) -> HashMap<String, usize> {
    // TODO: Count word frequencies in `input`, where each word is:
    //   1) split by whitespace (.split_whitespace())
    //   2) normalized: trim non-alphanumeric, lowercase
    //   3) skipped if it becomes empty
    //   4) counted with the entry API
    let mut counts: HashMap<String, usize> = HashMap::new();
    for string in input.split_whitespace() {
        let procsesed_string = normalise(string);
        *counts.entry(procsesed_string).or_insert(0) += 1
    }

    counts
}

fn main() {
    let input = "Hello, hello world!";
    let counts = word_count(input);
    println!("{counts:?}");
}
