pub fn safe_get(items: &[i32], index: usize) -> Option<i32> {
    // TODO: Return Some(value) if index is valid, None otherwise.
    items.get(index).copied()
}

fn main() {
    let nums = [10, 20, 30];

    match safe_get(&nums, 1) {
        Some(val) => println!("Found: {val}"),
        None => println!("Not Found"),
    }

    println!("{:?}", safe_get(&nums, 1));
    println!("{:?}", safe_get(&nums, 99));
}
