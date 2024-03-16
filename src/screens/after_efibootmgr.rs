use console::style;
use dialoguer::Confirm;
use dialoguer::theme::ColorfulTheme;
use crate::fixes::add_boot_entry::add_boot_entry;
use std::process::Command;

pub fn after_efibootmgr() {
    clearscreen::clear().expect("Failed to clearscreen");

    add_boot_entry();

    println!("{}", style("Your computer should be fixed now!").black().bold().on_white());

    let confirmation = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("To reboot to system, press Enter")
        .default(true)
        .interact()
        .unwrap();

    if confirmation {
        Command::new("sh")
            .arg("-c")
            .arg("reboot")
            .output().expect("Failed to reboot");
    } else {
        println!("To reboot system now, you need press and hold power key.");
    }
}