use sysinfo::{Component, Disks, Networks, System};

pub struct Monitor {
    sys: System,
}

impl Monitor {
    pub fn new() -> Self {
        Self {
            sys: System::new_all(),
        }
    }

    pub fn total_memory(&self) -> u64 {
        self.sys.total_memory()
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
