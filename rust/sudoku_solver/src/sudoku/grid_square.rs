use crate::sudoku::grid_state::GridState;

/// Defines a single square on a sudoku board.
#[allow(unused)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GridSquare {
    // The inked in value
    pub value: GridState,

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
    /// Returns true iff only one notes field is true.
    pub fn has_one_possible_value(&self) -> bool {
        let mut accumulator: usize = 0;
        if self.one   { accumulator += 1; }
        if self.two   { accumulator += 1; }
        if self.three { accumulator += 1; }
        if self.four  { accumulator += 1; }
        if self.five  { accumulator += 1; }
        if self.six   { accumulator += 1; }
        if self.seven { accumulator += 1; }
        if self.eight { accumulator += 1; }
        if self.nine  { accumulator += 1; }

        match accumulator {
            1 => true,
            _ => false,
        }

    }

    pub fn only_possible_value(&self) -> GridState {
        if self.has_one_possible_value() {
            if self.one { return GridState::One; } 
            else if self.two {
                return GridState::Two;
            } else if self.three {
                return GridState::Three;
            } else if self.four {
                return GridState::Four;
            } else if self.five {
                return GridState::Five;
            } else if self.six {
                return GridState::Six;
            } else if self.seven {
                return GridState::Seven;
            } else if self.eight {
                return GridState::Eight;
            } else if self.nine {
                return GridState::Nine;
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }

    // Only constructor for GridSquares, since it handles the logic well.
    // Adding more constructor methods would increase the surface for bugs to appear.
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

    pub fn remove_note(&mut self, note: GridState) {
        match note {
            GridState::One => self.one = false,
            GridState::Two => self.two = false,
            GridState::Three => self.three = false,
            GridState::Four => self.four = false,
            GridState::Five => self.five = false,
            GridState::Six => self.six = false,
            GridState::Seven => self.seven = false,
            GridState::Eight => self.eight = false,
            GridState::Nine => self.nine = false,
            _ => (),
        }
    }

    pub fn remove_all_notes(&mut self) {
        self.one = false;
        self.two = false;
        self.three = false;
        self.four = false;
        self.five = false;
        self.six = false;
        self.seven = false;
        self.eight = false;
        self.nine = false;
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

    /// Returns a vector representing which notes are true.
    pub fn notes(&self) -> Vec<usize> {
        let mut v: Vec<usize> = Vec::with_capacity(9);
        
        if self.one   { v.push(1); }
        if self.two   { v.push(2); }
        if self.three { v.push(3); }
        if self.four  { v.push(4); }
        if self.five  { v.push(5); }
        if self.six   { v.push(6); }
        if self.seven { v.push(7); }
        if self.eight { v.push(8); }
        if self.nine  { v.push(0); }

        v
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

// Unit Tests

#[test]
fn has_one_possible_value_all_false() {
    // Test 1: No notes are true, should return false.
    let t = GridSquare {
        value: GridState::One,
        one: false,
        two: false,
        three: false,
        four: false,
        five: false,
        six: false,
        seven: false,
        eight: false,
        nine: false,
    };
    assert_eq!(t.has_one_possible_value(), false);
}

#[test]
fn has_one_possible_value_single_true() {
    // Test 1: No notes are true, should return false.
    let t = GridSquare {
        value: GridState::Empty,
        one: false,
        two: false,
        three: false,
        four: true,
        five: false,
        six: false,
        seven: false,
        eight: false,
        nine: false,
    };
    assert_eq!(t.has_one_possible_value(), true);
}

#[test]
fn has_one_possible_value_multiple_true() {
    // Test 1: No notes are true, should return false.
    let t = GridSquare {
        value: GridState::Empty,
        one: true,
        two: false,
        three: false,
        four: true,
        five: false,
        six: false,
        seven: true,
        eight: false,
        nine: false,
    };
    assert_eq!(t.has_one_possible_value(), false);
}
