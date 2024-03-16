use std::process::Command;
use crate::utils::disks::{get_disk, get_uuid};

pub fn add_boot_entry() {
    let disk = get_disk();
    let root = get_uuid("root").unwrap();

    Command::new("sh")
        .arg("-c")
        .arg(format!("efibootmgr --verbose --create --disk {} --part 1 --label airos --loader '\\vmlinuz.efi' --unicode 'initrd=\\initramfs.img rd.vconsole.keymap=us root=UUID={}'", disk, root))
        .output()
        .expect("Failed to execute efibootmgr. Are you root?");
}
