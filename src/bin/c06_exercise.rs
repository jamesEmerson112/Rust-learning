pub fn fizzbuzz(n: u32) -> Vec<String> {
    // TODO: Implement classic fizzbuzz rules from 1..=n.
    // - Divisible by 3 and 5: "FizzBuzz"
    // - Divisible by 3 only: "Fizz"
    // - Divisible by 5 only: "Buzz"
    // - Otherwise: the number as a string
    let mut result = Vec::new();

    // int is_even = x % 2 == 0 ? true : false;

    // let is_even =
    //     if x % 2 == 0 {
    //         let v = true;
    //     } else {
    //         false;
    //     };

    for i in 1..=n {
        let item: String =
            match (i % 3 == 0, i % 5 == 0) {
                (true, true) => "FizzBuzz".to_string(),
                (true, false) => "Fizz".to_string(),
                (false, true) => "Buzz".to_string(),
                (false, false) => i.to_string()
            };

        result.push(item);
    }


    result
}

fn main() {
    let result = fizzbuzz(20);
    println!("{}", result.join(", "));
}
