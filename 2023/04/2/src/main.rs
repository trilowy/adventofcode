use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").expect("opening input file");

    let games_and_points: BTreeMap<usize, usize> = BufReader::new(file)
        .lines()
        .flatten()
        .map(get_game_id_and_points)
        .collect();

    let mut games_and_copies: HashMap<usize, usize> = games_and_points
        .keys()
        .map(|key| (key.clone(), 1))
        .collect();

    for (game_id, points) in games_and_points {
        let number_of_copies = games_and_copies.get(&game_id).unwrap().clone();
        for winning_game_id in (game_id + 1)..=(game_id + points) {
            if let Some(existing_copies) = games_and_copies.get(&winning_game_id) {
                games_and_copies.insert(winning_game_id, existing_copies + number_of_copies);
            }
        }
    }

    let result: usize = games_and_copies.values().sum();

    println!("{result}");
}

fn get_game_id_and_points(game: String) -> (usize, usize) {
    let game: Vec<&str> = game.split(": ").collect();
    let game_id = game[0].split_whitespace().nth(1).unwrap().parse().unwrap();

    let game: Vec<&str> = game[1].split(" | ").collect();

    let winning_numbers: HashSet<&str> = game[0].split_whitespace().collect();
    let numbers: HashSet<&str> = game[1].split_whitespace().collect();
    let number_of_matches = winning_numbers.intersection(&numbers).count();

    (game_id, number_of_matches)
}
