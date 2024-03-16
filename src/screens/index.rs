use console::style;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use crate::utils::mount::is_root_mounted;

pub fn index() {
    clearscreen::clear().expect("failed to clear screen");

    println!("{} {}", style("The Air Operating System Recovery").black().bold().on_white(), style(format!("(Version {})", env!("CARGO_PKG_VERSION"))).white().italic());
    println!("{}", style("Welcome to Recovery! What do you wanna do today?").bold());

    if is_root_mounted() {
        println!("\n ✅  {} \n", style("Root mounted").green().bold());
    } else {
        println!("\n ❌  {} \n", style("Root unmounted").red().bold());
    }

    let items = ["Mount root", "Reboot"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact()
        .unwrap();
}