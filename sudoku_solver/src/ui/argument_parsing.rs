use std::env;
use std::fs;



/// Returns the contents of a file specified as the first argument provided by the user
/// at CLI call only if the specified file exists and its contents pass validation.
pub fn load_from_arge() -> String {
    let extract = extract_filename_from_env_arguments().unwrap();
    load_board_from_file(extract)
}






#[derive(Debug)]
enum ArgumentError {
    NoArgumentsFound,
    TooManyArguments,
}


/// Returns filename: Ok(String) iff one argument was supplied to CLI call.
/// Else returns an error: Err(ArgumentError).
fn extract_filename_from_env_arguments() -> Result<String, ArgumentError> {
    // Gethers all provided environment variables. That said, we are only interested in args[1], by design.
    let args: Vec<String> = env::args().collect();

    // Recall the first argument is the execution path, so we want to ignore it.
    let number_of_arguments = args.len() - 1;

    match number_of_arguments {
        0 => Err(ArgumentError::NoArgumentsFound),
        1 => Ok(args[1].to_string()),
        _ => Err(ArgumentError::TooManyArguments),
    }
}



/// Given a filename, returns the contents of that file as a string iff those
/// contents pass validation.
fn load_board_from_file(filename: String) -> String {
    let contents: String = fs::read_to_string(filename).unwrap();

    // Remove any \n chars
    let contents = contents.replace("\n", "");

    // TODO - Panics if a disallowed character is found.
    
    // Panics iff contents.len() != 81.
    match contents.len() {
        0..81 => panic!("File contains less than 81 characters."),
        81 => (),
        _ => panic!("File contains more than 81 characters."),
    }

    contents
}




