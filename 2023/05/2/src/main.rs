use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
    str::FromStr,
    string::ParseError,
    time::Instant,
};

fn main() {
    let start = Instant::now();

    let file = File::open("input.txt").expect("opening input file");
    let mut lines = BufReader::new(file).lines().flatten();

    let mut seeds: Seeds = lines.next().unwrap().parse().unwrap();

    let mut seed_mappings = SeedMappings::new();
    let lines = lines.skip(2);

    for line in lines {
        if line.is_empty() {
            seeds.apply(&mut seed_mappings);
        } else if line.chars().next().unwrap().is_ascii_digit() {
            seed_mappings.add(line.parse().unwrap());
        }
    }
    seeds.apply(&mut seed_mappings);

    let result = seeds.min();

    println!("{result}");
    println!("{:?}", start.elapsed());
}

struct Seeds {
    seed_ranges: Vec<Range<i64>>,
}

impl FromStr for Seeds {
    type Err = ParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let seed_ranges = text
            .split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .map(|value| value.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
            .chunks_exact(2)
            .map(|range| Range {
                start: range[0],
                end: range[0] + range[1],
            })
            .collect();

        Ok(Self { seed_ranges })
    }
}

impl Seeds {
    fn apply(&mut self, seed_mappings: &mut SeedMappings) {
        let mut new_seed_ranges = Vec::new();

        while let Some(range) = self.seed_ranges.pop() {
            let mut has_overlapped = false;

            for seed_mapping in seed_mappings.seed_mappings.iter() {
                if range.is_empty() {
                    break;
                } else if seed_mapping.includes(&range) {
                    has_overlapped = true;
                    let shifted_range = seed_mapping.shift_included(&range);
                    new_seed_ranges.push(shifted_range);
                    break;
                } else if seed_mapping.is_included_in(&range) {
                    has_overlapped = true;
                    let (starting_range, shifted_range, ending_range) =
                        seed_mapping.shift_included_in(&range);

                    self.seed_ranges.push(starting_range);
                    new_seed_ranges.push(shifted_range);
                    self.seed_ranges.push(ending_range);
                    break;
                } else if seed_mapping.includes_start(&range) {
                    has_overlapped = true;
                    let (shifted_range, ending_range) = seed_mapping.shift_included_start(&range);

                    new_seed_ranges.push(shifted_range);
                    self.seed_ranges.push(ending_range);
                    break;
                } else if seed_mapping.includes_end(&range) {
                    has_overlapped = true;
                    let (starting_range, shifted_range) = seed_mapping.shift_included_end(&range);

                    self.seed_ranges.push(starting_range);
                    new_seed_ranges.push(shifted_range);
                    break;
                }
            }

            if !has_overlapped {
                new_seed_ranges.push(range);
            }
        }

        self.seed_ranges = new_seed_ranges;

        seed_mappings.clear();
    }

    fn min(&self) -> i64 {
        self.seed_ranges
            .iter()
            .map(|range| range.start)
            .min()
            .unwrap_or_default()
    }
}

#[derive(Debug)]
struct SeedMapping {
    source_range: Range<i64>,
    shift: i64,
}

impl FromStr for SeedMapping {
    type Err = ParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let entries: Vec<i64> = text
            .split_whitespace()
            .map(|entry| entry.parse().unwrap())
            .collect();

        Ok(Self {
            source_range: Range {
                start: entries[1],
                end: entries[1] + entries[2],
            },
            shift: entries[0] - entries[1],
        })
    }
}

impl SeedMapping {
    fn includes(&self, range: &Range<i64>) -> bool {
        self.source_range.contains(&range.start) && self.source_range.contains(&(range.end - 1))
    }

    fn shift_included(&self, range: &Range<i64>) -> Range<i64> {
        Range {
            start: range.start + self.shift,
            end: range.end + self.shift,
        }
    }

    fn is_included_in(&self, range: &Range<i64>) -> bool {
        range.contains(&self.source_range.start) && range.contains(&(self.source_range.end - 1))
    }

    fn shift_included_in(&self, range: &Range<i64>) -> (Range<i64>, Range<i64>, Range<i64>) {
        let starting_range = Range {
            start: range.start,
            end: self.source_range.start,
        };
        let shifted_range = Range {
            start: self.source_range.start + self.shift,
            end: self.source_range.end + self.shift,
        };
        let ending_range = Range {
            start: self.source_range.end,
            end: range.end,
        };

        (starting_range, shifted_range, ending_range)
    }

    fn includes_start(&self, range: &Range<i64>) -> bool {
        self.source_range.contains(&range.start)
    }

    fn shift_included_start(&self, range: &Range<i64>) -> (Range<i64>, Range<i64>) {
        let shifted_range = Range {
            start: range.start + self.shift,
            end: self.source_range.end + self.shift,
        };
        let ending_range = Range {
            start: self.source_range.end,
            end: range.end,
        };

        (shifted_range, ending_range)
    }

    fn includes_end(&self, range: &Range<i64>) -> bool {
        self.source_range.contains(&(range.end - 1))
    }

    fn shift_included_end(&self, range: &Range<i64>) -> (Range<i64>, Range<i64>) {
        let starting_range = Range {
            start: range.start,
            end: self.source_range.start,
        };
        let shifted_range = Range {
            start: self.source_range.start + self.shift,
            end: range.end + self.shift,
        };

        (starting_range, shifted_range)
    }
}

struct SeedMappings {
    seed_mappings: Vec<SeedMapping>,
}

impl SeedMappings {
    fn new() -> Self {
        Self {
            seed_mappings: Vec::new(),
        }
    }

    fn add(&mut self, seed_mapping: SeedMapping) {
        self.seed_mappings.push(seed_mapping);
    }

    fn clear(&mut self) {
        self.seed_mappings.clear();
    }
}

#[cfg(test)]
mod tests {
    use crate::SeedMapping;
    use std::ops::Range;

    #[test]
    fn test_includes() {
        let seed_mapping = SeedMapping {
            source_range: Range { start: 5, end: 9 },
            shift: 2,
        };

        let range_1 = Range { start: 5, end: 9 };
        let range_2 = Range { start: 6, end: 8 };

        assert!(seed_mapping.includes(&range_1));
        assert!(seed_mapping.includes(&range_2));
    }

    #[test]
    fn test_shift_included() {
        let seed_mapping = SeedMapping {
            source_range: Range { start: 5, end: 9 },
            shift: 2,
        };

        let range_1 = Range { start: 5, end: 9 };
        let range_2 = Range { start: 6, end: 8 };

        assert_eq!(
            seed_mapping.shift_included(&range_1),
            Range { start: 7, end: 11 }
        );
        assert_eq!(
            seed_mapping.shift_included(&range_2),
            Range { start: 8, end: 10 }
        );
    }

    #[test]
    fn test_is_included_in() {
        let seed_mapping = SeedMapping {
            source_range: Range { start: 5, end: 9 },
            shift: 2,
        };

        let range = Range { start: 4, end: 10 };

        assert!(seed_mapping.is_included_in(&range));
    }

    #[test]
    fn test_shift_included_in() {
        let seed_mapping = SeedMapping {
            source_range: Range { start: 5, end: 9 },
            shift: 2,
        };

        let range = Range { start: 4, end: 10 };

        assert_eq!(
            seed_mapping.shift_included_in(&range),
            (
                Range { start: 4, end: 5 },
                Range { start: 7, end: 11 },
                Range { start: 9, end: 10 }
            )
        );
    }

    #[test]
    fn test_includes_start() {
        let seed_mapping = SeedMapping {
            source_range: Range { start: 5, end: 9 },
            shift: 2,
        };

        let range = Range { start: 5, end: 10 };

        assert!(seed_mapping.includes_start(&range));
    }

    #[test]
    fn test_shift_included_start() {
        let seed_mapping = SeedMapping {
            source_range: Range { start: 5, end: 9 },
            shift: 2,
        };

        let range = Range { start: 6, end: 10 };

        assert_eq!(
            seed_mapping.shift_included_start(&range),
            (Range { start: 8, end: 11 }, Range { start: 9, end: 10 })
        );
    }

    #[test]
    fn test_includes_end() {
        let seed_mapping = SeedMapping {
            source_range: Range { start: 5, end: 9 },
            shift: 2,
        };

        let range = Range { start: 4, end: 9 };

        assert!(seed_mapping.includes_end(&range));
    }

    #[test]
    fn test_shift_included_end() {
        let seed_mapping = SeedMapping {
            source_range: Range { start: 5, end: 9 },
            shift: 2,
        };

        let range = Range { start: 4, end: 8 };

        assert_eq!(
            seed_mapping.shift_included_end(&range),
            (Range { start: 4, end: 5 }, Range { start: 7, end: 10 })
        );
    }
}
