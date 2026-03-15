struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let outer = Rectangle {
        width: 12,
        height: 8,
    };
    let inner = Rectangle {
        width: 10,
        height: 6,
    };

    println!("Outer area: {}", outer.area());
    println!("Outer perimeter: {}", outer.perimeter());
    println!("Can outer hold inner? {}", outer.can_hold(&inner));
}
