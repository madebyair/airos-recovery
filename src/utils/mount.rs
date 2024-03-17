use std::process::Command;
use std::fs;
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

pub fn mount_root() {
    let uuid = get_uuid("root").unwrap();

    fs::create_dir("/mnt/root").unwrap_or(());

    Command::new("sh")
        .arg("-c")
        .arg(format!("mount /dev/disk/by-uuid/{} /mnt/root", uuid))
        .output()
        .unwrap();
}

pub fn umount_root() {
    Command::new("sh")
        .arg("-c")
        .arg("umount /mnt/root")
        .output()
        .unwrap();

    fs::remove_dir("/mnt/root").unwrap();
}