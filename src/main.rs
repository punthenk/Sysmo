mod monitor;

use std::time::Duration;
use std::thread;

use crate::monitor::Monitor;

fn main() {
    let mut monitor = Monitor::new();

    loop {
        monitor.refresh();

        print!("\x1B[2J\x1B[1;1H");

        println!("\n=== CPU Usages ===");
        for (i, usage) in monitor.cpu_usage().iter().enumerate() {
            println!("Core {}: {:.2}%", i, usage);
        }

        thread::sleep(Duration::from_secs(1));
    }
}
