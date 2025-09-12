// When a given square on a sudoku board that only has one possible value it can be,
// (according to its notes), the inked value of that square must be that note's value.

// Example: If a square only contains a '1' note, it must be a 1.

use crate::sudoku::grid_state::GridState;
use crate::sudoku::sudoku::Sudoku;

pub fn ink_by_elimination(board: &mut Sudoku, row: usize, column: usize) {
    // Prevent re-inking a filled-in square.
    if board.grid[row][column].value != GridState::Empty {
        ()
    }

    if board.grid[row][column].has_one_possible_value() {
        let val = board.grid[row][column].only_possible_value();
        println!("Found a {} at location ({}, {}): Only one possible note.", val.to_char(), row, column);
        board.grid[row][column].value = val;
        board.grid[row][column].remove_all_notes();
    }
}
