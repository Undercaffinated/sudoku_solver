use crate::ui::user_input;
use crate::sudoku::sudoku::{self, Sudoku};
use crate::validation;


// Main Menu
pub fn main_menu() {
    loop {
        println!("Main Menu");
        println!("1. Solve from file");
        println!("2. Solve from String");
        println!("0. Exit");
        println!();

        let choice: usize = user_input::get_usize();
        println!();

        match choice {
            1 => (),
            2 => solve_from_string(),
            0 => break,
            _ => continue,
        }
    }
    

}



fn solve_from_string() {
    // Print Instructions to the user
    println!("Please enter a string representing the game board without notes.");
    println!("The only allowed characters are the numbers 1-9 and ?");
    println!("The string must be exactly 81 characters long, one for each square.");
    println!();
    println!("String: ");
    let input: String = user_input::get_string();
    println!();

    match validation::game_string::verify_game_string(input) {
        Ok(contents) => Sudoku::from_string(contents).solve(),
        Err(_) => println!("Invalid String"),
    }
}

