use console::style;

pub fn index() {
    clearscreen::clear().expect("failed to clear screen");

    println!("{}", style("The Air Operating System Recovery").black().bold().on_white());
    println!("{}", style("Welcome to Recovery! What do you wanna do today?").bold());
}