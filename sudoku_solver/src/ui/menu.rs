use crate::ui::user_input;


// Main Menu
pub fn main_menu() {
    loop {
        println!("Main Menu");
        println!("1. Solve from file");
        println!("2. Solve from String");
        println!("0. Exit");
        println!();

        let choice: usize = user_input::get_usize();

        match choice {
            1 => (),
            2 => (),
            0 => break,
            _ => continue,
        }
    }
    

}

