use std::default;

mod solving_logic;

pub struct Sudoku {
    grid: [GridSquare; 81],
}

impl Sudoku {
    fn print() {}
}

struct GridSquare {
    // The inked in value
    value: GridState,

    // Some solving patterns require knowing what values a square might contain.
    // These values are intended to serve that purpose.
    one: bool,
    two: bool,
    three: bool,
    four: bool,
    five: bool,
    six: bool,
    seven: bool,
    eight: bool,
    nine: bool,
}

impl GridSquare {
    fn from_sudoku_number(input: GridState) -> Self {
        Self {
            value: input,
            ..Default::default()
        }
    }
}

impl Default for GridSquare {
    fn default() -> Self {
        Self {
            value: GridState::Empty,
            one: false,
            two: false,
            three: false,
            four: false,
            five: false,
            six: false,
            seven: false,
            eight: false,
            nine: false,
        }
    }
}

/// Possible values that can be written into a sudoku grid square.
enum GridState {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Empty,
}
