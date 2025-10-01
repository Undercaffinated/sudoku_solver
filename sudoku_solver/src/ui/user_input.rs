use std::io;

pub fn get_usize() -> usize {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse().unwrap()
}


pub fn get_string() -> String {
    let mut s: String = String::new();
    io::stdin().read_line(&mut s).unwrap();

    s.trim().replace("\n", "")
}