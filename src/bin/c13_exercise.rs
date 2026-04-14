use std::collections::HashMap;

pub fn count_chars(s: &str) -> HashMap<char, usize> {
    // TODO: Count the frequency of each character in the string.
    let _ = s;
    let mut new_hash: HashMap<char, usize> = HashMap::new();
    for c in s.chars()
    {
        *new_hash.entry(c).or_insert(0) += 1;
    }
    new_hash
}

fn main() {
    let counts = count_chars("aab");
    println!("{counts:?}");
}
