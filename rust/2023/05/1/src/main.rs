use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").expect("opening input file");

    let mut lines = BufReader::new(file).lines();
    let seeds = parse_seeds(&lines.next().unwrap().unwrap());

    let mut maps: Vec<HashMap<(i64, i64), i64>> = vec![HashMap::new()];
    let lines = lines.skip(2);

    for line in lines.flatten() {
        if line.is_empty() {
            maps.push(HashMap::new());
        } else if line.chars().next().unwrap().is_ascii_digit() {
            let (key, value) = parse_line(line);
            maps.last_mut().unwrap().insert(key, value);
        }
    }

    let result = seeds
        .into_iter()
        .map(|seed| {
            maps.iter()
                .fold(seed, |seed, map| map.get_seed_mapping(seed))
        })
        .min()
        .unwrap();

    println!("{result}");
}

trait SeedMap {
    fn get_seed_mapping(&self, seed: i64) -> i64;
}

impl SeedMap for HashMap<(i64, i64), i64> {
    fn get_seed_mapping(&self, seed: i64) -> i64 {
        for ((destination_range_start, source_range_start), &range_length) in self {
            let shift = destination_range_start - source_range_start;

            if seed >= *source_range_start && seed < source_range_start + range_length {
                return seed + shift;
            }
        }
        seed
    }
}

fn parse_seeds(text: &str) -> Vec<i64> {
    text.split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect()
}

fn parse_line(text: String) -> ((i64, i64), i64) {
    let entries: Vec<i64> = text
        .split_whitespace()
        .map(|entry| entry.parse().unwrap())
        .collect();

    ((entries[0], entries[1]), entries[2])
}

#[cfg(test)]
mod tests {
    use crate::SeedMap;
    use std::collections::HashMap;

    #[test]
    fn get_seed_mapping() {
        let mut map = HashMap::new();
        map.insert((52, 50), 48);

        assert_eq!(map.get_seed_mapping(79), 81);
    }
}
