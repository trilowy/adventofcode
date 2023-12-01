use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./2023/01/1/input.txt").expect("opening input file");
    let regex = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|[0-9]").unwrap();
    let regex_rev = Regex::new(r"^.*(one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();

    let result: u32 = BufReader::new(file)
        .lines()
        .flatten()
        .map(|line| find_number(&regex, &regex_rev, line))
        .sum();

    println!("{result}");
}

fn find_number(regex: &Regex, regex_rev: &Regex, text: String) -> u32 {
    let first = regex.find(&text).unwrap().as_str();
    let last = &regex_rev.captures(&text).unwrap()[1];

    let first_digit = convert_to_digit(first);
    let last_digit = convert_to_digit(last);

    first_digit * 10 + last_digit
}

fn convert_to_digit(text: &str) -> u32 {
    match text {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => text.parse().unwrap(),
    }
}
