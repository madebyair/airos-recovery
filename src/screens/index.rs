use console::style;

pub fn index() {
    clearscreen::clear().expect("failed to clear screen");

    println!("{} {}", style("The Air Operating System Recovery").black().bold().on_white(), style(format!("(Version {})", env!("CARGO_PKG_VERSION"))).white().italic());
    println!("{}", style("Welcome to Recovery! What do you wanna do today?").bold());
}