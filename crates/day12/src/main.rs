use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq)]
struct Line {
    chars: String,
    counts: Vec<usize>,
}

impl From<&str> for Line {
    fn from(value: &str) -> Self {
        let (chars, counts) = value.split_once(' ').unwrap();
        let counts = counts.split(',').map(|num| num.parse().unwrap()).collect();
        Self {
            chars: String::from(chars),
            counts,
        }
    }
}

fn get_counts(s: &str) -> Vec<usize> {
    s.split('.')
        .filter_map(|s| if s.is_empty() { None } else { Some(s.len()) })
        .collect()
}

fn get_replacement_line(s: &str, mut variant_num: usize, num_unknowns: usize) -> String {
    let mut s = String::from(s);
    for _ in 0..num_unknowns {
        if variant_num % 2 == 0 {
            s = s.replacen('?', ".", 1);
        } else {
            s = s.replacen('?', "#", 1);
        }
        variant_num >>= 1;
    }
    s
}

impl Line {
    pub fn count_line_variants(self) -> usize {
        let num_unknowns = self.chars.chars().filter(|c| *c == '?').count();
        let mut variants = 0;
        for variant_num in 0..(1 << num_unknowns) {
            let replaced = get_replacement_line(&self.chars, variant_num, num_unknowns);
            if get_counts(&replaced) == self.counts {
                variants += 1;
            }
        }
        variants
    }

    pub fn five(s: &str) -> Self {
        let (chars, counts) = s.split_once(' ').unwrap();
        let counts = format!("{},{},{},{},{}", counts, counts, counts, counts, counts)
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect();
        Self {
            chars: format!("{}{}{}{}{}", chars, chars, chars, chars, chars),
            counts,
        }
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
        assert_eq!(part1("???.### 1,1,3"), 1);
        assert_eq!(part1(TEST_INPUT), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 525152);
    }
}
