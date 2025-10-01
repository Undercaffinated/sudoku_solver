use std::io;

pub fn get_usize() -> usize {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse().unwrap()
}