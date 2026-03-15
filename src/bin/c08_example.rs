fn first_word_len(s: &str) -> usize {
    s.split_whitespace().next().map_or(0, str::len)
}

fn main() {
    let text = "rust ownership rules";
    println!("First word length of '{text}': {}", first_word_len(text));

    println!("Single word: {}", first_word_len("borrow"));
    println!("Empty: {}", first_word_len(""));
    println!("Leading spaces: {}", first_word_len("   leading spaces"));
}
