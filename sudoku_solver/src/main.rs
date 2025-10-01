mod ui;
mod sudoku;


use crate::sudoku::sudoku::Sudoku;
use crate::ui::argument_parsing::*;

fn main() {
    let mut board: Sudoku = Sudoku::from_string(load_from_arge());
    board.solve();
}
