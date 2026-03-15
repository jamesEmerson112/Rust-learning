use std::collections::HashMap;

pub fn count_chars(s: &str) -> HashMap<char, usize> {
    // TODO: Count the frequency of each character in the string.
    let _ = s;
    HashMap::new()
}

fn main() {
    let counts = count_chars("aab");
    println!("{counts:?}");
}
