use std::env;
use std::fs;

#[allow(unused)]
pub fn load_file() -> Option<String> {
    let extraction_result = extract_filename();

    let maybe_filename: Option<String> = match extraction_result {
        Ok(None) => None,
        Ok(Some(content)) => Some(content),
        Err(_) => panic!(),
    };

    if maybe_filename == None {
        return None;
    }

    let filename = maybe_filename.unwrap();

    let content: String = fs::read_to_string(filename).unwrap().replace("\n", "");

    // We know that valid files will always contain 81 characters after trimming,
    // one for each grid square.
    match content.len() {
        81 => return Some(content),
        _ => panic!("File not correct length"),
    }
}

/// Reads filename from supplied CLI arguments.
fn extract_filename() -> Result<Option<String>, ArgumentError> {
    let args: Vec<String> = env::args().collect();

    let number_of_arguments: i32 = (args.len() - 1) as i32;

    match number_of_arguments {
        0 => Ok(None),
        1 => Ok(Some(args[1].to_string())),
        _ => Err(ArgumentError::TooManyArguments),
    }
}

#[derive(Debug)]
enum ArgumentError {
    TooManyArguments,
    // NoArgumentsFound,
}
