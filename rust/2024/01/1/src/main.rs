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

fn process(file: impl Read) -> u32 {
    let mut locations_1 = Vec::new();
    let mut locations_2 = Vec::new();

    for line in BufReader::new(file).lines().map_while(Result::ok) {
        let mut locations = line.split_whitespace();

        let location_1: u32 = locations.next().unwrap().parse().unwrap();
        locations_1.push(location_1);

        let location_2: u32 = locations.next().unwrap().parse().unwrap();
        locations_2.push(location_2);
    }

    locations_1.sort();
    locations_2.sort();

    locations_1
        .into_iter()
        .zip(locations_2)
        .map(|(location_1, location_2)| location_1.abs_diff(location_2))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::process;
    use stringreader::StringReader;

    #[test]
    fn test_example_result() {
        let lines = StringReader::new(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );

        assert_eq!(process(lines), 11);
    }
}
