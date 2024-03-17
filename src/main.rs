mod screens {
    pub mod welcome_screen;
    pub mod after_efibootmgr;
    pub mod index;
    pub mod os_version;
}

mod utils {
    pub mod disks;
    pub mod mount;
    pub mod version;
}

mod fixes {
    pub mod add_boot_entry;
}



use console::style;

use screens::welcome_screen::welcome_screen;

fn main() {
    println!("{}", style("The Air Operating System").black().bold().on_white());
    println!("Hello, world!");
    welcome_screen();
}
