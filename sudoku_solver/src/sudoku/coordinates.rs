use crate::sudoku::block::{map_block_to_array_of_coordinates, BlockNumber};


/// Coordinates of any given square on a Sudoku board.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coordinates {
    pub row_index: usize,
    pub column_index: usize,
}



impl Coordinates {
    pub fn from(row_index: usize, column_index: usize) -> Self {
        Self { row_index: row_index, column_index: column_index }
    }
}


impl Coordinates {
    pub fn get_row_coords(row_index: usize) -> [Coordinates; 9] {
        let mut a: [Coordinates; 9] = [Coordinates::default(); 9];

        for each in 0..9 {
            a[each] = Coordinates::from(row_index, each);
        }

        a
    }

    pub fn get_column_coords(column_index: usize) -> [Coordinates; 9] {
        let mut a: [Coordinates; 9] = [Coordinates::default(); 9];

        for each in 0..9 {
            a[each] = Coordinates::from(each, column_index);
        }

        a
    }

    pub fn get_block_coords(identifier: BlockNumber) -> [Coordinates; 9] {
        let unprocessed = map_block_to_array_of_coordinates(identifier);
        let mut processed: [Coordinates; 9] = [Coordinates::default(); 9];

        for index in 0..9 {
            processed[index] = Coordinates::from(unprocessed[index].0, unprocessed[index].1);
        }

        processed
    }
}



impl Default for Coordinates {
    fn default() -> Self {
        Self {
            row_index: 0,
            column_index: 0,
        }
    }
}



// Tests -------------------------------------------------------------------------

#[test]
fn get_block_coords_test() {
    let block_number: BlockNumber = BlockNumber::One;
    let expected_coordinates: [Coordinates; 9] = [
        Coordinates::from(0, 0),
        Coordinates::from(0, 1),
        Coordinates::from(0, 2),

        Coordinates::from(1, 0),
        Coordinates::from(1, 1),
        Coordinates::from(1, 2),

        Coordinates::from(2, 0),
        Coordinates::from(2, 1),
        Coordinates::from(2, 2),
        ];

    assert_eq!(expected_coordinates, Coordinates::get_block_coords(block_number));

}



