#![allow(unused)]
use std::fs;
extern crate libc;
use std::fmt;
use std::fs::read_to_string;
use std::{env, fmt::Display};

struct NanoFetch {
    username: String,
    hostname: String,
    system: String,
    kernel: String,
    cpu: String,
    topology: String,
    desktop: String,
    session_type: String,
    terminal: String,
    editor: String,
    memory: String,
    shell: String,
    uptime: String,
    colors: String,
}

impl NanoFetch {
    pub fn fetch() -> Self {
        Self {
            username: get_username(),
            hostname: get_hostname(),
            system: get_system(),
            kernel: get_kernel(),
            cpu: get_cpu(),
            topology: get_topology(),
            desktop: get_desktop(),
            session_type: get_session_type(),
            terminal: get_terminal(),
            editor: get_editor(),
            memory: get_memory(),
            shell: get_shell(),
            uptime: get_uptime(),
            colors: get_colors(),
        }
    }
}

impl Display for NanoFetch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{username}@{hostname}\n 
                system: {system}\n
                kernel: {kernel}\n
                cpu: {cpu}\n
                topoloty: {topology}\n 
                desktop: {desktop}({session_type})\n
                terminal: {terminal}\n
                editor: {editor}\n
                memory: {memory}\n
                shell: {shell} \n
                uptime: {uptime}",
            username = self.username,
            hostname = self.hostname,
            system = self.system,
            kernel = self.kernel,
            cpu = self.cpu,
            topology = self.topology,
            desktop = self.desktop,
            terminal = self.terminal,
            editor = self.editor,
            session_type = self.session_type,
            memory = self.memory,
            shell = self.shell,
            uptime = self.uptime,
        )
    }
}

fn get_editor() -> String {
    env::var("EDITOR").unwrap_or("Unknown".to_string())
}

fn get_terminal() -> String {
    env::var("TERM_PROGRAM").unwrap_or("Unknown".to_string())
}

fn get_colors() -> String {
    env::var("COLORTERM").unwrap_or("Unknown".to_string())
}

fn get_session_type() -> String {
    env::var("XDG_SESSION_TYPE").unwrap_or("Unknown".to_string())
}

fn get_system() -> String {
    env::var("system").unwrap_or("Unknown".to_string())
}

fn get_kernel() -> String {
    // https://man.archlinux.org/man/proc_version.5.en
    let kernal = fs::read_to_string("/proc/sys/kernel/ostype").unwrap_or("Unknown".to_string());
    let version = fs::read_to_string("/proc/sys/kernel/osrelease").unwrap_or("Unknown".to_string());
    format!("{kernal} {version}")
}

fn get_cpu() -> String {
    todo!()
}

fn get_topology() -> String {
    //  /proc/cpuinfo
    todo!("need to write parser")
}

fn get_memory() -> String {
    todo!()
}

fn get_uptime() -> String {
    let Ok(content) = fs::read_to_string("/proc/uptime") else {
        return String::from("Unknown");
    };

    let Some(first_field) = content.split_whitespace().next() else {
        return String::from("Unknown");
    };

    let Ok(total_seconds) = first_field.parse::<f64>() else {
        return String::from("Unknown");
    };

    let total_seconds = total_seconds as u64;

    let days = total_seconds / 86400;
    let hours = (total_seconds % 86400) / 3600;
    let minutes = (total_seconds % 3600) / 60;

    if days > 0 {
        format!("{days}d {hours}h {minutes}m")
    } else if hours > 0 {
        format!("{hours}h {minutes}m")
    } else {
        format!("{minutes}m")
    }
}

fn get_desktop() -> String {
    env::var("XDG_CURRENT_DESKTOP").unwrap_or("Unknown".to_string())
}

fn get_username() -> String {
    env::var("USER").unwrap_or("Unknown".to_string())
}

fn get_hostname() -> String {
    env::var("HOSTNAME").unwrap_or("Unknown".to_string())
}

fn get_shell() -> String {
    let shell_path = env::var("SHELL");
    if let Ok(shell_path) = shell_path {
        let shell = shell_path.split('/').last().unwrap().to_string();
        shell
    } else {
        String::from("Unknown")
    }
}

fn main() {
    let fetch = NanoFetch::fetch();
    println!("{}", fetch);
}
