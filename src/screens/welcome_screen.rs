use console::style;
use dialoguer::{theme::ColorfulTheme, Select};

pub fn welcome_screen() {
    clearscreen::clear().expect("failed to clear screen");
    println!("{}", style("The Air Operating System Recovery").black().bold().on_white());

    println!("{}", style("\n * What happened?").bold());
    println!("You are currently at Airos recovery. You could be here for many reasons, here are a few:");
    println!("\n- You are at new computer, with disk from old.");
    println!("- Your computer UEFI has been restarted.");
    println!("- Your airos update was failed.");
    println!("- You want to fix your airos erros {}", style("(requires error code)").white().italic());
    println!("- You are professional user, and you know what are you doing {}", style("(requires brain)").white().italic());

    println!("{}", style("\n * What should I do now?").bold());
    println!("Now you need select one of the option shown below");

    println!("{}", style("Use the up and down arrows, on your keyboard to navigate").white().bold());

    let items = ["The system stopped loading and this is loading instead", "I'm nerd open, the doors!"];

    let _selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact()
        .unwrap();
}