pub trait Describable {
    fn describe(&self) -> String;
}

pub struct Item {
    pub name: String,
    pub price: f64,
}

impl Describable for Item {
    fn describe(&self) -> String {
        // TODO: Return "<name>: $<price>" with price to 2 decimal places.
        let a = &self.name;
        let b = self.price;
        format!("{}: ${:.2}", a, b)
    }
}

fn main() {
    let item = Item {
        name: "Widget".to_string(),
        price: 9.99,
    };
    println!("{}", item.describe());
}
