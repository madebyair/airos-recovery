use console::style;
use std::fs;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use crate::screens::index::index;
use crate::utils::disks::{get_disk, get_uuid};
use crate::utils::version::SystemInfo;

pub fn os_version() {
    clearscreen::clear().expect("failed to clear screen");

    let file = fs::read_to_string("/mnt/root/etc/os-release");
    let system = SystemInfo::from_string(file.unwrap().as_str());

    println!("{}", style("Airos version").black().bold().on_white());
    println!("\n{} {} {}", style(system.pretty_name.clone()).bold(), system.codename, style(system.version_id.clone()).white().bold().italic());

    println!("{}", style("\n * Version information").bold());
    println!("{} {}", style("Name: ").bold().cyan(), style(system.name.clone()).white());
    println!("{} {}", style("Full name: ").bold().cyan(), style(system.pretty_name).white());
    println!("{} {}", style("Version: ").bold().cyan(), style(system.version_id).white());
    println!("{} {}", style("Codename: ").bold().cyan(), style(system.codename).white());
    println!("{} {}", style("Website: ").bold().cyan(), style(system.home_url).white());
    println!("{} {}", style("Support: ").bold().cyan(), style(system.support_url).white());
    println!("{} {}", style("Bugzilla: ").bold().cyan(), style(system.bug_report_url).white());
    println!("{} {}", style("License: ").bold().cyan(), style(system.license.clone()).white());

    let root = get_uuid("root").unwrap();
    let boot = get_uuid("boot").unwrap();
    let disk = get_disk();

    println!("{}", style("\n * Disk information").bold());
    println!("{} {}", style("Disk location: ").bold().cyan(), style(disk).white());
    println!("{} {}", style("Root partition: ").bold().cyan(), style(root).white());
    println!("{} {}", style("Boot partition: ").bold().cyan(), style(boot).white());

    println!("\n{}", style(format!("{} is licensed under {}", system.name, system.license)).italic().white());

    let items = ["Go back"];

    Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact()
        .unwrap();

    index();
}