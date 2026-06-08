pub struct WalkInQueue {
    names: Vec<String>,
    index: usize,
}

impl WalkInQueue {
    pub fn new(names: Vec<String>) -> Self {
        Self { names, index: 0 }
    }
}

impl Iterator for WalkInQueue {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.names.len() {
            let name = self.names[self.index].clone();
            self.index += 1;
            Some(name)
        } else {
            None
        }
    }
}

pub fn gel_prices(queue: WalkInQueue) -> Vec<u32> {
    // TODO: Filter entries containing "Gel", then map each to its price.
    // Each entry is "Name:Service:Price" — split on ':', take index 2, parse as u32.
    let gel_prices: Vec<u32> = queue.filter(|entry| entry.contains("Gel"))
        .map(|entry| {
            entry.split(":").nth(2).unwrap().parse::<u32>().unwrap()
        }).collect();
    gel_prices
}

fn main() {
    let queue = WalkInQueue::new(vec![
        "Mai:Gel:4500".to_string(),
        "Linh:Pedicure:3500".to_string(),
    ]);
    println!("{:?}", gel_prices(queue));
}
