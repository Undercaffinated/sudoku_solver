//! # What is a Block?
//! 
//! Blocks are 3x3 subsets of a sudoku board which are guaranteed to contain 1 instance each
//! of every number from 1 to 9. There are 9 distinct blocks numbered 1 to 9 which are laid out
//! as shown below.
//! 
//!  1 | 2 | 3    
//! ---+---+---     
//!  4 | 5 | 6     
//! ---+---+---    
//!  7 | 8 | 9     
//! 
//! Note: Blocks, and by extension ```BlockNumbers``` are not zero indexed for a handful of reasons.
//! - Blocks are not intended to be iterated over.
//! - There is no proper Block struct. Blocks are only used as conceptual tools.


/// List of postitions a given Block can be in.
#[derive(Debug, PartialEq)]
pub enum BlockNumber {
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

impl BlockNumber {
    /// Returns a supplied ```i32``` as a BlockNumber without modifying the input.
    /// 
    /// # Example
    /// 
    /// ```
    /// assert_eq!(BlockNumber::from_i32(1), BlockNumber::One);
    /// ```
    /// 
    pub fn from_i32 (input: i32) -> Self {
        match input {
            1 => BlockNumber::One,
            2 => BlockNumber::Two,
            3 => BlockNumber::Three,
            4 => BlockNumber::Four,
            5 => BlockNumber::Five,
            6 => BlockNumber::Six,
            7 => BlockNumber::Seven,
            8 => BlockNumber::Eight,
            9 => BlockNumber::Nine,
            0 => panic!("Was given a value of 0!"),
            _ => panic!("Was given a value greater than 9!"),
        }
    }
}


/// Given a square with row index r and column index c, returns correlating ```BlockNumber```.
pub fn map_coordinates_to_block(row_index: usize, column_index: usize) -> BlockNumber {
    match (row_index, column_index) {
        (0..3, 0..3) => BlockNumber::One,
        (0..3, 3..6) => BlockNumber::Two,
        (0..3, 6..9) => BlockNumber::Three,

        (3..6, 0..3) => BlockNumber::Four,
        (3..6, 3..6) => BlockNumber::Five,
        (3..6, 6..9) => BlockNumber::Six,

        (6..9, 0..3) => BlockNumber::Seven,
        (6..9, 3..6) => BlockNumber::Eight,
        (6..9, 6..9) => BlockNumber::Nine,

        (_, _) => panic!(),
    }
}

/// Returns an array of coordinates associated with the input ```BlockNumber```.
pub fn map_block_to_array_of_coordinates(block: BlockNumber) -> [(usize, usize); 9] {
    match block {
        // BlockNumber -> [(row_index, column_index); 9]
        // Top left
        BlockNumber::One => [
            (0, 0), (0, 1), (0, 2),
            (1, 0), (1, 1), (1, 2),
            (2, 0), (2, 1), (2, 2),
        ],
        // Top center
        BlockNumber::Two => [
            (0, 3), (0, 4), (0, 5),
            (1, 3), (1, 4), (1, 5),
            (2, 3), (2, 4), (2, 5),
        ],
        // Top right
        BlockNumber::Three => [
            (0, 6), (0, 7), (0, 8),
            (1, 6), (1, 7), (1, 8),
            (2, 6), (2, 7), (2, 8),
        ],
        // Middle left
        BlockNumber::Four => [
            (3, 0), (3, 1), (3, 2),
            (4, 0), (4, 1), (4, 2),
            (5, 0), (5, 1), (5, 2),
        ],
        // Center
        BlockNumber::Five => [
            (3, 3), (3, 4), (3, 5),
            (4, 3), (4, 4), (4, 5),
            (5, 3), (5, 4), (5, 5),
        ],
        // Middle right
        BlockNumber::Six => [
            (3, 6), (3, 7), (3, 8),
            (4, 6), (4, 7), (4, 8),
            (5, 6), (5, 7), (5, 8),
        ],
        // Bottom left
        BlockNumber::Seven => [
            (6, 0), (6, 1), (6, 2),
            (7, 0), (7, 1), (7, 2),
            (8, 0), (8, 1), (8, 2),
        ],
        // Bottom center
        BlockNumber::Eight => [
            (6, 3), (6, 4), (6, 5),
            (7, 3), (7, 4), (7, 5),
            (8, 3), (8, 4), (8, 5),
        ],
        // Bottom right
        BlockNumber::Nine => [
            (6, 6), (6, 7), (6, 8),
            (7, 6), (7, 7), (7, 8),
            (8, 6), (8, 7), (8, 8),
        ],
    }
}
