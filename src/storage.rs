use libc;
use std::ffi::CString;
use std::fs;

fn into_gb(size: u64) -> f64 {
    size as f64 / (1024.0 * 1024.0 * 1024.0)
}

pub struct StorageInfo {
    pub mount_point: String,
    pub total: f64,
    pub used: f64,
    pub used_percentage: f64,
}

impl StorageInfo {
    pub fn get_storage_info() -> Vec<StorageInfo> {
        let real_fs = [
            "ext4", "ext3", "ext2", "xfs", "btrfs", "zfs", "ntfs", "vfat", "fuseblk", "f2fs",
            "jfs", "reiserfs",
        ];

        let Ok(content) = fs::read_to_string("/proc/mounts") else {
            return Vec::new();
        };

        let mut seen_devices: Vec<u64> = Vec::new();

        content
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 3 || !real_fs.contains(&parts[2]) {
                    return None;
                }
                let mount_point = parts[1];
                if matches!(mount_point, "/dev" | "/proc" | "/sys") {
                    return None;
                }

                let c_path = CString::new(mount_point).ok()?;
                let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
                if unsafe { libc::statvfs(c_path.as_ptr(), &mut stat) } != 0 {
                    return None;
                }

                let fs_id = stat.f_fsid as u64;
                if seen_devices.contains(&fs_id) {
                    return None;
                }
                seen_devices.push(fs_id);

                let block_size = stat.f_frsize as u64;
                let total = stat.f_blocks as u64 * block_size;
                let free = stat.f_bfree as u64 * block_size;
                let used = total - free;

                Some(StorageInfo {
                    mount_point: mount_point.to_string(),
                    total: into_gb(total),
                    used: into_gb(used),
                    used_percentage: (used as f64 / total as f64) * 100.0,
                })
            })
            .collect()
    }
}
