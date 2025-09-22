use crate::sudoku::block::BlockNumber;
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
        let elements: [Coordinates; 9] = Coordinates::get_row_coords(rows);
        ink_exclusive_notes_in_set(board, elements);
    }

    for columns in 0..9 {
        let elements: [Coordinates; 9] = Coordinates::get_column_coords(columns);
        ink_exclusive_notes_in_set(board, elements);
    }
    
    for blocks in 0..9 {
        let elements: [Coordinates; 9] = Coordinates::get_block_coords(BlockNumber::from_i32(blocks));
        ink_exclusive_notes_in_set(board, elements);
    }
}

fn ink_exclusive_notes_in_set(board: &mut Sudoku, coords: [Coordinates; 9]) {
    let mut counts: [u8; 9] = [0; 9];

    for each in coords {
        add_assign_arrays(&mut counts, &board.get_square(each).notes_array());
    }

    // Check if any elements in the counts array indicate there is only one instance in the set.
    for index in 0..9 {
        if counts[index] == 1 {
            find_and_ink(board, coords, (index + 1) as u8);
        }
    }
}


fn find_and_ink(board: &mut Sudoku, coordinates: [Coordinates; 9], target_note: u8) {
    for each in coordinates {
        if board.get_square(each).check_note(target_note as usize) {
            board.get_square(each).ink(GridState::from_u8(target_note));
        }
    }
}


#[allow(unused)]
/// Left += Right
fn add_assign_arrays(left: &mut [u8; 9], right: &[u8; 9]) {
    for index in 0..9 {
        left[index] = left[index] + right[index];
    }
}



// get_column(board: &mut Sudoku, column_index: usize) -> [&mut GridSquare; 9]
// get_block(board: &mut Sudoku, BlockNumber) -> [&mut GridSquare; 9]




