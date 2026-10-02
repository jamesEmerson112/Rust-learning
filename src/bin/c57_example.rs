// Box<dyn Trait> = runtime polymorphism: store different concrete types behind one trait and
// dispatch through a vtable at runtime (generics, by contrast, pick the type at compile time).
// Coming from C: it's the struct-of-function-pointers vtable you'd hand-roll to put unlike
// objects in one array and call them uniformly — here the compiler builds the vtable for you.
//
// RUST GENERAL HOSPITAL: a bedside monitor runs a mix of sensors. Different types, one
// rack — every sensor answers name() and power_draw() through the vtable.
trait Sensor {
    fn name(&self) -> String;
    fn power_draw(&self) -> u32;
}

struct HeartRate;
struct BloodOxygen;
struct BloodPressure;

impl Sensor for HeartRate {
    fn name(&self) -> String {
        "Heart Rate".to_string()
    }
    fn power_draw(&self) -> u32 {
        40
    }
}

impl Sensor for BloodOxygen {
    fn name(&self) -> String {
        "Blood Oxygen".to_string()
    }
    fn power_draw(&self) -> u32 {
        25
    }
}

impl Sensor for BloodPressure {
    fn name(&self) -> String {
        "Blood Pressure".to_string()
    }
    fn power_draw(&self) -> u32 {
        15
    }
}

fn total_draw(monitor: &[Box<dyn Sensor>]) -> u32 {
    monitor.iter().map(|s| s.power_draw()).sum()
}

fn over_budget(monitor: &[Box<dyn Sensor>], budget: u32) -> bool {
    total_draw(monitor) > budget
}

fn main() {
    let monitor: Vec<Box<dyn Sensor>> =
        vec![Box::new(HeartRate), Box::new(BloodOxygen), Box::new(BloodPressure)];
    for s in &monitor {
        println!("[monitor] {} sensor ({} mW)", s.name(), s.power_draw());
    }
    println!("[monitor] total draw: {} mW", total_draw(&monitor));
    println!("[monitor] over a 60 mW battery budget? {}", over_budget(&monitor, 60));
}
