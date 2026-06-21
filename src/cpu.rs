use std::fs;

pub struct CpuInfo {
    pub model: String,
    pub cores: String,
    pub threads: String,
    pub max_freq: String,
}

impl Default for CpuInfo {
    fn default() -> Self {
        Self {
            model: String::from("Unknown"),
            cores: String::from("Unknown"),
            threads: String::from("Unknown"),
            max_freq: String::from("Unknown"),
        }
    }
}

impl CpuInfo {
    fn new(model: String, cores: String, threads: String, max_freq: String) -> Self {
        Self {
            model,
            cores,
            threads,
            max_freq,
        }
    }

    pub fn get_max_freq() -> String {
        // // it gives info in kilo hertz
        fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq")
            .map(|kilo_freq| {
                kilo_freq
                    .trim()
                    .parse::<f64>()
                    .map(|freq_khz| format!("{} GHz", (freq_khz / 1000000.0)))
                    .unwrap_or("Unknown".to_string())
            })
            .unwrap_or("Unknown".to_string())
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
            let max_freq = Self::get_max_freq();

            Self::new(
                model.to_string(),
                cores.to_string(),
                threads.count().to_string(),
                max_freq,
            )
        } else {
            Self::default()
        }
    }
}
