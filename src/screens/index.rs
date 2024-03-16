use console::style;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use crate::utils::mount::{is_root_mounted, mount_root, umount_root};

pub fn index() {
    clearscreen::clear().expect("failed to clear screen");

    println!("{} {}", style("The Air Operating System Recovery").black().bold().on_white(), style(format!("(Version {})", env!("CARGO_PKG_VERSION"))).white().italic());
    println!("{}", style("Welcome to Recovery! What do you wanna do today?").bold());

    let root_mounted: bool = is_root_mounted();

    if root_mounted {
        println!("\n ✅  {} \n", style("Root mounted").green().bold());
    } else {
        println!("\n ❌  {} \n", style("Root unmounted").red().bold());
    }

    let mut items: Vec<&str> = Vec::new();

    let items_unmounted = ["Mount root", "Reboot"];

    let items_mounted = ["Unmount root", "Reboot"];

    if root_mounted {
        items.extend_from_slice(&items_mounted);
    } else {
        items.extend_from_slice(&items_unmounted);
    }


    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact()
        .unwrap();

    if root_mounted {
        if selection == 0 {
            umount_root();
            index();
        }
    } else {
        if selection == 0 {
            mount_root();
            index();
        }
    }
}