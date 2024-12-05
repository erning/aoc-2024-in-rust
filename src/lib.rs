use std::fs;

pub mod day01;

pub fn read_as_string(day: u8, filename: &str) -> String {
    let filename = format!("inputs/{:02}-{}.txt", day, filename);
    fs::read_to_string(filename).unwrap()
}

pub fn read_input(day: u8) -> String {
    read_as_string(day, "input")
}
pub fn read_example(day: u8) -> String {
    read_as_string(day, "example")
}
