use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    time::Instant,
};

fn main() {
    let start = Instant::now();

    let file = File::open("input.txt").expect("opening input file");
    println!("{}", process(file));

    println!("{:?}", start.elapsed());
}

fn process(file: impl Read) -> usize {
    BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .map(process_report)
        .filter(|result| *result)
        .count()
}

fn process_report(line: String) -> bool {
    let mut levels = line.split_whitespace();

    let mut previous_level: u32 = levels.next().unwrap().parse().unwrap();
    let mut level_type = None;

    for level in levels {
        let level: u32 = level.parse().unwrap();

        match level_type {
            None => {
                if is_increasing(previous_level, level) {
                    level_type = Some(LevelType::Increase);
                } else if is_decreasing(previous_level, level) {
                    level_type = Some(LevelType::Decrease);
                } else {
                    return false;
                }
            }
            Some(LevelType::Increase) => {
                if !is_increasing(previous_level, level) {
                    return false;
                }
            }
            Some(LevelType::Decrease) => {
                if !is_decreasing(previous_level, level) {
                    return false;
                }
            }
        }

        previous_level = level;
    }

    true
}

const MAX_SLOPE: u32 = 3;

fn is_decreasing(previous_level: u32, level: u32) -> bool {
    previous_level > level && previous_level - level <= MAX_SLOPE
}

fn is_increasing(previous_level: u32, level: u32) -> bool {
    previous_level < level && level - previous_level <= MAX_SLOPE
}

enum LevelType {
    Increase,
    Decrease,
}

#[cfg(test)]
mod tests {
    use crate::{process, process_report};
    use std::io::BufReader;

    #[test]
    fn test_example_reports() {
        assert!(process_report("7 6 4 2 1".to_string()));
        assert!(!process_report("1 2 7 8 9".to_string()));
        assert!(!process_report("9 7 6 2 1".to_string()));
        assert!(!process_report("1 3 2 4 5".to_string()));
        assert!(!process_report("8 6 4 4 1".to_string()));
        assert!(process_report("1 3 6 7 9".to_string()));
    }

    #[test]
    fn test_example_result() {
        let lines = BufReader::new(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
                .as_bytes(),
        );

        assert_eq!(process(lines), 2);
    }
}
