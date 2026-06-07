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
    let queue: WalkInQueue = WalkInQueue::new(vec![
        "Mai".to_string(),
        "Linh".to_string(),
        "Trang".to_string(),
    ]);

    for client in queue {
        println!("Now serving: {client}");
    }
}
