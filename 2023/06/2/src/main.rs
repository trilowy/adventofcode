use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./2023/06/1/input.txt").expect("opening input file");

    let mut lines = BufReader::new(file).lines().flatten();
    let times = lines.next().unwrap();
    let time = parse_line(&times);
    let distances = lines.next().unwrap();
    let distance = parse_line(&distances);

    let result = calculate_win_possibilities(time, distance);

    println!("{result}");
}

fn calculate_win_possibilities(time: u64, distance: u64) -> usize {
    (1..time)
        .map(|current_time| (time - current_time) * current_time)
        .filter(|distance_result| *distance_result > distance)
        .count()
}

fn parse_line(text: &str) -> u64 {
    text.split_once(':')
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .fold(String::new(), |acc, e| acc + e)
        .parse()
        .unwrap()
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
