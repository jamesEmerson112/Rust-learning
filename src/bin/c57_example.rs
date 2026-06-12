// Box<dyn Trait> = runtime polymorphism: store different concrete types behind one trait and
// dispatch through a vtable at runtime (generics, by contrast, pick the type at compile time).
// Coming from C: it's the struct-of-function-pointers vtable you'd hand-roll to put unlike
// objects in one array and call them uniformly — here the compiler builds the vtable for you.
trait Service {
    fn price(&self) -> u32;
}

struct Manicure;
struct Pedicure;

impl Service for Manicure {
    fn price(&self) -> u32 {
        4500
    }
}

impl Service for Pedicure {
    fn price(&self) -> u32 {
        6000
    }
}

fn total_price(services: &[Box<dyn Service>]) -> u32 {
    services.iter().map(|s| s.price()).sum()
}

fn main() {
    let booked: Vec<Box<dyn Service>> = vec![Box::new(Manicure), Box::new(Pedicure)];
    println!("Total: {} cents", total_price(&booked));
}
