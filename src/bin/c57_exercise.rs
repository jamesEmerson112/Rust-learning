// RUST GENERAL HOSPITAL — Ward Equipment
// Wire up the bedside monitor. Three different sensor types, one rack: Vec<Box<dyn Sensor>>.
// Each call to name()/power_draw() dispatches through the vtable at runtime.
pub trait Sensor {
    fn name(&self) -> String;
    fn power_draw(&self) -> u32;
}

pub struct HeartRate;
pub struct BloodOxygen;
pub struct BloodPressure;

impl Sensor for HeartRate {
    fn name(&self) -> String {
        // TODO: "Heart Rate"
        "Heart Rate".to_string()
    }
    fn power_draw(&self) -> u32 {
        // TODO: The heart-rate sensor draws 40 mW.
        40
    }
}

impl Sensor for BloodOxygen {
    fn name(&self) -> String {
        // TODO: "Blood Oxygen"
        "Blood Oxygen".to_string()
    }
    fn power_draw(&self) -> u32 {
        // TODO: The blood-oxygen sensor draws 25 mW.
        25
    }
}

impl Sensor for BloodPressure {
    fn name(&self) -> String {
        // TODO: "Blood Pressure"
        "Blood Pressure".to_string()
    }
    fn power_draw(&self) -> u32 {
        // TODO: The blood-pressure cuff draws 15 mW.
        15
    }
}

pub fn full_monitor() -> Vec<Box<dyn Sensor>> {
    // TODO: Return all three sensors boxed, in order: HeartRate, BloodOxygen, BloodPressure.
    // This is the move that makes trait objects click: three types, one Vec.
    vec![Box::new(HeartRate), Box::new(BloodOxygen), Box::new(BloodPressure)]
}

pub fn total_draw(monitor: &[Box<dyn Sensor>]) -> u32 {
    // TODO: Sum power_draw() across the monitor — each element is a different
    // concrete type behind `dyn Sensor`; the call dispatches at runtime.
    monitor.iter().map(|p| p.power_draw()).sum()
}

pub fn over_budget(monitor: &[Box<dyn Sensor>], budget: u32) -> bool {
    // TODO: true when the sensors draw MORE than the monitor's battery budget.
    total_draw(monitor) > budget
}

fn main() {
    let monitor = full_monitor();
    println!("[monitor] total draw: {} mW (want 80)", total_draw(&monitor));
    println!("[monitor] over a 60 mW budget? {} (want true)", over_budget(&monitor, 60));
}
