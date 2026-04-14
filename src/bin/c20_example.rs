fn apply(x: i32, f: impl Fn(i32) -> i32) -> i32 {
    f(x)
}

fn main() {
    let double = |n| n * 2;
    println!("double(5) = {}", double(5));

    let offset = 10;
    let shift = |n| n + offset;
    println!("shift(5) = {}", shift(5));

    println!("apply(7, square) = {}", apply(7, |x| x * x));
}
