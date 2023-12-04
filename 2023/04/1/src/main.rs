use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./2023/04/1/input.txt").expect("opening input file");

    let result: u32 = BufReader::new(file)
        .lines()
        .flatten()
        .map(get_game_points)
        .sum();

    println!("{result}");
}

fn get_game_points(game: String) -> u32 {
    let game: Vec<&str> = game.split(": ").nth(1).unwrap().split(" | ").collect();

    let winning_numbers: HashSet<&str> = game[0].split_whitespace().collect();
    let numbers: HashSet<&str> = game[1].split_whitespace().collect();
    let number_of_matches = winning_numbers.intersection(&numbers).count() as u32;

    if number_of_matches == 0 {
        0
    } else {
        2_u32.pow(number_of_matches - 1)
    }
}
