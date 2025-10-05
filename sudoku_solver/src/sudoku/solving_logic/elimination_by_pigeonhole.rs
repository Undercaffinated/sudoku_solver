//! Elimination by Pigeonhole Principle
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
//! # How to Implement
//! 
//! ## Observations
//! 1. The only two positions for a ```9``` in ```Box 2``` are ```(0, 5)``` and ```(1, 5)```.
//! 2. The only two positions for a ```9``` in ```Box 3``` are ```(0, 6)``` and ```(1, 6)```.
//! 3. If ∃ ```9``` is in row 0 of box 2 -> ```9```
//! 
//! 
//! 
//! 