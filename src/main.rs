use std::fmt::Display;
use std::path::PathBuf;
use std::{fmt, fs};

mod cli;
mod colors;
mod cpu;
mod memory;
mod storage;
mod uptime;
mod utils;

use clap::Parser;
use cli::Args;
use cpu::CpuInfo;
use memory::MemoryInfo;
use storage::StorageInfo;

const LOGO: &[&str] = &[
    "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣾⣻⣥⣴⣾⣛⠉⠀⠀⠀⠀",
    "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣟⡟⠛⠒⠶⣌⠉⠻⣶⣀⠀⠀",
    "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣤⣤⣤⣀⠀⠀⠀⠀⠀⢀⣤⣄⠀⠀⣾⣿⠿⠷⣄⠀⢠⣼⡇⠀⠹⣿⣆⠀",
    "⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡴⠋⠁⠀⠀⠀⠉⢳⡄⠀⠀⢠⠏⠀⠉⠀⣼⡟⠀⠀⠀⠀⣀⣤⣅⠀⠀⠀⠉⣿⡆",
    "⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⡟⠂⠀⠀⠀⣀⡀⢨⡇⠀⣤⣜⡶⠆⠀⠀⣿⠀⠀⠀⠀⢰⣿⠙⠛⠀⠀⠀⣠⡿⠀",
    "⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠀⠀⠀⠀⠀⠻⠿⠟⠁⠀⠀⣿⠁⢰⡏⠀⢿⡀⠀⠀⠀⠀⠻⣤⣀⣾⣿⣿⠿⠁⠀",
    "⠀⢀⡀⠀⠀⠀⠀⠀⠀⢿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣤⣾⡆⠀⣨⣷⣦⣄⢸⠟⢀⣈⠉⠛⠉⠁⠀⠀⠀",
    "⡾⠋⣿⠀⠀⠀⠀⠀⠀⠀⠻⣄⠀⡀⠀⠀⠀⠀⠀⠀⠀⠀⢙⡿⣷⡐⣧⠉⢧⡈⠻⣇⣟⠙⠁⠀⢀⣤⠔⠀⠀",
    "⢧⠀⠀⠀⠀⠀⠀⠀⠀⣠⡄⠈⠙⠻⠷⣦⣀⣀⠀⠀⠀⠐⠛⠛⠿⣷⡀⠀⠈⠻⣶⡈⢿⡇⠀⠀⣾⠁⠀⠀⠀",
    "⠈⠶⣆⠀⠀⠀⠀⢰⠀⠙⠳⠦⣤⣤⣠⣤⡬⣭⣿⣿⣿⣿⣶⣾⣷⣦⣿⣦⣄⠀⢈⢿⡘⣷⠀⢠⣿⠀⠀⠀⠀",
    "⠀⠀⠈⠙⠒⠂⠆⣿⡷⠆⠒⠚⠛⣉⡵⠟⠋⠉⠀⠀⠀⠀⠀⣰⣾⠿⢿⣯⣿⣷⣼⣦⠳⡜⣆⠘⣿⡄⠀⠀⠀",
    "⠀⠀⠀⠀⠀⠀⠀⠋⠀⠀⠀⣠⡾⠋⠀⠀⠀⠀⠀⠀⠀⠀⢸⡋⠀⠀⠀⢀⡎⠉⠻⢿⣷⡹⣾⠆⣾⣿⡀⠀⠀",
    "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⡿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢁⡀⠀⣀⣸⣿⡆⡀⠀⠙⢿⣞⠆⠻⣿⣧⠀⠀",
    "⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⡏⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢈⢿⡏⠉⣼⠟⠁⠉⠙⠶⣄⠹⣇⠀⠙⣿⡆⠀",
    "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣇⠀⠘⣻⡆⠀⠀⠀⠀⠀⠀⠀⠀⡜⠈⠁⡀⠛⠀⠀⠀⢠⣤⣌⣳⡜⢧⣸⣼⣷⠀",
    "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠳⠾⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣄⣈⣿⠀⢀⣶⠿⠛⠛⠛⠻⢿⣿⣝⣿⣿⡆",
    "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠁⠀⠘⠁⠀⠀⠀⠀⠀⠀⠙⢿⣟⣿⡇",
    "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣴⠶⠶⣦⣄⠀⠀⠀⣠⣤⣄⡀⠀⠈⣿⣿⡇",
    "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⡾⡫⢤⡀⠀⠀⠹⣧⡀⠘⠇⠀⠙⢿⣤⡀⣿⣿⡇",
    "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠛⠀⠀⣠⣤⠀⠀⣹⡇⠀⠀⠀⠀⠘⣿⣇⣿⡟⠀",
    "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⡀⠀⠀⣻⣯⣀⣴⠟⠁⠀⠀⠀⠀⢸⣿⣿⡿⠀⠀",
    "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⣧⠀⠈⠋⠉⠉⠁⠀⠀⠀⠀⠀⣠⣾⣿⠋⠀⠀⠀",
    "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⣷⣤⣀⣀⣴⠇⠀⠀⣀⣤⣾⡿⠟⠁⠀⠀⠀⠀",
    "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⠿⠿⠿⠿⠟⠟⠛⠉⠀⠀⠀⠀⠀⠀⠀",
];

struct NanoFetch {
    username: String,
    hostname: String,
    system: String,
    kernel: String,
    cpu_info: CpuInfo,
    desktop: String,
    session_type: String,
    terminal: String,
    editor: String,
    memory_info: MemoryInfo,
    storage_info: Vec<StorageInfo>,
    shell: String,
    uptime: String,
    colors: String,
    logo: Vec<String>,
}

impl NanoFetch {
    pub fn fetch(file_path: Option<PathBuf>) -> Self {
        let mut logo = Vec::new();
        if let Some(path) = file_path {
            if let Ok(content) = fs::read_to_string(path) {
                content.lines().for_each(|line| logo.push(line.to_string()))
            }
        }

        if logo.len() == 0 {
            LOGO.iter().for_each(|item| logo.push(item.to_string()));
        }

        Self {
            username: utils::get_username(),
            hostname: utils::get_hostname(),
            system: utils::get_system(),
            kernel: utils::get_kernel(),
            cpu_info: CpuInfo::get_cpu_info(),
            desktop: utils::get_desktop(),
            session_type: utils::get_session_type(),
            terminal: utils::get_terminal(),
            editor: utils::get_editor(),
            memory_info: MemoryInfo::get_memory_info(),
            storage_info: StorageInfo::get_storage_info(),
            shell: utils::get_shell(),
            uptime: uptime::get_uptime(),
            colors: utils::get_colors(),
            logo,
        }
    }
}

impl Display for NanoFetch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header = format!(
            "{}{}{}{}@{}{}{}{}",
            colors::BOLD_YELLOW,
            self.username,
            colors::RESET,
            colors::BRIGHT_RED,
            colors::RESET,
            colors::BRIGHT_GREEN,
            self.hostname,
            colors::RESET,
        );

        let mut info: Vec<String> = vec![
            header,
            format!(
                "{blue}{:13}{reset}  {}",
                "OS",
                self.system,
                blue = colors::BLUE,
                reset = colors::RESET
            ),
            format!(
                "{blue}{:13}{reset}  {}",
                "Kernel",
                self.kernel,
                blue = colors::BLUE,
                reset = colors::RESET
            ),
            format!(
                "{blue}{:13}{reset}  {} @ {}",
                "CPU",
                self.cpu_info.model,
                self.cpu_info.max_freq,
                blue = colors::BLUE,
                reset = colors::RESET,
            ),
            format!(
                "{blue}{:13}{reset}  {} cores, {} threads",
                "Topology",
                self.cpu_info.cores,
                self.cpu_info.threads,
                blue = colors::BLUE,
                reset = colors::RESET
            ),
            format!(
                "{blue}{:13}{reset}  {} ({})",
                "DE",
                self.desktop,
                self.session_type,
                blue = colors::BLUE,
                reset = colors::RESET
            ),
            format!(
                "{blue}{:13}{reset}  {}",
                "Terminal",
                self.terminal,
                blue = colors::BLUE,
                reset = colors::RESET
            ),
            format!(
                "{blue}{:13}{reset}  {}",
                "Editor",
                self.editor,
                blue = colors::BLUE,
                reset = colors::RESET
            ),
            format!(
                "{blue}{:13}{reset}  {:.2} GiB / {:.2} GiB ({:.0}%)",
                "Memory",
                self.memory_info.used_memory,
                self.memory_info.total_memory,
                self.memory_info.used_percentage,
                blue = colors::BLUE,
                reset = colors::RESET
            ),
            format!(
                "{blue}{:13}{reset}  {}",
                "Shell",
                self.shell,
                blue = colors::BLUE,
                reset = colors::RESET
            ),
            format!(
                "{blue}{:13}{reset}  {}",
                "Uptime",
                self.uptime,
                blue = colors::BLUE,
                reset = colors::RESET
            ),
        ];

        if let Some(root) = self.storage_info.iter().find(|m| m.mount_point == "/") {
            info.push(format!(
                "{blue}{:13}{reset}  {:.2} GiB / {:.2} GiB ({:.0}%)",
                "Storage",
                root.used,
                root.total,
                root.used_percentage,
                blue = colors::BLUE,
                reset = colors::RESET
            ));
        }

        info.push(format!(
            "{blue}{:13}{reset}  {}",
            "Colors",
            self.colors,
            blue = colors::BLUE,
            reset = colors::RESET
        ));

        let logo_width = self.logo.iter().map(|l| l.len()).max().unwrap_or(0);
        let total = info.len().max(self.logo.len());

        for i in 0..total {
            let logo_line = self.logo.get(i);
            let info_line = info.get(i);

            if logo_line.is_none() && info_line.is_none() {
                break;
            }

            if let Some(logo) = logo_line {
                write!(
                    f,
                    "{cyan}{}{reset}  {}",
                    logo,
                    info_line.map(|s| s.as_str()).unwrap_or(""),
                    cyan = colors::CYAN,
                    reset = colors::RESET,
                )?;
            } else {
                write!(
                    f,
                    "{:width$}  {}",
                    "",
                    info_line.map(|s| s.as_str()).unwrap_or(""),
                    width = logo_width
                )?;
            }

            if i < total - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

fn main() {
    let args = Args::parse();
    let fetch = NanoFetch::fetch(args.logo);
    println!("{}", fetch);
}
