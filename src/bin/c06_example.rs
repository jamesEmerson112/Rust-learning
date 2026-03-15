fn fizzbuzz(n: u32) -> Vec<String> {
    let mut out = Vec::new();

    for i in 1..=n {
        let item = match (i % 3 == 0, i % 5 == 0) {
            (true, true) => "FizzBuzz".to_string(),
            (true, false) => "Fizz".to_string(),
            (false, true) => "Buzz".to_string(),
            (false, false) => i.to_string(),
        };
        out.push(item);
    }

    out
}

fn main() {
    let result = fizzbuzz(20);
    println!("{}", result.join(", "));
}
