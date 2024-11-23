use petgraph::{algo::dijkstra, graphmap::UnGraphMap};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
    time::Instant,
};

const START: char = 'S';
const GROUND: char = '.';

fn main() {
    let start = Instant::now();

    let file = File::open("input.txt").expect("opening input file");
    println!("{}", process(file));

    println!("{:?}", start.elapsed());
}

fn process(file: impl Read) -> i32 {
    let mut start = NodeId { x: 0, y: 0 };

    let mut graph = UnGraphMap::new();
    let mut edges = HashMap::new();

    for (y, line) in BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .enumerate()
    {
        for (x, c) in line.chars().enumerate() {
            if c != GROUND {
                let pipe = Pipe::from(c);

                if c == START {
                    start.x = x;
                    start.y = y;
                }

                let id = NodeId { x, y };

                edges.insert(id, pipe);
                graph.add_node(id);
            }
        }
    }

    for (id, pipe) in edges.iter() {
        if pipe.right {
            let right_node = NodeId {
                x: id.x + 1,
                y: id.y,
            };
            if let Some(right_node) = graph.nodes().find(|node| node == &right_node) {
                if edges.get(&right_node).unwrap().left {
                    graph.add_edge(*id, right_node, 1);
                }
            }
        }

        if pipe.down {
            let down_node = NodeId {
                x: id.x,
                y: id.y + 1,
            };
            if let Some(down_node) = graph.nodes().find(|node| node == &down_node) {
                if edges.get(&down_node).unwrap().up {
                    graph.add_edge(*id, down_node, 1);
                }
            }
        }
    }

    let shortest_paths = dijkstra(&graph, start, None, |_| 1);

    *shortest_paths.values().max().unwrap()
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
struct NodeId {
    x: usize,
    y: usize,
}

#[derive(Default)]
struct Pipe {
    up: bool,
    right: bool,
    down: bool,
    left: bool,
}

impl Pipe {
    fn from(c: char) -> Self {
        match c {
            '|' => Self {
                up: true,
                down: true,
                ..Default::default()
            },
            '-' => Self {
                left: true,
                right: true,
                ..Default::default()
            },
            'L' => Self {
                up: true,
                right: true,
                ..Default::default()
            },
            'J' => Self {
                up: true,
                left: true,
                ..Default::default()
            },
            '7' => Self {
                down: true,
                left: true,
                ..Default::default()
            },
            'F' => Self {
                down: true,
                right: true,
                ..Default::default()
            },
            GROUND => Self::default(),
            _ => Self {
                up: true,
                right: true,
                down: true,
                left: true,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::process;
    use stringreader::StringReader;

    #[test]
    fn test_example_result_1() {
        let lines = StringReader::new(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );

        assert_eq!(process(lines), 4);
    }

    #[test]
    fn test_example_result_2() {
        let lines = StringReader::new(
            "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
        );

        assert_eq!(process(lines), 4);
    }

    #[test]
    fn test_example_result_3() {
        let lines = StringReader::new(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        );

        assert_eq!(process(lines), 8);
    }

    #[test]
    fn test_example_result_4() {
        let lines = StringReader::new(
            "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
        );

        assert_eq!(process(lines), 8);
    }
}
