use std::process::Command;
use crate::utils::disks::get_uuid;

pub fn is_root_mounted() -> bool {
    let uuid = get_uuid("root").unwrap();

    let command = Command::new("sh")
        .arg("-c")
        .arg(format!("lsblk -o MOUNTPOINT /dev/disk/by-uuid/{} | grep -v MOUNTPOINT", uuid))
        .output()
        .unwrap();

    let output = String::from_utf8(command.stdout).expect("non utf8").replace("\n", "").replace(" ", "");

    if output == "" {
        return false;
    }

    true
}