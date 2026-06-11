use std::fs;

pub struct CpuInfo {
    pub model: String,
    pub cores: String,
    pub threads: String,
}

impl Default for CpuInfo {
    fn default() -> Self {
        Self {
            model: String::from("Unknown"),
            cores: String::from("Unknown"),
            threads: String::from("Unknown"),
        }
    }
}

impl CpuInfo {
    fn new(model: String, cores: String, threads: String) -> Self {
        Self {
            model,
            cores,
            threads,
        }
    }

    pub fn get_cpu_info() -> CpuInfo {
        if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
            let mut threads = cpuinfo.split("\n\n");
            let mut cores = "";
            let mut model = "";
            if let Some(thread) = threads.next() {
                thread.lines().for_each(|line| {
                    if line.contains("cpu cores") {
                        cores = line.split(":").last().unwrap_or("Unknown").trim();
                    }
                    if line.contains("model name") {
                        model = line.split(":").last().unwrap_or("Unknown").trim();
                    }
                })
            }

            Self::new(
                model.to_string(),
                cores.to_string(),
                threads.count().to_string(),
            )
        } else {
            Self::default()
        }
    }
}
