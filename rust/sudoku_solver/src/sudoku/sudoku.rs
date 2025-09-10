use crate::sudoku::grid_square::GridSquare;

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
                temp_grid[row][column] = GridSquare::from_char(input[9 * row + column]);
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
}

impl Default for Sudoku {
    fn default() -> Self {
        Self {
            grid: [[GridSquare::default(); 9]; 9],
        }
    }
}

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
