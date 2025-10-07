//! # Elimination by Pigeonhole Principle
//! 
//! Suppose we have a board like the following:
//! ```text
//! ??1 ?8? ??4      
//! ??2 4?? ??1      
//! ?X8 3?1 7??      
//! 
//! 127 ??? 369      
//! 986 213 547      
//! ??4 9?? 218      
//! 
//! 873 195 4??      
//! 219 ??? 875      
//! 645 ??? 19? 
//! ```     
//! 
//! Note the X at ```(row: 2, column: 1)```. This value can be inked a ```9``` due to the pigeonhole principle.
//! 
//! The key observation here is that the only squares where a ```9``` can go in box two are ```(0, 5)``` and ```(1, 5)```.
//! Similarly, the only squares that can be a ```9``` in box 3 are ```(0, 6)``` and ```(1, 6)```.
//! 
//! With this information, we can know for certain that a 9 will appear once in row 0 and row 1 and cannot be in those
//! rows in box 1. Therefore, we can eliminate 9 from the notes in ```(0, 1)``` and ```(1, 1)```, meaning a 9 can be found
//! in ```(2, 1)```.
//! 
//! The process of eliminating the notes in box 1 is referred to as Elimination by Pigeonhole Principle, the implementation of
//! which is the goal of this file.
//! 
//! 
//! # Equivalently and Far More Simply:
//! Because, as there so often is, there's a dumber way to do it.
//! 
//! Consider blocks 1, 2, and 3. If a number n cannot exist in row r for two of those blocks, it must be in that row for
//! the third. Then, in the third block, we can eliminate all notes of n not in row r.
//! 
//! 
//! 
//! 

use crate::sudoku::block::BlockNumber;
use crate::sudoku::block::*;
use crate::sudoku::coordinates::Coordinates;
use crate::sudoku::sudoku::Sudoku;
use crate::sudoku::grid_square::GridSquare;


//  fn could_you_imagine_if_this_works() {
//      // Wrapper for pigeonhole which can literally just switch the row and column variables.
// }


fn pigeonhole(board: &mut Sudoku, row_index: usize, note: usize) {
    let mut group_elements: Vec<GridSquare> = Vec::with_capacity(9);

    // This should generate the list of nine elements, assuming it's not bugged. // TODO: Write a test.
    for column_index in 0..9 {
        group_elements.push(*board.get_square(Coordinates::from(row_index, column_index)));
    }

    // Because I'm a maniac who doesn't believe in tests!!!!!
    assert_eq!(group_elements.len(), 9usize);

    // A shorthand representation of whether a subset of 3 squares per block; whether it can contain the noted value.
    let mut subset_contains_note: [bool; 3] = [false; 3];

    // Now that we have our group elements, we can properly construct the scn.
    for each in 0..group_elements.len() {
        if group_elements[each].check_note(note) {
            subset_contains_note[each / 3] = true;
        }
    }

    // Get the total number of true(s) in the scn and the index of the first true.
    let mut trues_in_scn: u8 = 0;
    let mut first_true_index: Option<usize> = None;
    for index in 0..subset_contains_note.len() {
        // If a true element is found...
        if subset_contains_note[index] { 
            trues_in_scn += 1;  // Update the counter

            // Mark the index if its the first one found.
            if first_true_index == None {
                first_true_index = Some(index);
            } 
    // The dark side is a pathway to many abilities some consider to be unnatural.
    }   }

    // Ensure the correct number of matches were found since we don't trust like that.
    // The only 'successful' execution path happens when trues_in_scn == 1.

    // 0 can occur when the note has already been inked, meaning there will be no matches and nothing to learn.
    if trues_in_scn == 0 { return; }

    // Likewise, if too many matches are found, we don't learn anything.
    if trues_in_scn == 2 || trues_in_scn == 3 {
        return;
    }  

    // Figure out which block this nonsense affects.
    let bn: BlockNumber = map_coordinates_to_block(row_index, 3 * first_true_index.unwrap());

    // Inside that block, we want to erase any notes of the target value not found in the row we are working on.
    let block_coordinate_tuples = map_block_to_array_of_coordinates(bn);
    let mut block_coordinates: Vec<Coordinates> = Vec::with_capacity(9);

    for each in block_coordinate_tuples {
        let (row, column) = each;
        block_coordinates.push( Coordinates::from(row, column));
    }

    // At this point, we have a vec of all the coordinates of the elements in the block where we demonstrated
    // the target value must be contained and we even know which row it is in, meaning we can erase the notes
    // for that value for all elements not in row r.
    let _ = block_coordinates.extract_if(.., |x| x.row_index == row_index);

    assert_eq!(block_coordinates.len(), 6);

    for each in block_coordinates {
        board.get_square(each).remove_note_by_usize(note);
    }

}







