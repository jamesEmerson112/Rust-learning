// Implementing Deref lets your own wrapper act like a pointer: *pump works, and method
// calls "auto-deref" through it. This is the trait that makes Box/Rc/Arc feel built in.
// Coming from C: it's defining what unary `*` means for your type — a user-overloadable
// pointer dereference, with the compiler inserting the derefs for you where needed.
//
// RUST GENERAL HOSPITAL: an infusion pump wraps the setting a nurse programmed into it.
// Deref done right means any code can read that setting straight through the pump.
use std::ops::Deref;

struct Pump<T: Default> {
    device: String,
    programmed: T,      // what the nurse entered
    factory_default: T, // what the pump ships with, kept for a reset
}

impl<T: Default> Pump<T> {
    fn new(device: &str, programmed: T) -> Pump<T> {
        Pump {
            device: device.to_string(),
            programmed,
            factory_default: T::default(),
        }
    }

    fn is_default(&self) -> bool
    where
        T: PartialEq,
    {
        self.programmed == self.factory_default
    }
}

impl<T: Default> Deref for Pump<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.programmed // serve the PROGRAMMED setting, not the factory default
    }
}

fn two_hour_volume(rate_ml_per_hour: &i32) -> i32 {
    *rate_ml_per_hour * 2
}

fn main() {
    let iv = Pump::new("IV pump 3", 21);
    println!("[{}] still on factory default? {}", iv.device, iv.is_default());
    println!("[{}] rate {} ml/h — {} ml over two hours", iv.device, *iv, two_hour_volume(&iv));

    let syringe = Pump::new("syringe pump 1", String::from("insulin"));
    println!("[{}] loaded with '{}' ({} chars)", syringe.device, *syringe, syringe.len());
}
