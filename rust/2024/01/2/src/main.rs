use std::{
    collections::HashMap,
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

fn process(file: impl Read) -> u32 {
    let mut locations_1 = Vec::new();
    let mut locations_2 = HashMap::new();

    for line in BufReader::new(file).lines().map_while(Result::ok) {
        let mut locations = line.split_whitespace();

        let location_1: u32 = locations.next().unwrap().parse().unwrap();
        locations_1.push(location_1);

        let location_2: u32 = locations.next().unwrap().parse().unwrap();
        let location_2_value = locations_2
            .get(&location_2)
            .map_or(1, |old_value| old_value + 1);
        locations_2.insert(location_2, location_2_value);
    }

    locations_1
        .into_iter()
        .map(|location_1| location_1 * locations_2.get(&location_1).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::process;
    use std::io::BufReader;

    #[test]
    fn test_example_result() {
        let lines = BufReader::new(
            "3   4
4   3
2   5
1   3
3   9
3   3"
                .as_bytes(),
        );

        assert_eq!(process(lines), 31);
    }
}
