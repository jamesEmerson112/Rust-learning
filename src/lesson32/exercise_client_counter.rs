pub struct ClientCounter {
    count: u32,
}

impl ClientCounter {
    pub fn new() -> Self {
        // TODO: Initialize a counter starting at 0.
        Self { count: 0 }
    }

    pub fn increment(&mut self) {
        // TODO: Add 1 to the count.
        self.count += 1;
    }

    pub fn value(&self) -> u32 {
        // TODO: Return the current count.
        self.count
    }
}
