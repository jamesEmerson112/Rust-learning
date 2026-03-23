pub fn first_word_len(s: &str) -> usize {
    // TODO: Return the length of the first word.
    // Hint: .split_whitespace().next() gives you an Option<&str>.
    s.split_whitespace().next().map_or(0, str::len)
}

fn main() {
    let text = String::from("hello rust");
    println!("First word length: {}", first_word_len(&text));
}
