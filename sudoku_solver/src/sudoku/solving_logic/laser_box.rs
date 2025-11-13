//! # The Laser Box Method
//! 
//! If note n appears in at least two squares within box b and are contained by a group which is either a
//! row or a column g, then for all n in g not in b, we can remove n.
//! 
//! Example: Suppose note '4' appears twice in the first row of box 1 and nowhere else in that box. This eliminates
//! the possiblity of there be an inked four in boxes 2 or 3 in row 0.
//! 

use std::collections::HashMap;

use crate::sudoku::sudoku::Sudoku;
use crate::sudoku::block::{BlockNumber, map_block_to_array_of_coordinates};


pub fn laser_box(board: &mut Sudoku) -> bool {
    
    for note in 1..= 9usize {
        for block in 1..= 9 {
            let maybe_lb_variant = _identify_lb(board, BlockNumber::from_i32(block), note);

            if maybe_lb_variant == None { continue; }

            let lb_variant = maybe_lb_variant.unwrap();

            // Remove the appropriate notes.

        }
    }



    board.grid == board.previous_grid

}






fn _identify_lb(board: &mut Sudoku, block: BlockNumber, note: usize) -> Option<LaserBoxVariant> {

    assert!(note < 10 && note > 0);

    let squares = map_block_to_array_of_coordinates(block);


    let mut intermediate: [bool; 9] = [false; 9];

    for index in 0..9 {
        let (row, column) = squares[index];

        intermediate[index] = board.grid[row][column].check_note(note);
    }


    let mut representative_array = [false; 6];
    representative_array[0] = make_rep_array_element(intermediate, LaserBoxVariant::Row0);
    representative_array[1] = make_rep_array_element(intermediate, LaserBoxVariant::Row1);
    representative_array[2] = make_rep_array_element(intermediate, LaserBoxVariant::Row2);
    representative_array[3] = make_rep_array_element(intermediate, LaserBoxVariant::Col0);
    representative_array[4] = make_rep_array_element(intermediate, LaserBoxVariant::Col1);
    representative_array[5] = make_rep_array_element(intermediate, LaserBoxVariant::Col2);


    let mut h = HashMap::new();


    for index in 0..representative_array.len() {
        let hash_insert_result = h.insert(representative_array[index], index);

        match (representative_array[index], hash_insert_result) {
            (true, None) => continue,
            (false, _) => continue,
            (true, Some(_)) => return None,
        }
    }

    Some(LaserBoxVariant::from_usize(h[&true]))

}




fn make_rep_array_element(r: [bool; 9], variant: LaserBoxVariant) -> bool {
    match variant {
        LaserBoxVariant::Row0 => (r[0] && r[1]) || (r[0] && r[2]) || (r[1] && r[2]),
        LaserBoxVariant::Row1 => (r[3] && r[4]) || (r[3] && r[5]) || (r[4] && r[5]),
        LaserBoxVariant::Row2 => (r[6] && r[7]) || (r[6] && r[8]) || (r[7] && r[8]),

        LaserBoxVariant::Col0 => (r[0] && r[3]) || (r[0] && r[6]) || (r[3] && r[6]),
        LaserBoxVariant::Col1 => (r[1] && r[4]) || (r[1] && r[7]) || (r[4] && r[7]),
        LaserBoxVariant::Col2 => (r[2] && r[5]) || (r[2] && r[8]) || (r[5] && r[8]),
    }
}


#[derive(PartialEq)]
enum LaserBoxVariant {
    Row0,
    Row1,
    Row2,
    Col0,
    Col1,
    Col2
}

impl LaserBoxVariant {
    fn from_usize(u: usize) -> Self {
        match u {
            0 => Self::Row0,
            1 => Self::Row1,
            2 => Self::Row2,
            3 => Self::Col0,
            4 => Self::Col1,
            5 => Self::Col2
        }
    }
}


