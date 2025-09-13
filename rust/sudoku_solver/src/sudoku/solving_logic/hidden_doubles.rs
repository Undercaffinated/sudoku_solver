// Let a and b both be valid numbers for a Sudoku board.
// Let set S be a row, column or block.

// Given a set S containing two squares whose the only notes are a and b,
// the rest of the squares in S cannot be those value, and the notes
// representing a and b in those other squares can be erased.

// Example:
// Given row: [1, (2,3), (2,3) | ... | ... ], none of the squares after the first | can be 2 or 3.

use std::collections::HashMap;

use crate::sudoku::grid_square::GridSquare;
use crate::sudoku::sudoku::Sudoku;

fn find_hidden_pairs(board: &mut Sudoku) {
    for row in 0..9 {
        let mut vec_1: Vec<Mapping> = Vec::with_capacity(9);

        for column in 0..9 {
            let v = board.grid[row][column];
            let c: Coordinates = Coordinates {
                row_index: row,
                column_index: column,
            };
            let m: Mapping = Mapping {
                value: v,
                coordinates: c,
            };

            if v.notes().len() == 2 {
                vec_1.push(m);
            }
        }

        // Now that the elements of the given row are assembled, we can
        // construct and leverage the hashmap.
    }

    // Check each column
    // Check each block
}

struct Mapping {
    value: GridSquare,
    coordinates: Coordinates,
}

struct Coordinates {
    row_index: usize,
    column_index: usize,
}
