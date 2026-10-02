// RUST GENERAL HOSPITAL — Ward Equipment — ★ BUG HUNT ★
//
// BUG: Every pump behaves as if nobody programmed it. Nurse Mai set IV pump 3 to
// 21 ml/h, and is_default() confirms the setting is stored — but anything that reads
// the pump gets zeroes, blank drug names, factory values. Something in the deref path
// serves the wrong field.
//
// Find it, fix it: cargo test --test c58_tests
use std::ops::Deref;

pub struct Pump<T: Default> {
    pub device: String,
    programmed: T,      // what the nurse entered
    factory_default: T, // what the pump ships with, kept for a reset
}

impl<T: Default> Pump<T> {
    pub fn new(device: &str, programmed: T) -> Pump<T> {
        Pump {
            device: device.to_string(),
            programmed,
            factory_default: T::default(),
        }
    }

    pub fn is_default(&self) -> bool
    where
        T: PartialEq,
    {
        self.programmed == self.factory_default
    }
}

impl<T: Default> Deref for Pump<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.factory_default
    }
}

pub fn two_hour_volume(rate_ml_per_hour: &i32) -> i32 {
    *rate_ml_per_hour * 2
}

fn main() {
    let iv = Pump::new("IV pump 3", 21);
    println!("[check] still on factory default? {} (so the setting IS stored)", iv.is_default());
    println!("[check] pump reads {} ml/h — {} ml over two hours (want 21 and 42)", *iv, two_hour_volume(&iv));
    println!("══ when the pump reads 21 ml/h, the ward equipment checks out ══");
}
