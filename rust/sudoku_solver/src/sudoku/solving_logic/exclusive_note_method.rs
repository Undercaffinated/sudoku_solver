use crate::sudoku::sudoku::Sudoku;

/// If a given note value (i.e. a one, etc.) can only be found in one square in a given group,
/// that square must contain that exclusive value.
/// Example: If only one square in a row has a '1' note, that square must be a 1.
pub fn exclusive_note_method(board: &mut Sudoku, row: usize, column: usize) {
    // Method 1: Count all the notes in a given set, then if some notes ony appear once, then the squares
    // with those notes can be solved.
    
    // Given some square (row, column), determine if it is the only square in that set that contains a
    // particular note.
}




