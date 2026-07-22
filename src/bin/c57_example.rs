// Box<dyn Trait> = runtime polymorphism: store different concrete types behind one trait and
// dispatch through a vtable at runtime (generics, by contrast, pick the type at compile time).
// Coming from C: it's the struct-of-function-pointers vtable you'd hand-roll to put unlike
// objects in one array and call them uniformly — here the compiler builds the vtable for you.
//
// THE VAULT RUN: your deck runs a mixed loadout of ICE-breaker programs. Different types,
// one slot rack — every program answers name() and power_draw() through the vtable.
trait Program {
    fn name(&self) -> String;
    fn power_draw(&self) -> u32;
}

struct Icepick;
struct Siphon;
struct Ghost;

impl Program for Icepick {
    fn name(&self) -> String {
        "Icepick".to_string()
    }
    fn power_draw(&self) -> u32 {
        40
    }
}

impl Program for Siphon {
    fn name(&self) -> String {
        "Siphon".to_string()
    }
    fn power_draw(&self) -> u32 {
        25
    }
}

impl Program for Ghost {
    fn name(&self) -> String {
        "Ghost".to_string()
    }
    fn power_draw(&self) -> u32 {
        15
    }
}

fn total_draw(loadout: &[Box<dyn Program>]) -> u32 {
    loadout.iter().map(|p| p.power_draw()).sum()
}

fn over_budget(loadout: &[Box<dyn Program>], budget: u32) -> bool {
    total_draw(loadout) > budget
}

fn main() {
    let deck: Vec<Box<dyn Program>> = vec![Box::new(Icepick), Box::new(Siphon), Box::new(Ghost)];
    for p in &deck {
        println!("[deck] slotted {} ({} MW)", p.name(), p.power_draw());
    }
    println!("[deck] total draw: {} MW", total_draw(&deck));
    println!("[deck] over a 60 MW budget? {}", over_budget(&deck, 60));
}
