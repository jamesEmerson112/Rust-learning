use std::collections::HashMap;

pub fn count_chars(s: &str) -> HashMap<char, usize> {
    // TODO: Count the frequency of each character in the string.
    let _ = s;
    let mut newHash: HashMap<char, usize> = HashMap::new();
    for c in s.chars()
    {
        *newHash.entry(c).or_insert(0) += 1;
    }
    newHash
}

fn main() {
    let counts = count_chars("aab");
    println!("{counts:?}");
}
