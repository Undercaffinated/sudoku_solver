use crate::sudoku::grid_square::GridSquare;
use crate::sudoku::grid_state::GridState;

use super::block::*;

#[allow(unused)]
pub struct Sudoku {
    // Index 0 is the upper-left corner square.
    // Index 8 is the upper-right corner.
    pub grid: [[GridSquare; 9]; 9],
}

#[allow(unused)]
impl Sudoku {
    pub fn from_file(maybe_contents: Option<String>) -> Self {
        match maybe_contents {
            None => Sudoku::default(),
            Some(content) => Sudoku::from_string(content),
        }
    }

    fn from_string(input: String) -> Self {
        let input: Vec<char> = input.chars().collect();
        let mut temp_grid: [[GridSquare; 9]; 9] = [[GridSquare::default(); 9]; 9];

        for row in 0..9 {
            for column in 0..9 {
                temp_grid[row][column] =
                    GridSquare::from_grid_state(GridState::from_char(input[9 * row + column]));
            }
        }

        Self { grid: temp_grid }
    }

    pub fn print(&self) {
        let mut intermediate: [[char; 12]; 11] = [['u'; 12]; 11];
        println!();

        for row in 0..9 {
            for column in 0..9 {
                let target_row: usize = translate_for_printing(row);
                let target_column: usize = translate_for_printing(column);
                intermediate[target_row][target_column] = self.grid[row][column].to_char();
                insert_sp_chars_for_printing(&mut intermediate);
            }
        }

        for row in 0..11 {
            for column in 0..12 {
                print!("{}", intermediate[row][column]);
            }
        }
        println!();
    }

    /// Writes a final value into a particular square on the sudoku board.
    /// Name comes from the idea that a pen is permanent.
    fn ink_square(&mut self, row: usize, column: usize, value: GridState) {
        self.grid[row][column] = GridSquare::from_grid_state(value);
        // This is redundant: self.grid[row][column].remove_all_notes();
    }

    fn remove_note(&mut self, row: usize, column: usize, value: GridState) {
        self.grid[row][column].remove_note(value);
    }
}

impl Default for Sudoku {
    fn default() -> Self {
        Self {
            grid: [[GridSquare::default(); 9]; 9],
        }
    }
}

/// Maps a given index on a working sudoku board onto the correct index of a printable board.
fn translate_for_printing(index: usize) -> usize {
    match index {
        0 | 1 | 2 => index,
        3 | 4 | 5 => index + 1,
        6 | 7 | 8 => index + 2,
        _ => panic!(),
    }
}

fn insert_sp_chars_for_printing(printer_object: &mut [[char; 12]; 11]) {
    for row in 0..11 {
        for column in 0..12 {
            let target_coordinates = (row, column);
            match target_coordinates {
                (3, 3) => printer_object[row][column] = '+',
                (3, 7) => printer_object[row][column] = '+',
                (7, 3) => printer_object[row][column] = '+',
                (7, 7) => printer_object[row][column] = '+',

                (3, 0..11) => printer_object[row][column] = '-',
                (7, 0..11) => printer_object[row][column] = '-',
                (0..11, 3) => printer_object[row][column] = '|',
                (0..11, 7) => printer_object[row][column] = '|',

                (0..11, 11) => printer_object[row][column] = '\n',

                (_, _) => continue,
            }
        }
    }
}

/// After initial construction, all empty squares on a sudoku board will claim they can
/// be any number, based on their notes. This removes all impossible note values based on
/// the inked values on the board. This function is expensive and should be called sparingly.
fn init_notes(grid: &mut [[GridSquare; 9]; 9]) {
    // For each square on the board,
    // Check if it contains an inked value
    // If so, remove that value from the notes of all containing groups.
    for row in 0..9 {
        for column in 0..9 {
            match grid[row][column].value {
                // If the square is empty, it doesn't contribute to removing notes.
                GridState::Empty => continue,

                // If the square contains an inked value, we need to erase notes s.t.
                // all squares in the same row, column, and block cannot be that value.
                _ => remove_conflicting_notes(grid, row, column, grid[row][column].value),
            }
        }
    }
}

fn remove_conflicting_notes(
    grid: &mut [[GridSquare; 9]; 9],
    row: usize,
    column: usize,
    note_to_remove: GridState,
) {
    // Remove notes from same row
    for columns in 0..9 {
        grid[row][columns].remove_note(note_to_remove);
    }
    // Remove notes from same column
    for rows in 0..9 {
        grid[rows][column].remove_note(note_to_remove);
    }
    // Remove notes from same 3x3 block
    let targets: [(usize, usize); 9] =
        map_block_to_array_of_coordinates(map_coordinates_to_block(row, column));

    for (r, c) in targets {
        grid[r][c].remove_note(note_to_remove);
    }
}
