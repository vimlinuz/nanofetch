use std::{fmt, fs};

pub struct MemoryInfo {
    total_memory: f64,
    used_memory: f64,
    used_percentage: f64,
}

impl fmt::Display for MemoryInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "memory: {}GiB / {}GiB ({}%)",
            self.used_memory, self.total_memory, self.used_percentage
        )
    }
}

impl MemoryInfo {
    fn new(total_memory: f64, used_memory: f64) -> Self {
        Self {
            total_memory,
            used_memory,
            used_percentage: ((used_memory / total_memory) * 100.0),
        }
    }

    pub fn get_memory_info() -> MemoryInfo {
        let mut mem_total: f64 = f64::default();
        let mut mem_available: f64 = f64::default();

        if let Ok(content) = fs::read_to_string("/proc/meminfo") {
            content.lines().for_each(|line| {
                if line.contains("MemTotal") {
                    if let Some(total) = line
                        .split(":")
                        .last()
                        .map(|f| f.trim().split_whitespace().next().map(|s| s.to_string()))
                    {
                        mem_total = total
                            .map(|s| s.parse::<f64>().unwrap_or(0.0))
                            .unwrap_or(0.0);
                    }
                };

                if line.contains("MemAvailable") {
                    if let Some(available) = line
                        .split(":")
                        .last()
                        .map(|f| f.trim().split_whitespace().next().map(|s| s.to_string()))
                    {
                        mem_available = available
                            .map(|s| s.parse::<f64>().unwrap_or(0.0))
                            .unwrap_or(0.0);
                    }
                }
            });
        };

        Self::new(mem_total, mem_available)
    }
}
