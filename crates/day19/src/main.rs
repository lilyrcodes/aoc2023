use std::{
    collections::HashMap,
    fs::read_to_string,
    sync::{mpsc, Arc},
    thread,
};
use threadpool::ThreadPool;

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
    value: u16,
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
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Rule {
    condition: Option<Condition>,
    target: Stage,
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        if let Some((left, right)) = value.split_once(':') {
            let condition = Some(Condition::from(left));
            Self {
                condition,
                target: right.into(),
            }
        } else {
            Self {
                condition: None,
                target: value.into(),
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
        self.target.clone()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Workflow {
    name: Arc<str>,
    rules: Vec<Rule>,
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let (name, rules) = value.split_once('{').unwrap();
        let rules = &rules[0..rules.len() - 1];
        let rules = rules.split(',').map(Rule::from).collect();
        Self {
            name: name.into(),
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
}

type WorkflowMap = HashMap<Arc<str>, Workflow>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
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
    pub fn total(&self) -> u16 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Stage {
    Accept,
    Reject,
    Workflow(Arc<str>),
}

impl From<&str> for Stage {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::Workflow(value.into()),
        }
    }
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

fn parse_workflows(s: &str) -> WorkflowMap {
    s.lines()
        .map(Workflow::from)
        .map(|wf| (wf.name.clone(), wf))
        .collect()
}

fn parse_parts(s: &str) -> Vec<Part> {
    s.lines().map(Part::from).collect()
}

fn parse_input(s: &str) -> (WorkflowMap, Vec<Part>) {
    let (workflows, parts) = s.split_once("\n\n").unwrap();
    (parse_workflows(workflows), parse_parts(parts))
}

fn accept_part(workflows: &WorkflowMap, part: &Part) -> bool {
    let mut stage = Stage::default();
    while let Stage::Workflow(name) = &stage {
        let workflow = workflows.get(name).unwrap();
        stage = workflow.get_next_stage(part);
    }
    stage.accepted()
}

fn part1(s: &str) -> u64 {
    let (workflows, parts) = parse_input(s);
    parts
        .iter()
        .filter(|part| accept_part(&workflows, part))
        .map(|part| part.total() as u64)
        .sum()
}

fn part2(s: &str) -> usize {
    let (workflows, _) = parse_input(s);
    let (tx, rx) = mpsc::channel();
    let (done_tx, done_rx) = mpsc::channel();
    let pool = ThreadPool::default();
    for x in 0..=4000 {
        let tx = tx.clone();
        let done_tx = done_tx.clone();
        let workflows = workflows.clone();
        pool.execute(move || {
            for m in 0..=4000 {
                for a in 0..=4000 {
                    for s in 0..=4000 {
                        if accept_part(&workflows, &Part { x, m, a, s }) {
                            tx.send(()).unwrap();
                        }
                    }
                }
                done_tx.send(()).unwrap();
            }
        });
    }
    let (count_tx, count_rx) = mpsc::channel();
    thread::spawn(move || {
        let mut done = 0;
        while let Ok(()) = done_rx.recv() {
            done += 1;
            println!("{} / 16000000", done);
        }
    });
    thread::spawn(move || {
        let mut count = 0;
        while let Ok(()) = rx.recv() {
            count += 1;
        }
        count_tx.send(count).unwrap();
    });
    count_rx.recv().unwrap()
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 952408144115);
    }
}
