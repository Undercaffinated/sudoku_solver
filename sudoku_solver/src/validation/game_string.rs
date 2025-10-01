// Module for verifying whether a given string is a valid representation of a Sudoku board.


pub fn verify_game_string(unverified: String) -> Result<String, GameStringVerificationError> {
    if has_incorrect_length(&unverified) { return Err(GameStringVerificationError::IncorrectLength); }

    else {
        return Ok(unverified);
    }
}


fn has_incorrect_length(unverified: &String) -> bool {
    unverified.len() != 81
}


pub enum GameStringVerificationError {
    IncorrectLength,
}