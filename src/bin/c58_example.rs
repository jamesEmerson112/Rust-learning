// Implementing Deref lets your own wrapper act like a pointer: *implant works, and method
// calls "auto-deref" through it. This is the trait that makes Box/Rc/Arc feel built in.
// Coming from C: it's defining what unary `*` means for your type — a user-overloadable
// pointer dereference, with the compiler inserting the derefs for you where needed.
//
// THE VAULT RUN: chrome implants wrap a skill. Deref done right means the skill shines
// through the chrome — the deck calls i32/String methods on the implant as if bare wetware.
use std::ops::Deref;

struct Implant<T: Default> {
    model: String,
    firmware: T,        // what's actually installed
    factory_default: T, // the stock image, kept for hard resets
}

impl<T: Default> Implant<T> {
    fn new(model: &str, firmware: T) -> Implant<T> {
        Implant {
            model: model.to_string(),
            firmware,
            factory_default: T::default(),
        }
    }

    fn is_stock(&self) -> bool
    where
        T: PartialEq,
    {
        self.firmware == self.factory_default
    }
}

impl<T: Default> Deref for Implant<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.firmware // serve the INSTALLED firmware, not the factory image
    }
}

fn boost(skill: &i32) -> i32 {
    *skill * 2
}

fn main() {
    let reflex = Implant::new("neural-lace mk2", 21);
    println!("[{}] stock? {}", reflex.model, reflex.is_stock());
    println!("[{}] deck reads {} — boosted: {}", reflex.model, *reflex, boost(&reflex));

    let stealth = Implant::new("subdermal ghostweave", String::from("stealth"));
    println!("[{}] skill '{}' loaded ({} chars)", stealth.model, *stealth, stealth.len());
}
