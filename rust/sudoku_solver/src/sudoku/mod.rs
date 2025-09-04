mod solving_logic;

pub struct Sudoku {}

struct GridSquare {
    // The inked in value
    value: Option<SudokuNumber>,

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

enum SudokuNumber {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}
