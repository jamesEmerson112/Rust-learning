// THE VAULT RUN — Chapter 1: LOADOUT
// Slot your ICE-breakers. Three different concrete types, one rack: Vec<Box<dyn Program>>.
// Each call to name()/power_draw() dispatches through the vtable at runtime.
pub trait Program {
    fn name(&self) -> String;
    fn power_draw(&self) -> u32;
}

pub struct Icepick;
pub struct Siphon;
pub struct Ghost;

impl Program for Icepick {
    fn name(&self) -> String {
        // TODO: "Icepick"
        String::new()
    }
    fn power_draw(&self) -> u32 {
        // TODO: The Icepick pulls 40 MW.
        0
    }
}

impl Program for Siphon {
    fn name(&self) -> String {
        // TODO: "Siphon"
        String::new()
    }
    fn power_draw(&self) -> u32 {
        // TODO: The Siphon pulls 25 MW.
        0
    }
}

impl Program for Ghost {
    fn name(&self) -> String {
        // TODO: "Ghost"
        String::new()
    }
    fn power_draw(&self) -> u32 {
        // TODO: The Ghost sips 15 MW.
        0
    }
}

pub fn full_loadout() -> Vec<Box<dyn Program>> {
    // TODO: Return all three programs boxed, in order: Icepick, Siphon, Ghost.
    // This is the move that makes trait objects click: three types, one Vec.
    Vec::new()
}

pub fn total_draw(loadout: &[Box<dyn Program>]) -> u32 {
    // TODO: Sum power_draw() across the rack — each element is a different
    // concrete type behind `dyn Program`; the call dispatches at runtime.
    let _ = loadout;
    0
}

pub fn over_budget(loadout: &[Box<dyn Program>], budget: u32) -> bool {
    // TODO: true when the rack pulls MORE than the deck's power budget.
    let _ = (loadout, budget);
    false
}

fn main() {
    let deck = full_loadout();
    println!("[deck] total draw: {} MW (want 80)", total_draw(&deck));
    println!("[deck] over a 60 MW budget? {} (want true)", over_budget(&deck, 60));
}
