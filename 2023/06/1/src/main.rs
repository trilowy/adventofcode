use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").expect("opening input file");

    let mut lines = BufReader::new(file).lines().flatten();
    let times = lines.next().unwrap();
    let times = parse_line(&times);
    let distances = lines.next().unwrap();
    let distances = parse_line(&distances);
    let time_and_distance = times.zip(distances);

    let result = time_and_distance
        .map(|time_and_distance| {
            calculate_win_possibilities(time_and_distance.0, time_and_distance.1)
        })
        .reduce(|acc, e| acc * e)
        .unwrap_or_default();

    println!("{result}");
}

fn calculate_win_possibilities(time: u32, distance: u32) -> usize {
    (1..time)
        .map(|current_time| (time - current_time) * current_time)
        .filter(|distance_result| *distance_result > distance)
        .count()
}

fn parse_line(text: &str) -> impl Iterator<Item = u32> + '_ {
    text.split_once(':')
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|text| text.parse::<u32>().unwrap())
}

#[cfg(test)]
mod tests {
    use crate::calculate_win_possibilities;

    #[test]
    fn test_calculate_win_possibilities_1() {
        assert_eq!(calculate_win_possibilities(7, 9), 4);
    }

    #[test]
    fn test_calculate_win_possibilities_2() {
        assert_eq!(calculate_win_possibilities(15, 40), 8);
    }

    #[test]
    fn test_calculate_win_possibilities_3() {
        assert_eq!(calculate_win_possibilities(30, 200), 9);
    }
}
