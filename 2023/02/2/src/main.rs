use std::{
    cmp::max,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").expect("opening input file");

    let result: u32 = BufReader::new(file)
        .lines()
        .flatten()
        .map(get_power_of_game)
        .sum();

    println!("{result}");
}

fn get_power_of_game(game: String) -> u32 {
    let game = game.split(": ").nth(1).unwrap();

    let min_cubes = game
        .split("; ")
        .map(min_cubes_of_game_subset)
        .reduce(|acc, e| (max(acc.0, e.0), max(acc.1, e.1), max(acc.2, e.2)))
        .unwrap();

    min_cubes.0 * min_cubes.1 * min_cubes.2
}

fn min_cubes_of_game_subset(game_subset: &str) -> (u32, u32, u32) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for number_and_color in game_subset.split(", ") {
        let mut number_and_color = number_and_color.split(' ');
        let number: u32 = number_and_color.next().unwrap().parse().unwrap();
        let color = number_and_color.next().unwrap();

        match color {
            "red" => red = number,
            "green" => green = number,
            _ => blue = number,
        }
    }

    (red, green, blue)
}
