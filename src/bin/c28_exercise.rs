pub fn larger<T: Ord>(a: T, b: T) -> T {
    // TODO: Return whichever of `a` or `b` is larger (or `a` on tie).
    // The bound `T: Ord` lets you compare with >= or <=.
    if a >= b {a} else {b}
}

fn main() {
    println!("{}", larger(3, 7));
    println!("{}", larger("apple", "banana"));
}
