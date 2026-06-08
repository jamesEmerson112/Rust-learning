use std::cell::RefCell;

struct Station<'a> {
    name: String,
    log: &'a RefCell<Vec<String>>,
}

impl<'a> Drop for Station<'a> {
    fn drop(&mut self) {
        self.log.borrow_mut().push(format!("closing {}", self.name));
    }
}

fn main() {
    let log = RefCell::new(Vec::new());
    {
        let _a = Station { name: "A".to_string(), log: &log };
        let _b = Station { name: "B".to_string(), log: &log };
        println!("Both stations open.");
    } // _b drops first, then _a — reverse of declaration order
    println!("{:?}", log.borrow());
}
