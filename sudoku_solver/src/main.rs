mod ui;
mod sudoku;

use std::env;

use crate::sudoku::sudoku::Sudoku;
use crate::ui::argument_parsing::*;
use crate::ui::menu;


fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if an argument was supplied at startup. If so, we can solve and exit.    
    if args.len() - 1 == 1 {
        let mut board: Sudoku = Sudoku::from_string(load_from_arge());
        board.solve();
    }

    // Else, main menu.
    menu::main_menu();
}
