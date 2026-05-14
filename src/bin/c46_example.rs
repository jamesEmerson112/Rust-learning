struct WalkInQueue {
    names: Vec<String>,
    index: usize,
}

impl WalkInQueue {
    fn new(names: Vec<String>) -> Self {
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

fn main() {
    let queue = WalkInQueue::new(vec![
        "Mai:Gel:4500".to_string(),
        "Linh:Pedicure:3500".to_string(),
        "Trang:Acrylic:6000".to_string(),
    ]);

    let gel_prices: Vec<u32> = queue
        .filter(|entry| entry.contains("Gel"))
        .map(|entry| {
            entry.split(':').nth(2).unwrap().parse::<u32>().unwrap()
        })
        .collect();

    println!("Gel service prices: {gel_prices:?}");
}
