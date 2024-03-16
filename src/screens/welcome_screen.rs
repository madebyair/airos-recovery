use console::style;

pub fn welcome_screen() {
    clearscreen::clear().expect("failed to clear screen");
    println!("{}", style("The Air Operating System Recovery").black().bold().on_white());

    println!("{}", style("\n * What happened?").bold());
    println!("You are currently at Airos recovery. You could be here for many reasons, here are a few:");
    println!("\n- You are at new computer, with disk from old.");
    println!("- Your computer UEFI has been restarted.");
    println!("- Your airos update was failed.");
    println!("- You want to fix your airos erros {}", style("(requires error code)").black());
    println!("- You are professional user, and you know what are you doing {}", style("(requires brain)").black());
}