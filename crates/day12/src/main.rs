use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Spring {
    Unknown,
    Damaged,
    Operational,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '?' => Self::Unknown,
            '#' => Self::Damaged,
            _ => Self::Operational,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Line {
    springs: Vec<Spring>,
    counts: Vec<usize>,
}

impl From<&str> for Line {
    fn from(value: &str) -> Self {
        let (chars, counts) = value.split_once(' ').unwrap();
        let springs = chars.chars().map(Spring::from).collect();
        let counts = counts.split(',').map(|num| num.parse().unwrap()).collect();
        Self { springs, counts }
    }
}

fn get_counts_recursive(
    map: &mut HashMap<(Line, Spring), usize>,
    line: Line,
    prev: Spring,
) -> usize {
    let key = (line.clone(), prev);
    if let Some(result) = map.get(&key) {
        return *result;
    }
    let (springs, target_counts) = (line.springs, line.counts);
    if springs.is_empty()
        && (target_counts.is_empty() || (target_counts.len() == 1 && target_counts[0] == 0))
    {
        return 1;
    } else if springs.is_empty() {
        return 0;
    } else if target_counts.is_empty() {
        // Invalid if no targets and still some damaged.
        if springs.iter().any(|spring| *spring == Spring::Damaged) {
            return 0;
        }
    }

    let result = match (prev, springs[0]) {
        (Spring::Operational, Spring::Operational) => get_counts_recursive(
            map,
            Line {
                springs: springs[1..].to_vec(),
                counts: target_counts,
            },
            Spring::Operational,
        ),
        (Spring::Damaged, Spring::Operational) => {
            if target_counts[0] == 0 {
                get_counts_recursive(
                    map,
                    Line {
                        springs: springs[1..].to_vec(),
                        counts: target_counts[1..].to_vec(),
                    },
                    Spring::Operational,
                )
            } else {
                0
            }
        }
        (_, Spring::Damaged) => {
            if target_counts[0] == 0 {
                0
            } else {
                get_counts_recursive(
                    map,
                    Line {
                        springs: springs[1..].to_vec(),
                        counts: Some(target_counts[0] - 1)
                            .into_iter()
                            .chain(target_counts[1..].iter().copied())
                            .collect::<Vec<usize>>(),
                    },
                    Spring::Damaged,
                )
            }
        }
        (_, Spring::Unknown) => {
            get_counts_recursive(
                map,
                Line {
                    springs: Some(Spring::Damaged)
                        .into_iter()
                        .chain(springs[1..].iter().copied())
                        .collect::<Vec<Spring>>(),
                    counts: target_counts.clone(),
                },
                prev,
            ) + get_counts_recursive(
                map,
                Line {
                    springs: Some(Spring::Operational)
                        .into_iter()
                        .chain(springs[1..].iter().copied())
                        .collect::<Vec<Spring>>(),
                    counts: target_counts,
                },
                prev,
            )
        }
        (_, _) => panic!("Shouldn't be able to have 'Unknown' as prev"),
    };
    map.insert(key, result);
    result
}

impl Line {
    pub fn count_line_variants(self) -> usize {
        get_counts_recursive(&mut HashMap::new(), self, Spring::Operational)
    }

    pub fn five(s: &str) -> Self {
        let (left, right) = s.split_once(' ').unwrap();
        let expanded = format!(
            "{}?{}?{}?{}?{} {},{},{},{},{}",
            left, left, left, left, left, right, right, right, right, right
        );
        Self::from(expanded.as_str())
    }
}

fn part1(s: &str) -> usize {
    s.lines()
        .map(Line::from)
        .map(Line::count_line_variants)
        .sum()
}

fn part2(s: &str) -> usize {
    s.lines()
        .map(Line::five)
        .map(Line::count_line_variants)
        .sum()
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

    const TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_part1() {
        assert_eq!(part1("??? 2,1"), 0);
        assert_eq!(part1("???? 2,1"), 1);
        assert_eq!(part1("???.### 1,1,3"), 1);
        assert_eq!(part1(".??..??...?##. 1,1,3"), 4);
        assert_eq!(part1("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(part1("????.#...#... 4,1,1"), 1);
        assert_eq!(part1("????.######..#####. 1,6,5"), 4);
        assert_eq!(part1("?###???????? 3,2,1"), 10);
        assert_eq!(part1(TEST_INPUT), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 525152);
    }
}
