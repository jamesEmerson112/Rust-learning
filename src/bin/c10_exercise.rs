pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        // TODO: Return width * height.
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        // TODO: Return true only if self is strictly larger in both dimensions.
        let _ = other;
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let outer = Rectangle {
        width: 10,
        height: 8,
    };
    let inner = Rectangle {
        width: 9,
        height: 7,
    };

    println!("Area: {}", outer.area());
    println!("Can hold: {}", outer.can_hold(&inner));
}
