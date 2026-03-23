#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn distance_from_origin(p: &Point) -> f64 {
    // TODO: Return the distance from (0, 0) to (p.x, p.y).
    // Hint: Use (x*x + y*y).sqrt()
    (p.x * p.x + p.y * p.y).sqrt()
}

fn main() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("Distance: {}", distance_from_origin(&p));
}
