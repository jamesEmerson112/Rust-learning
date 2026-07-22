// THE VAULT RUN — Chapter 1: LOADOUT — ★ BUG HUNT ★
//
// BUG: Every implant behaves factory-fresh. The 21-point reflex firmware IS installed —
// is_stock() proves it's in the chrome — but the deck keeps reading zeroes, blank
// strings, stock everything. Something in the deref path serves the wrong slot.
//
// Find it, fix it: cargo test --test c58_tests
use std::ops::Deref;

pub struct Implant<T: Default> {
    pub model: String,
    firmware: T,        // what's actually installed
    factory_default: T, // the stock image, kept for hard resets
}

impl<T: Default> Implant<T> {
    pub fn new(model: &str, firmware: T) -> Implant<T> {
        Implant {
            model: model.to_string(),
            firmware,
            factory_default: T::default(),
        }
    }

    pub fn is_stock(&self) -> bool
    where
        T: PartialEq,
    {
        self.firmware == self.factory_default
    }
}

impl<T: Default> Deref for Implant<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.factory_default
    }
}

pub fn boost(skill: &i32) -> i32 {
    *skill * 2
}

fn main() {
    let reflex = Implant::new("neural-lace mk2", 21);
    println!("[diagnostic] stock chrome? {} (so firmware IS installed)", reflex.is_stock());
    println!("[diagnostic] deck reads {} — boosted: {} (want 21 and 42)", *reflex, boost(&reflex));
    println!("══ when the deck reads 42, CHAPTER 1: LOADOUT is complete ══");
}
