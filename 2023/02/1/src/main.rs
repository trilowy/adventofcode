use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const GAME_PREFIX_SIZE: usize = "Game ".len();

fn main() {
    let file = File::open("./2023/02/1/input.txt").expect("opening input file");

    let result: u32 = BufReader::new(file)
        .lines()
        .flatten()
        .map(get_valid_game_number)
        .sum();

    println!("{result}");
}

fn get_valid_game_number(game: String) -> u32 {
    let game: Vec<&str> = game[GAME_PREFIX_SIZE..].split(": ").collect();

    for game_subset in game[1].split("; ") {
        if is_wrong_game_subset(game_subset) {
            return 0;
        }
    }

    game[0].parse().unwrap()
}

fn is_wrong_game_subset(game_subset: &str) -> bool {
    game_subset.split(", ").any(|number_and_color| {
        let mut number_and_color = number_and_color.split(' ');
        let number: u32 = number_and_color.next().unwrap().parse().unwrap();
        let color = number_and_color.next().unwrap();

        match color {
            "red" => number > 12,
            "green" => number > 13,
            _ => number > 14,
        }
    })
}
