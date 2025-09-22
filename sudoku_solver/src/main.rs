mod argument_parsing;
mod sudoku;

use crate::sudoku::sudoku::Sudoku;

use crate::argument_parsing::load_file;

fn main() {
    let mut board: Sudoku = Sudoku::from_file(load_file());
    board.init_notes();
    board.solve();
}
