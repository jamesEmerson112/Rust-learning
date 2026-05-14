#[path = "../lesson33/example_service_log.rs"]
mod service_log;

use service_log::ServiceLog;

fn main() {
    let mut log = ServiceLog::new();
    log.add_service("Mai", 4500);
    log.add_service("Mai", 3500);
    log.add_service("Linh", 6000);

    if let Some(avg) = log.average_revenue("Mai") {
        println!("Mai average revenue: {avg:.2} cents");
    }

    match log.average_revenue("Trang") {
        Some(avg) => println!("Trang average revenue: {avg:.2}"),
        None => println!("No services logged for Trang"),
    }
}
