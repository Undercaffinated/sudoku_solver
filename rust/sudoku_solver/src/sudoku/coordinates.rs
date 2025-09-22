
/// Coordinates of any given square on a Sudoku board.
#[derive(Clone, Copy)]
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
}



impl Default for Coordinates {
    fn default() -> Self {
        Self {
            row_index: 0,
            column_index: 0,
        }
    }
}