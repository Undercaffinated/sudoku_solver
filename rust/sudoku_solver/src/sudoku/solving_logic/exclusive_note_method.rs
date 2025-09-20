use crate::sudoku::sudoku::Sudoku;
use crate::sudoku::grid_square::GridSquare;
use crate::sudoku::grid_state::GridState;

/// If a square in a row, column, or block exclusively contains some note,
/// that square's inked value must be that exclusive value.
/// Example: If only one square in a row has a '1' note, that square must be a 1.
pub fn exclusive_note_method(board: &mut Sudoku) {
    // Check all the rows
    for row in 0..9 {
        // Construct an array that will be used to count instances of notes.
        // In other words, counts[0] is the number of one notes in the set, and so on.
        let mut counts: [u8; 9] = [0; 9];

        for column in 0..9 {
            add_assign_arrays(&mut counts, &board.grid[row][column].notes_array());
        }

        // Check if any elements in the counts array indicate there is only one instance in the set.
        for index in 0..9 {
            if counts[index] == 1 {

                // Find the square in the row that contains the indicated note, and ink its value.
                for square in 0..9 {
                    if board.grid[row][square].check_note(index + 1) {
                        board.grid[row][square].ink(GridState::from_u8((index + 1) as u8))
                    }
                }
            }
        }
    }

}

/// Left += Right
fn add_assign_arrays(left: &mut [u8; 9], right: &[u8; 9]) {
    for index in 0..9 {
        left[index] = left[index] + right[index];
    }
}




