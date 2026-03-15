fn evens_up_to(n: u32) -> Vec<u32> {
    let mut result = Vec::new();
    for i in 1..=n {
        if i % 2 == 0 {
            result.push(i);
        }
    }
    result
}

fn main() {
    let evens = evens_up_to(10);
    println!("Evens up to 10: {evens:?}");

    println!("Evens up to 0: {:?}", evens_up_to(0));
    println!("Evens up to 1: {:?}", evens_up_to(1));
}
