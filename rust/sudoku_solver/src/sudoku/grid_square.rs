use crate::sudoku::grid_state::GridState;

/// Defines a single square on a sudoku board.
#[allow(unused)]
#[derive(Copy, Clone)]
pub struct GridSquare {
    // The inked in value
    value: GridState,

    // Some solving patterns require knowing what values a square might contain.
    // These values are intended to serve that purpose.
    // A value of 'true' indicates that the containing square could be that value.
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

#[allow(unused)]
impl GridSquare {
    pub fn from_char(input: char) -> Self {
        Self {
            value: match input {
                '1' => GridState::One,
                '2' => GridState::Two,
                '3' => GridState::Three,
                '4' => GridState::Four,
                '5' => GridState::Five,
                '6' => GridState::Six,
                '7' => GridState::Seven,
                '8' => GridState::Eight,
                '9' => GridState::Nine,
                '?' => GridState::Empty,
                _ => panic!(),
            },
            ..Default::default()
        }
    }

    pub fn from_grid_state(input: GridState) -> Self {
        match input {
            GridState::Empty => Self {
                value: GridState::Empty,
                ..Default::default()
            },
            _ => Self {
                value: input,
                one: false,
                two: false,
                three: false,
                four: false,
                five: false,
                six: false,
                seven: false,
                eight: false,
                nine: false,
            },
        }
    }

    pub fn from_str(slice: &str) -> Self {
        Self {
            value: match slice {
                "1" => GridState::One,
                "2" => GridState::Two,
                "3" => GridState::Three,
                "4" => GridState::Four,
                "5" => GridState::Five,
                "6" => GridState::Six,
                "7" => GridState::Seven,
                "8" => GridState::Eight,
                "9" => GridState::Nine,
                "?" => GridState::Empty,
                _ => panic!(),
            },
            ..GridSquare::default()
        }
    }

    pub fn to_u8(&self) -> u8 {
        match &self.value {
            GridState::One => 1u8,
            GridState::Two => 2u8,
            GridState::Three => 3u8,
            GridState::Four => 4u8,
            GridState::Five => 5u8,
            GridState::Six => 6u8,
            GridState::Seven => 7u8,
            GridState::Eight => 8u8,
            GridState::Nine => 8u8,
            GridState::Empty => 0u8,
        }
    }

    /// Returns a &str representing the current state of Self.
    /// GridState::One => "1", etc.
    pub fn to_char(&self) -> char {
        match &self.value {
            GridState::One => '1',
            GridState::Two => '2',
            GridState::Three => '3',
            GridState::Four => '4',
            GridState::Five => '5',
            GridState::Six => '6',
            GridState::Seven => '7',
            GridState::Eight => '8',
            GridState::Nine => '9',
            GridState::Empty => '?',
        }
    }

    /// Returns a &str representing the current state of Self.
    /// GridState::One => "1", etc.
    pub fn to_str(&self) -> &str {
        match &self.value {
            GridState::One => "1",
            GridState::Two => "2",
            GridState::Three => "3",
            GridState::Four => "4",
            GridState::Five => "5",
            GridState::Six => "6",
            GridState::Seven => "7",
            GridState::Eight => "8",
            GridState::Nine => "9",
            GridState::Empty => "?",
        }
    }
}

impl Default for GridSquare {
    fn default() -> Self {
        Self {
            value: GridState::Empty,
            one: true,
            two: true,
            three: true,
            four: true,
            five: true,
            six: true,
            seven: true,
            eight: true,
            nine: true,
        }
    }
}
