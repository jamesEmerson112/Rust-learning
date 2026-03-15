pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Self {
        // TODO: Initialize a counter starting at 0.
        Self { count: 0 }
    }

    pub fn increment(&mut self) {
        // TODO: Add 1 to the count.
        let _ = &self.count;
    }

    pub fn value(&self) -> u32 {
        // TODO: Return the current count.
        let _ = &self.count;
        0
    }
}
