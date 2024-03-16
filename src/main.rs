mod screens {
    pub mod welcome_screen;
}

mod utils {
    pub mod disks;
}


use console::style;

use screens::welcome_screen::welcome_screen;

fn main() {
    println!("{}", style("The Air Operating System").black().bold().on_white());
    println!("Hello, world!");
    welcome_screen();
}
