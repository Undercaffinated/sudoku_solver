use crate::sudoku::coordinates::Coordinates;
use crate::sudoku::grid_square::GridSquare;
use crate::sudoku::sudoku::Sudoku;
use crate::sudoku::grid_state::GridState;

/// If a square in a row, column, or block exclusively contains some note,
/// that square's inked value must be that exclusive value.
/// Example: If only one square in a row has a '1' note, that square must be a 1.
pub fn exclusive_note_method(board: &mut Sudoku) {
    // Check all the rows
    for rows in 0..9 {
        let row_elements: [Coordinates; 9] = Coordinates::get_row_coords(rows);
        ink_exclusive_notes_in_set(board, row_elements);
    }

    // TODO: for columns ...
    // TODO: for blocks...
}

fn ink_exclusive_notes_in_set(board: &mut Sudoku, coords: [Coordinates; 9]) {
    // Construct an array that will be used to count instances of notes.
    // In other words, counts[0] is the number of one notes in the set, and so on.
    let mut counts: [u8; 9] = [0; 9];

    for each in set {
        add_assign_arrays(&mut counts, &each.notes_array());
    }

    // Check if any elements in the counts array indicate there is only one instance in the set.
    
}


fn find_and_ink(set: [&mut GridSquare; 9], target_note: u8) {
    for each in set {
        if each.check_note(target_note as usize) {
            each.ink(GridState::from_u8(target_note));
        }
    }
}


/// Left += Right
fn add_assign_arrays(left: &mut [u8; 9], right: &[u8; 9]) {
    for index in 0..9 {
        left[index] = left[index] + right[index];
    }
}



// get_column(board: &mut Sudoku, column_index: usize) -> [&mut GridSquare; 9]
// get_block(board: &mut Sudoku, BlockNumber) -> [&mut GridSquare; 9]




