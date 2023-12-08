use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

const START_NODE: &str = "AAA";
const END_NODE: &str = "ZZZ";

fn main() {
    let file = File::open("./2023/08/1/input.txt").expect("opening input file");

    let mut lines = BufReader::new(file).lines().flatten();
    let instructions = lines.next().unwrap();
    let lines = lines.skip(1);

    let nodes: HashMap<String, (String, String)> = lines.map(parse_node).collect();

    let mut steps = 0;
    let mut current_node = START_NODE;

    'outer: loop {
        for instruction in instructions.chars() {
            if current_node == END_NODE {
                break 'outer;
            }
            if instruction == 'L' {
                current_node = &nodes.get(current_node).unwrap().0;
            } else {
                current_node = &nodes.get(current_node).unwrap().1;
            }
            steps += 1;
        }
    }

    println!("{steps}");
}

fn parse_node(line: String) -> (String, (String, String)) {
    let (value, left_right) = line.split_once(" = ").unwrap();

    let (left, right) = left_right[1..left_right.len() - 1]
        .split_once(", ")
        .unwrap();

    (value.to_string(), (left.to_string(), right.to_string()))
}

#[cfg(test)]
mod tests {
    use crate::parse_node;

    #[test]
    fn test_parse_node() {
        assert_eq!(
            parse_node("BBB = (AAA, ZZZ)".to_string()),
            ("BBB".to_string(), ("AAA".to_string(), "ZZZ".to_string()))
        );
    }
}
