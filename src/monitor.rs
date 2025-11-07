use sysinfo::{Component, Disks, Networks, System, Process};

pub struct Monitor {
    sys: System,
}

impl Monitor {
    pub fn new() -> Self {
        Self {
            sys: System::new_all(),
        }
    }

    pub fn processes(&self) -> Vec<(Option<&str>, f32, u64)> {
        self.sys.processes()
            .values()
            .map(|process| {
                (
                    process.name().to_str(),
                    process.cpu_usage(),
                    process.memory(),
                )
            })
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
}
