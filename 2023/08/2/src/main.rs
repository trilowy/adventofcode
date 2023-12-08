use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

const START_NODE: char = 'A';
const END_NODE: char = 'Z';

fn main() {
    let file = File::open("./2023/08/1/input.txt").expect("opening input file");

    let mut lines = BufReader::new(file).lines().flatten();
    let instructions = lines.next().unwrap();
    let lines = lines.skip(1);

    let nodes: HashMap<String, (String, String)> = lines.map(parse_node).collect();

    let steps = nodes
        .iter()
        .filter(|(node, _)| node.ends_with(START_NODE))
        .map(|(node, _)| number_of_steps(node, &instructions, &nodes))
        .reduce(|acc, e| lcm(acc, e))
        .unwrap();

    println!("{steps}");
}

fn number_of_steps(
    start: &str,
    instructions: &str,
    nodes: &HashMap<String, (String, String)>,
) -> usize {
    let mut steps = 0;
    let mut current_node = start;

    'outer: loop {
        for instruction in instructions.chars() {
            if current_node.ends_with(END_NODE) {
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

    steps
}

fn parse_node(line: String) -> (String, (String, String)) {
    let (value, left_right) = line.split_once(" = ").unwrap();

    let (left, right) = left_right[1..left_right.len() - 1]
        .split_once(", ")
        .unwrap();

    (value.to_string(), (left.to_string(), right.to_string()))
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
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
