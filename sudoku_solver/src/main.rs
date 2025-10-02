//! # Sudoku Solver
//! 
//! CLI Tool for assisting users in solving sudoku puzzles.
//! 
//! # Usage
//! 
//! Syntax: ```cargo run [optional filename]```
//! 
//! If a filename is supplied as an argument, the program will try to read that file into a string,
//! which is then processed into a sudoku board to be solved. Else, the program calls ```main_menu()```.


mod ui;
mod sudoku;
mod validation;

use std::env;

use crate::sudoku::sudoku::Sudoku;
use crate::ui::argument_parsing::*;
use crate::ui::menu::main_menu;



fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if an argument was supplied at startup. If so, we can solve and exit.    
    if args.len() - 1 == 1 {
        let mut board: Sudoku = Sudoku::from_string(load_from_arge());
        board.solve();
    }

    else {
        main_menu();
    }
}
