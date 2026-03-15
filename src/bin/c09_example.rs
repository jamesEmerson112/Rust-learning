#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn distance_from_origin(p: &Point) -> f64 {
    (p.x * p.x + p.y * p.y).sqrt()
}

fn main() {
    let a = Point { x: 3.0, y: 4.0 };
    let origin = Point { x: 0.0, y: 0.0 };

    println!("Point: {a:?}");
    println!("Distance from origin: {}", distance_from_origin(&a));
    println!("Origin distance: {}", distance_from_origin(&origin));
}
