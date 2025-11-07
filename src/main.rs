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

        println!("\n=== Memory ===");
        println!("Total: {} MB", monitor.total_memory() / 1024 / 1024);
        println!("Used: {} MB", monitor.used_memory() / 1024 / 1024);
        println!("Availible: {} MB", monitor.availible_memory() / 1024 / 1024);

        println!("\n=== Top Processes (by Memory) ===");
        let mut processes = monitor.processes();

        processes.sort_by(|a, b| b.2.cmp(&a.2));

        for (i, (name, cpu, memory)) in processes.iter().take(15).enumerate() {
            println!("{:?}", name);
        }

        thread::sleep(Duration::from_secs(1));
    }
}
