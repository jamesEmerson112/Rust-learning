use std::collections::HashMap;
use std::time::Instant;

fn main() {
    // Build a ~1M word string by repeating a paragraph
    let base = "the quick brown fox jumps over the lazy dog and the fox runs fast \
                through the dark forest while the dog sleeps by the warm fire and \
                the cat watches from the window as birds fly over the old stone wall";
    let mut text = String::with_capacity(base.len() * 30_000);
    for _ in 0..30_000 {
        text.push_str(base);
        text.push(' ');
    }

    let start = Instant::now();
    let mut counts: HashMap<String, usize> = HashMap::new();
    for word in text.split_whitespace() {
        let w = word.to_lowercase();
        *counts.entry(w).or_insert(0) += 1;
    }
    let elapsed = start.elapsed();

    println!("{}", elapsed.as_secs_f64() * 1000.0);
    eprintln!("unique = {}", counts.len());
}
