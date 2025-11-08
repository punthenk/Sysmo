use std::{collections::HashMap, usize};

use sysinfo::{Component, Disks, Networks, Process, System};

pub struct Monitor {
    sys: System,
}

impl Monitor {
    pub fn new() -> Self {
        Self {
            sys: System::new_all(),
        }
    }

    pub fn processes(&self) -> Vec<(String, f32, u64)> {
        self.sys.processes()
            .values()
            .map(|process| {
                (
                    process.name().to_string_lossy().to_string(),
                    process.cpu_usage(),
                    process.memory(),
                )
            })
            .collect()
    }

    pub fn processes_grouped(&self) -> Vec<(String, f32, u64, usize)> {
        let mut grouped: HashMap<String, (f32, u64, usize)> = HashMap::new();

        for process in self.sys.processes().values() {
            let name = process.name().to_string_lossy().to_string();
            let entry = grouped.entry(name).or_insert((0.0, 0, 0));
            entry.0 = process.cpu_usage();
            entry.1 = process.memory();
            entry.2 += 1;
        }

        grouped.into_iter()
            .map(|(name, (cpu, mem, count))| (name, cpu, mem, count))
            .collect()
    }

    pub fn total_memory(&self) -> u64 {
        self.sys.total_memory()
    }

    pub fn used_memory(&self) -> u64 {
        self.sys.used_memory()
    }
    
    pub fn availible_memory(&self) -> u64 {
        self.sys.available_memory()
    }

    pub fn cpu_usage(&self) -> Vec<f32> {
        self.sys.cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage())
            .collect()
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_all();
    }

    pub fn uptime_days(&self) -> f32 {
        System::uptime() as f32 / 3600.0 / 24.0
    }

    pub fn system_info(&self) -> String {
        let kernel = System::kernel_version().unwrap_or_else(|| "<unknown>".to_owned());
        let os_version = System::long_os_version().unwrap_or_else(|| "<unknown>".to_owned());
        let distro = System::distribution_id();
        let arch = std::env::consts::ARCH;

        format!(
            "{} kernel {} • {} • {}",
            distro,
            kernel,
            arch,
            os_version
        )
    }
}
