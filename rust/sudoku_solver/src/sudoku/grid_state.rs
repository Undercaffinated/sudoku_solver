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

impl Default for GridState {
    fn default() -> Self {
        GridState::Empty
    }
}
