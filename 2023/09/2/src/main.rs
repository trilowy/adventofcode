use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn main() {
    let file = File::open("input.txt").expect("opening input file");
    println!("{}", process(file));
}

fn process(file: impl Read) -> i64 {
    BufReader::new(file)
        .lines()
        .flatten()
        .map(find_next_value)
        .sum()
}

fn find_next_value(line: String) -> i64 {
    let mut all_zeros = true;

    let mut values: Vec<i64> = line
        .split_whitespace()
        .map(|value| {
            let value = value.parse().unwrap();
            if value != 0 {
                all_zeros = false;
            }
            value
        })
        .collect();

    let mut first_values = Vec::new();

    while !all_zeros {
        first_values.push(*values.first().unwrap());

        all_zeros = true;

        values = values
            .windows(2)
            .map(|tuple| {
                let value = tuple[1] - tuple[0];
                if value != 0 {
                    all_zeros = false;
                }
                value
            })
            .collect();
    }

    first_values
        .into_iter()
        .rev()
        .reduce(|acc, e| e - acc)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use crate::process;
    use stringreader::StringReader;

    #[test]
    fn test_example_result() {
        let lines = StringReader::new("10 13 16 21 30 45");

        assert_eq!(process(lines), 5);
    }
}
