fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    let x: i32 = 5;
    let y: i32 = 10;
    let mut sum = x + y;
    println!("{x} + {y} = {sum}");

    sum += 1;
    println!("After mutation: {sum}");

    let small: u8 = 255;
    let flag: bool = true;
    println!("u8 max: {small}, flag: {flag}");

    println!("double(5) = {}", double(5));
    println!("double(-3) = {}", double(-3));
}
