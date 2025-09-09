mod solving_logic;

pub struct Sudoku {
    // Index 0 is the upper-left corner square.
    // Index 8 is the upper-right corner.
    grid: [GridSquare; 81],
}

impl Sudoku {
    fn from_string(input: String) -> Self {
        let input = input.trim();
        let temp_grid: [GridSquare; 81] = [GridSquare::default(); 81];

        Sudoku::default()
    }

    fn print(&self) {
        // I'm sorry in advance.
        let mut print_object: String = String::with_capacity(132);

        // Line 1
        print_object.push_str(self.grid[0].to_str());
        print_object.push_str(self.grid[1].to_str());
        print_object.push_str(self.grid[2].to_str());
        print_object.push_str("|");

        print_object.push_str(self.grid[3].to_str());
        print_object.push_str(self.grid[4].to_str());
        print_object.push_str(self.grid[5].to_str());
        print_object.push_str("|");

        print_object.push_str(self.grid[6].to_str());
        print_object.push_str(self.grid[7].to_str());
        print_object.push_str(self.grid[8].to_str());
        print_object.push_str("\n");

        // Line 2
        print_object.push_str(self.grid[9].to_str());
        print_object.push_str(self.grid[10].to_str());
        print_object.push_str(self.grid[11].to_str());
        print_object.push_str("|");

        print_object.push_str(self.grid[12].to_str());
        print_object.push_str(self.grid[13].to_str());
        print_object.push_str(self.grid[14].to_str());
        print_object.push_str("|");

        print_object.push_str(self.grid[15].to_str());
        print_object.push_str(self.grid[16].to_str());
        print_object.push_str(self.grid[17].to_str());
        print_object.push_str("\n");

        // Line 3
        print_object.push_str(self.grid[18].to_str());
        print_object.push_str(self.grid[19].to_str());
        print_object.push_str(self.grid[20].to_str());
        print_object.push_str("|");

        print_object.push_str(self.grid[21].to_str());
        print_object.push_str(self.grid[22].to_str());
        print_object.push_str(self.grid[23].to_str());
        print_object.push_str("|");

        print_object.push_str(self.grid[24].to_str());
        print_object.push_str(self.grid[25].to_str());
        print_object.push_str(self.grid[26].to_str());
        print_object.push_str("\n");

        // Line 4
        print_object.push_str("---+---+---\n");

        // Line 5
        print_object.push_str(self.grid[27].to_str());
        print_object.push_str(self.grid[28].to_str());
        print_object.push_str(self.grid[29].to_str());
        print_object.push_str("|");

        print_object.push_str(self.grid[30].to_str());
        print_object.push_str(self.grid[31].to_str());
        print_object.push_str(self.grid[32].to_str());
        print_object.push_str("|");

        print_object.push_str(self.grid[33].to_str());
        print_object.push_str(self.grid[34].to_str());
        print_object.push_str(self.grid[35].to_str());
        print_object.push_str("\n");

        // Line 6
        print_object.push_str(self.grid[36].to_str());
        print_object.push_str(self.grid[37].to_str());
        print_object.push_str(self.grid[38].to_str());
        print_object.push_str("|");

        print_object.push_str(self.grid[39].to_str());
        print_object.push_str(self.grid[40].to_str());
        print_object.push_str(self.grid[41].to_str());
        print_object.push_str("|");

        print_object.push_str(self.grid[42].to_str());
        print_object.push_str(self.grid[43].to_str());
        print_object.push_str(self.grid[44].to_str());
        print_object.push_str("\n");

        // Line 7
        print_object.push_str(self.grid[45].to_str());
        print_object.push_str(self.grid[46].to_str());
        print_object.push_str(self.grid[47].to_str());
        print_object.push_str("|");

        print_object.push_str(self.grid[48].to_str());
        print_object.push_str(self.grid[49].to_str());
        print_object.push_str(self.grid[50].to_str());
        print_object.push_str("|");

        print_object.push_str(self.grid[51].to_str());
        print_object.push_str(self.grid[52].to_str());
        print_object.push_str(self.grid[53].to_str());
        print_object.push_str("\n");

        // Line 8
        print_object.push_str("---+---+---\n");

        // Line 9
        print_object.push_str(self.grid[54].to_str());
        print_object.push_str(self.grid[55].to_str());
        print_object.push_str(self.grid[56].to_str());
        print_object.push_str("|");

        print_object.push_str(self.grid[57].to_str());
        print_object.push_str(self.grid[58].to_str());
        print_object.push_str(self.grid[59].to_str());
        print_object.push_str("|");

        print_object.push_str(self.grid[60].to_str());
        print_object.push_str(self.grid[61].to_str());
        print_object.push_str(self.grid[62].to_str());
        print_object.push_str("\n");

        // Line 10
        print_object.push_str(self.grid[63].to_str());
        print_object.push_str(self.grid[64].to_str());
        print_object.push_str(self.grid[65].to_str());
        print_object.push_str("|");

        print_object.push_str(self.grid[66].to_str());
        print_object.push_str(self.grid[67].to_str());
        print_object.push_str(self.grid[68].to_str());
        print_object.push_str("|");

        print_object.push_str(self.grid[69].to_str());
        print_object.push_str(self.grid[70].to_str());
        print_object.push_str(self.grid[71].to_str());
        print_object.push_str("\n");

        // Line 10
        print_object.push_str(self.grid[72].to_str());
        print_object.push_str(self.grid[73].to_str());
        print_object.push_str(self.grid[74].to_str());
        print_object.push_str("|");

        print_object.push_str(self.grid[75].to_str());
        print_object.push_str(self.grid[76].to_str());
        print_object.push_str(self.grid[77].to_str());
        print_object.push_str("|");

        print_object.push_str(self.grid[78].to_str());
        print_object.push_str(self.grid[79].to_str());
        print_object.push_str(self.grid[80].to_str());
        print_object.push_str("\n");

        // Print the final object
        println!("{}", print_object);
    }
}

impl Default for Sudoku {
    fn default() -> Self {
        Self {
            grid: [GridSquare::default(); 81],
        }
    }
}

#[derive(Copy, Clone)]
struct GridSquare {
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

impl GridSquare {
    fn from_grid_state(input: GridState) -> Self {
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

    fn from_str(slice: &str) -> Self {
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

    fn to_u8(&self) -> u8 {
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
    fn to_str(&self) -> &str {
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

#[derive(Copy, Clone)]
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

impl Default for GridState {
    fn default() -> Self {
        GridState::Empty
    }
}
