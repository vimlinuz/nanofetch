use crate::colors;
use std::env;
use std::fs;

pub fn get_editor() -> String {
    env::var("EDITOR").unwrap_or("Unknown".to_string())
}

pub fn get_terminal() -> String {
    env::var("TERM_PROGRAM").unwrap_or("Unknown".to_string())
}

pub fn get_colors() -> String {
    let mut colors = String::new();

    for i in 0..8 {
        colors.push_str(&format!("{}{}{}", colors::color256(i), " ", colors::RESET));
    }

    colors
}

pub fn get_session_type() -> String {
    env::var("XDG_SESSION_TYPE").unwrap_or("Unknown".to_string())
}

pub fn get_system() -> String {
    let mut sys = "";
    if let Ok(contents) = fs::read_to_string("/etc/os-release") {
        contents.lines().for_each(|line| {
            if line.contains("PRETTY_NAME") {
                sys = line.split('"').nth(1).unwrap_or("Unknown")
            }
        });
        sys.to_string()
    } else {
        "Unknown".to_string()
    }
}

pub fn get_kernel() -> String {
    // https://man.archlinux.org/man/proc_version.5.en
    let kernal = fs::read_to_string("/proc/sys/kernel/ostype").unwrap_or("Unknown".to_string());
    let version = fs::read_to_string("/proc/sys/kernel/osrelease").unwrap_or("Unknown".to_string());
    format!("{} {}", kernal.trim(), version.trim())
}

pub fn get_desktop() -> String {
    env::var("XDG_CURRENT_DESKTOP").unwrap_or("Unknown".to_string())
}

pub fn get_username() -> String {
    env::var("USER").unwrap_or("Unknown".to_string())
}

pub fn get_hostname() -> String {
    env::var("HOSTNAME")
        .or_else(|_| {
            std::fs::read_to_string("/proc/sys/kernel/hostname").map(|s| s.trim().to_string())
        })
        .unwrap_or_else(|_| "Unknown".to_string())
}

pub fn get_shell() -> String {
    let shell_path = env::var("SHELL");
    if let Ok(shell_path) = shell_path {
        let shell = shell_path
            .split('/')
            .last()
            .unwrap_or("Unknown")
            .to_string();
        shell
    } else {
        String::from("Unknown")
    }
}
