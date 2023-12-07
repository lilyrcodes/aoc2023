use std::fs::read_to_string;

#[derive(Debug, Clone, PartialEq)]
struct MapEntry {
    source_start: usize,
    source_end: usize,
    offset: i64,
}

impl From<&str> for MapEntry {
    fn from(value: &str) -> Self {
        let nums: Vec<usize> = value
            .split_whitespace()
            .map(|entry| entry.parse::<usize>().unwrap())
            .collect();
        let dest_start = nums[0];
        let source_start = nums[1];
        let range = nums[2];
        Self {
            source_start,
            source_end: source_start + range - 1,
            offset: dest_start as i64 - source_start as i64,
        }
    }
}

impl MapEntry {
    pub fn map_source(&self, num: usize) -> Option<usize> {
        if self.source_start <= num && num <= self.source_end {
            Some((num as i64 + self.offset) as usize)
        } else {
            None
        }
    }
}

struct Map {
    entries: Vec<MapEntry>,
}

impl Map {
    pub fn map_source(&self, num: usize) -> usize {
        for entry in self.entries.iter() {
            if let Some(value) = entry.map_source(num) {
                return value;
            }
        }
        num
    }
}

struct Data {
    start_numbers: Vec<usize>,
    maps: Vec<Map>,
}

impl Data {
    fn map_source(&self, mut num: usize) -> usize {
        for map in self.maps.iter() {
            num = map.map_source(num);
        }
        num
    }

    pub fn calc_lowest(&self) -> usize {
        let mut lowest = self.map_source(self.start_numbers[0]);
        for num in self.start_numbers.iter().skip(1) {
            let end = self.map_source(*num);
            if end < lowest {
                lowest = end;
            }
        }
        lowest
    }

    pub fn calc_lowest_ranges(&self) -> usize {
        let mut lowest = self.map_source(self.start_numbers[0]);
        let mut iter = self.start_numbers.iter();
        while let Some(start) = iter.next() {
            let range = iter.next().unwrap();
            for num in *start..(*start + *range) {
                let end = self.map_source(num);
                if end < lowest {
                    lowest = end;
                }
            }
        }
        lowest
    }
}

fn parse_input(s: &str) -> Data {
    let mut maps: Vec<Map> = Vec::default();
    let mut buf = Vec::default();
    let start_numbers: Vec<usize> = s
        .lines()
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    for line in s.lines().skip(2) {
        if line.is_empty() {
            maps.push(Map {
                entries: buf.into_iter().map(MapEntry::from).collect(),
            });
            buf = Vec::default();
        } else if line.contains("map") {
        } else {
            buf.push(line);
        }
    }
    if !buf.is_empty() {
        maps.push(Map {
            entries: buf.into_iter().map(MapEntry::from).collect(),
        });
    }
    Data {
        start_numbers,
        maps,
    }
}

fn part1(s: &str) -> u64 {
    let data = parse_input(s);
    data.calc_lowest() as u64
}

fn part2(s: &str) -> u64 {
    let data = parse_input(s);
    data.calc_lowest_ranges() as u64
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let answer1 = part1(&input);
    println!("Part 1: {}", answer1);
    let answer2 = part2(&input);
    println!("Part 2: {}", answer2);
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_parse_line() {
        let foo = MapEntry::from("50 98 2");
        assert_eq!(foo.source_start, 98);
        assert_eq!(foo.source_end, 99);
        assert_eq!(foo.offset, -48);
    }

    #[test]
    fn test_part1() {
        let actual = part1(TEST_INPUT);
        assert_eq!(actual, 35);
    }

    #[test]
    fn test_part2() {
        let actual = part2(TEST_INPUT);
        assert_eq!(actual, 46);
    }
}
