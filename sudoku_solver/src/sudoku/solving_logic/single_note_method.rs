use crate::sudoku::grid_state::GridState;
use crate::sudoku::sudoku::Sudoku;


/// Performs the single_note_method for each square in the grid.
/// Returns true if changes were made.
pub fn single_note_method(board: &mut Sudoku) -> bool {
    for row in 0..9 {
        for column in 0..9 {
            _single_note_method(board, row, column);
        }
    }

    if board.grid != board.previous_grid {
        return true;
    } else {
        return false;
    }
}

/// When a given square on a sudoku board that only has one possible value it can be,
/// (according to its notes), the inked value of that square must be that note's value.
/// Example: If a square only contains a '1' note, it must be a 1.
fn _single_note_method(board: &mut Sudoku, row: usize, column: usize) {
    // Prevent re-inking a filled-in square.
    if board.grid[row][column].value != GridState::Empty {
        ()
    }

    if board.grid[row][column].has_one_possible_value() {
        let val = board.grid[row][column].only_possible_value();
        println!("Found a {} at location (row: {}, column: {}): Single Note Method.", val.to_char(), row, column);
        board.grid[row][column].value = val;
        board.grid[row][column].remove_all_notes();
    }
}

