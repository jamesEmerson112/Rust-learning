trait Describable {
    fn describe(&self) -> String;
}

struct Item {
    name: String,
    price: f64,
}

impl Describable for Item {
    fn describe(&self) -> String {
        format!("{}: ${:.2}", self.name, self.price)
    }
}

fn print_description(item: &impl Describable) {
    println!("{}", item.describe());
}

fn main() {
    let item = Item {
        name: "Widget".to_string(),
        price: 9.99,
    };
    print_description(&item);
}
