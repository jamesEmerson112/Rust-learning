pub fn apply(x: i32, f: impl Fn(i32) -> i32) -> i32 {
    // TODO: Call the closure `f` on `x` and return the result.
    let _ = (x, &f);
    0
}

fn main() {
    let double = |n| n * 2;
    println!("{}", apply(5, double));
    println!("{}", apply(5, |n| n + 100));
}
