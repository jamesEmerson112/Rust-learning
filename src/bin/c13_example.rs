use std::collections::HashMap;

fn count_chars(s: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    counts
}

fn main() {
    let counts = count_chars("hello");
    for (ch, n) in &counts {
        println!("'{ch}': {n}");
    }
}
