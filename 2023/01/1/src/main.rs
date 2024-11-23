use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").expect("opening input file");

    let result: u32 = BufReader::new(file)
        .lines()
        .flatten()
        .map(find_number)
        .sum();

    println!("{result}");
}

fn find_number(text: String) -> u32 {
    let first_digit = find_digit_or_zero(text.chars());
    let last_digit = find_digit_or_zero(text.chars().rev());

    first_digit * 10 + last_digit
}

fn find_digit_or_zero(mut chars: impl Iterator<Item = char>) -> u32 {
    chars
        .find(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap_or_default())
        .unwrap_or_default()
}
