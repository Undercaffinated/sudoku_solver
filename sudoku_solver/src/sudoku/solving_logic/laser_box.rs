//! # The Laser Box Method
//! 
//! Suppose box one looks something like this and we are interested in removing '4' notes from boxes 2 and 3.
//! We don't necessarily know where the '4' is in box one, but we do know that for all valid arrangements,
//! the 4 must be in row[1]. Therefore, we can remove all '4' notes from row 1 in boxes 2 and 3.
//! 
//! 
//! ```text
//! 123
//! ???
//! 789
//! ```
//! 
//! In general, when all notes of a given number within a block happen to be located in the same row or the same column,
//! we can eliminate those notes from the same row or column in the other blocks that share that row or column.



