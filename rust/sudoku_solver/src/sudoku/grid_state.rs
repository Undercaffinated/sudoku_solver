#[derive(Copy, Clone)]
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

impl Default for GridState {
    fn default() -> Self {
        GridState::Empty
    }
}
