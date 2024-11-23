use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
}

struct Rectangle {
    top_left_corner: Coordinates,
    bottom_right_corner: Coordinates,
}

impl Rectangle {
    fn is_inside(&self, coordinates: &Coordinates) -> bool {
        coordinates.x >= self.top_left_corner.x
            && coordinates.y >= self.top_left_corner.y
            && coordinates.x <= self.bottom_right_corner.x
            && coordinates.y <= self.bottom_right_corner.y
    }
}

#[derive(Debug)]
struct Number {
    number: String,
    coordinates: Coordinates,
}

impl Number {
    fn is_consecutive(&self, x: usize, y: usize) -> bool {
        y as i32 == self.coordinates.y && x as i32 == self.coordinates.x + self.number.len() as i32
    }

    fn get_neighbor_zone(&self) -> Rectangle {
        let top_left_corner = Coordinates {
            x: self.coordinates.x - 1,
            y: self.coordinates.y - 1,
        };
        let bottom_right_corner = Coordinates {
            x: self.coordinates.x + self.number.len() as i32,
            y: self.coordinates.y + 1,
        };
        Rectangle {
            top_left_corner,
            bottom_right_corner,
        }
    }

    fn has_symbol_in_neighbor(&self, symbols: &Vec<Coordinates>) -> bool {
        let neighbor_zone = self.get_neighbor_zone();

        symbols.iter().any(|symbol| neighbor_zone.is_inside(symbol))
    }
}

#[derive(Debug)]
struct Schematic {
    numbers: Vec<Number>,
    symbols: Vec<Coordinates>,
}

impl Schematic {
    fn from(file: File) -> Self {
        let mut schematic = Self {
            numbers: Vec::new(),
            symbols: Vec::new(),
        };

        for (y, line) in BufReader::new(file).lines().flatten().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    c if c.is_ascii_digit() => schematic.add_digit(c, x, y),
                    '.' => (),
                    _ => schematic.add_symbol(x, y),
                }
            }
        }

        schematic
    }

    fn get_all_numbers_with_symbol(&self) -> Vec<u32> {
        self.numbers
            .iter()
            .map(|number| {
                if number.has_symbol_in_neighbor(&self.symbols) {
                    number.number.parse().ok()
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }

    fn add_digit(&mut self, digit: char, x: usize, y: usize) {
        if let Some(last_number) = self.numbers.last_mut() {
            if last_number.is_consecutive(x, y) {
                last_number.number.push(digit);
                return;
            }
        }

        self.numbers.push(Number {
            number: digit.to_string(),
            coordinates: Coordinates::new(x, y),
        });
    }

    fn add_symbol(&mut self, x: usize, y: usize) {
        self.symbols.push(Coordinates::new(x, y))
    }
}

fn main() {
    let file = File::open("input.txt").expect("opening input file");

    let schematic = Schematic::from(file);

    let result: u32 = schematic.get_all_numbers_with_symbol().into_iter().sum();

    println!("{result}");
}

#[cfg(test)]
mod tests {
    use crate::{Coordinates, Number, Rectangle};

    #[test]
    fn is_inside_rectangle() {
        let rectangle = Rectangle {
            top_left_corner: Coordinates { x: 1, y: 2 },
            bottom_right_corner: Coordinates { x: 3, y: 4 },
        };

        assert!(rectangle.is_inside(&Coordinates { x: 1, y: 2 }));
        assert!(rectangle.is_inside(&Coordinates { x: 3, y: 4 }));
        assert!(rectangle.is_inside(&Coordinates { x: 1, y: 3 }));
        assert!(rectangle.is_inside(&Coordinates { x: 2, y: 4 }));
        assert!(rectangle.is_inside(&Coordinates { x: 2, y: 3 }));

        assert!(!rectangle.is_inside(&Coordinates { x: 0, y: 2 }));
        assert!(!rectangle.is_inside(&Coordinates { x: 3, y: 5 }));
        assert!(!rectangle.is_inside(&Coordinates { x: 0, y: 1 }));
        assert!(!rectangle.is_inside(&Coordinates { x: 4, y: 5 }));

        let rectangle = Rectangle {
            top_left_corner: Coordinates { x: 4, y: -1 },
            bottom_right_corner: Coordinates { x: 8, y: 1 },
        };

        assert!(!rectangle.is_inside(&Coordinates { x: 3, y: 1 }));
        assert!(!rectangle.is_inside(&Coordinates { x: 6, y: 3 }));
        assert!(!rectangle.is_inside(&Coordinates { x: 3, y: 4 }));
        assert!(!rectangle.is_inside(&Coordinates { x: 5, y: 5 }));
        assert!(!rectangle.is_inside(&Coordinates { x: 5, y: 8 }));
    }

    #[test]
    fn get_neighbor_zone() {
        let number = Number {
            number: "114".to_string(),
            coordinates: Coordinates { x: 5, y: 0 },
        };
        let rectangle = number.get_neighbor_zone();

        assert_eq!(rectangle.top_left_corner.x, 4);
        assert_eq!(rectangle.top_left_corner.y, -1);
        assert_eq!(rectangle.bottom_right_corner.x, 8);
        assert_eq!(rectangle.bottom_right_corner.y, 1);
    }

    #[test]
    fn has_symbol_in_neighbor() {
        let number = Number {
            number: "114".to_string(),
            coordinates: Coordinates { x: 5, y: 0 },
        };
        let symbols = vec![
            Coordinates { x: 3, y: 1 },
            Coordinates { x: 6, y: 3 },
            Coordinates { x: 3, y: 4 },
            Coordinates { x: 5, y: 5 },
            Coordinates { x: 3, y: 8 },
            Coordinates { x: 5, y: 8 },
        ];

        assert!(!number.has_symbol_in_neighbor(&symbols));
    }
}
