fn larger<T: Ord>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

fn main() {
    println!("{}", larger(3, 7));
    println!("{}", larger("apple", "banana"));
}
