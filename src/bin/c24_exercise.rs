pub fn swap<T>(a: T, b: T) -> (T, T) {
    // TODO: Return (b, a) — a generic swap that works for any T.
    let _ = (&a, &b);
    (a, b)
}

fn main() {
    println!("{:?}", swap(1, 2));
    println!("{:?}", swap("hi", "bye"));
}
