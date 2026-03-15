fn safe_get(items: &[i32], index: usize) -> Option<i32> {
    items.get(index).copied()
}

fn main() {
    let nums = [10, 20, 30];

    // Using match
    match safe_get(&nums, 1) {
        Some(val) => println!("Found: {val}"),
        None => println!("Not found"),
    }

    // Using unwrap_or
    let val = safe_get(&nums, 99).unwrap_or(-1);
    println!("With default: {val}");

    // Using if let
    if let Some(first) = safe_get(&nums, 0) {
        println!("First: {first}");
    }
}
