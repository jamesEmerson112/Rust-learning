pub fn fizzbuzz(n: u32) -> Vec<String> {
    // TODO: Implement classic fizzbuzz rules from 1..=n.
    // - Divisible by 3 and 5: "FizzBuzz"
    // - Divisible by 3 only: "Fizz"
    // - Divisible by 5 only: "Buzz"
    // - Otherwise: the number as a string
    let mut result = Vec::new();

    for _ in 0..n {
        result.push("TODO".to_string());
    }

    result
}

fn main() {
    let result = fizzbuzz(20);
    println!("{}", result.join(", "));
}
