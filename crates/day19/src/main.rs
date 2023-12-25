use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operator {
    Greater,
    Less,
}

impl From<char> for Operator {
    fn from(value: char) -> Self {
        match value {
            '>' => Self::Greater,
            '<' => Self::Less,
            _ => panic!("Unknown value for operator."),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Field {
    X,
    M,
    A,
    S,
}

impl From<char> for Field {
    fn from(value: char) -> Self {
        match value {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => panic!("Unknown value for field."),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Condition {
    field: Field,
    operator: Operator,
    value: i64,
}

impl From<&str> for Condition {
    fn from(value: &str) -> Self {
        let field = Field::from(value.chars().nth(0).unwrap());
        let operator = Operator::from(value.chars().nth(1).unwrap());
        let value = value[2..].parse().unwrap();
        Self {
            field,
            operator,
            value,
        }
    }
}

impl Condition {
    pub fn matches(&self, part: &Part) -> bool {
        let field_value = match self.field {
            Field::X => part.x,
            Field::M => part.m,
            Field::A => part.a,
            Field::S => part.s,
        };
        match self.operator {
            Operator::Greater => field_value > self.value,
            Operator::Less => field_value < self.value,
        }
    }

    pub fn get_range(&self) -> Range {
        match self.operator {
            Operator::Greater => Range {
                start: self.value as u16,
                end: 4000,
            },
            Operator::Less => Range {
                start: 0,
                end: self.value as u16,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Rule {
    condition: Option<Condition>,
    target: String,
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        if let Some((left, right)) = value.split_once(':') {
            let condition = Some(Condition::from(left));
            let target = right.to_string();
            Self { condition, target }
        } else {
            Self {
                condition: None,
                target: value.to_string(),
            }
        }
    }
}

impl Rule {
    pub fn should_apply(&self, part: &Part) -> bool {
        if let Some(condition) = self.condition {
            condition.matches(part)
        } else {
            true
        }
    }

    pub fn get_stage(&self) -> Stage {
        match self.target.as_str() {
            "A" => Stage::Accept,
            "R" => Stage::Reject,
            _ => Stage::Workflow(self.target.clone()),
        }
    }

    pub fn get_accept_range(&self, workflows: &HashMap<String, Workflow>) -> Option<AcceptsRange> {
        todo!()
    }

    pub fn get_condition_range(&self) -> Option<Range> {
        self.condition.as_ref().map(Condition::get_range)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Range {
    start: u16,
    end: u16,
}

impl Default for Range {
    fn default() -> Self {
        Self {
            start: 0,
            end: 4000,
        }
    }
}

impl Range {
    pub fn size(&self) -> u16 {
        self.end - self.start + 1
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct AcceptsRange {
    x: Vec<Range>,
    m: Vec<Range>,
    a: Vec<Range>,
    s: Vec<Range>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let (name, rules) = value.split_once('{').unwrap();
        let rules = &rules[0..rules.len() - 1];
        let rules = rules.split(',').map(Rule::from).collect();
        Self {
            name: name.to_string(),
            rules,
        }
    }
}

impl Workflow {
    pub fn get_next_stage(&self, part: &Part) -> Stage {
        self.rules
            .iter()
            .find(|rule| rule.should_apply(part))
            .unwrap()
            .get_stage()
    }

    pub fn get_accepted_ranges(
        &self,
        workflows: &HashMap<String, Workflow>,
    ) -> Option<AcceptsRange> {
        for rule in self.rules.iter() {
            let rule_range = rule.get_accept_range(workflows);
        }
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let parts = &value[1..value.len() - 1];
        let mut part = Part::default();
        for s in parts.split(',') {
            let val = s[2..s.len()].parse().unwrap();
            match s.chars().nth(0).unwrap() {
                'x' => part.x = val,
                'm' => part.m = val,
                'a' => part.a = val,
                's' => part.s = val,
                _ => panic!("Unknown field"),
            }
        }
        part
    }
}

impl Part {
    pub fn total(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Stage {
    Accept,
    Reject,
    Workflow(String),
}

impl Default for Stage {
    fn default() -> Self {
        Self::Workflow("in".into())
    }
}

impl Stage {
    pub fn accepted(&self) -> bool {
        *self == Self::Accept
    }
}

fn parse_workflows(s: &str) -> HashMap<String, Workflow> {
    s.lines()
        .map(Workflow::from)
        .map(|wf| (wf.name.clone(), wf))
        .collect()
}

fn parse_parts(s: &str) -> Vec<Part> {
    s.lines().map(Part::from).collect()
}

fn parse_input(s: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let (workflows, parts) = s.split_once("\n\n").unwrap();
    (parse_workflows(workflows), parse_parts(parts))
}

fn accept_part(workflows: &HashMap<String, Workflow>, part: &Part) -> bool {
    let mut stage = Stage::default();
    while let Stage::Workflow(name) = &stage {
        let workflow = workflows.get(name).unwrap();
        stage = workflow.get_next_stage(part);
    }
    stage.accepted()
}

fn part1(s: &str) -> i64 {
    let (workflows, parts) = parse_input(s);
    parts
        .iter()
        .filter(|part| accept_part(&workflows, part))
        .map(Part::total)
        .sum()
}

fn part2(s: &str) -> usize {
    todo!()
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

    const TEST_INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 19114);
    }

    /*
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 952408144115);
    }
    */
}
