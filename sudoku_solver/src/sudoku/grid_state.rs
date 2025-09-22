#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
/// Possible values that can be written into a sudoku grid square.
pub enum GridState {
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

// Constructor-style functions
impl GridState {
    pub fn from_char(character: char) -> Self {
        match character {
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
        }
    }

    
}

impl GridState {
    pub fn to_char(&self) -> char {
        match self {
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
}

impl Default for GridState {
    fn default() -> Self {
        GridState::Empty
    }
}


#[allow(unused)]
impl GridState {
    /// Example ```from_u8(1u8) -> GridState::One```
    pub fn from_u8(number: u8) -> Self {
        match number {
            1 => GridState::One,
            2 => GridState::Two,
            3 => GridState::Three,
            4 => GridState::Four,
            5 => GridState::Five,
            6 => GridState::Six,
            7 => GridState::Seven,
            8 => GridState::Eight,
            9 => GridState::Nine,
            _ => panic!(),
        }
    }
}
