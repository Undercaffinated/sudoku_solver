use crate::sudoku::grid_square::GridSquare;

#[allow(unused)]
pub struct Sudoku {
    // Index 0 is the upper-left corner square.
    // Index 8 is the upper-right corner.
    grid: [GridSquare; 81],
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
        let input = input.trim().chars();
        let mut temp_grid: [GridSquare; 81] = [GridSquare::default(); 81];

        let mut incrementor: usize = 0;

        for each in input {
            temp_grid[incrementor] = GridSquare::from_char(each);
            incrementor += 1;
        }

        Self { grid: temp_grid }
    }

    pub fn print(&self) {
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

        // Line 11
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
