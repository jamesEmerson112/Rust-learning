#[path = "../lesson33/exercise_service_log.rs"]
mod service_log;

pub use service_log::ServiceLog;

fn main() {
    let mut log = ServiceLog::new();
    log.add_service("Mai", 4500);
    log.add_service("Mai", 5500);

    println!("Mai avg revenue: {:?}", log.average_revenue("Mai"));
}
