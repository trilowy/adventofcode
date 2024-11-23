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
    let mut max_x = 0;
    let mut max_y = 0;

    let mut graph = UnGraphMap::new();
    let mut edges = HashMap::new();

    for (y, line) in BufReader::new(file).lines().flatten().enumerate() {
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
            max_x = x;
        }
        max_y = y;
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

    let mut loop_nodes: Vec<NodeId> = dijkstra(&graph, start, None, |_| 1).into_keys().collect();
    loop_nodes.push(start);

    let mut result = 0;

    for y in 0..=max_y {
        for x in 0..=max_x {
            if !loop_nodes.contains(&NodeId { x, y }) {
                let is_inside = (x + 1..=max_x)
                    .filter(|&x| loop_nodes.contains(&NodeId { x, y }))
                    .count()
                    % 2
                    == 1;
                if is_inside {
                    result += 1;
                }
            }
        }
    }

    result
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
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        );

        assert_eq!(process(lines), 4);
    }

    #[test]
    fn test_example_result_2() {
        let lines = StringReader::new(
            ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        );

        assert_eq!(process(lines), 8);
    }

    #[test]
    fn test_example_result_3() {
        let lines = StringReader::new(
            "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        );

        assert_eq!(process(lines), 10);
    }
}
