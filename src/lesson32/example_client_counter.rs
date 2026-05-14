pub struct ClientCounter {
    count: u32,
}

impl ClientCounter {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn value(&self) -> u32 {
        self.count
    }
}
