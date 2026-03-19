pub fn evens_up_to(n: u32) -> Vec<u32> {
    // TODO: Return all even numbers from 1 to n (inclusive).
    let mut  temp =  Vec::new();
    for i in 1..=n {
        if i % 2 == 0 {
            temp.push(i);
        }
    }
    temp

}

fn main() {
    println!("Evens up to 6: {:?}", evens_up_to(6));
}
