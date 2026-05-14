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
        // TODO: If index < names.len(), clone the name at index,
        // advance index, and return Some(name). Otherwise None.
        None
    }
}

fn main() {
    let queue = WalkInQueue::new(vec!["Mai".to_string(), "Linh".to_string()]);
    for client in queue {
        println!("Serving: {client}");
    }
}
