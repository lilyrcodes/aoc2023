use petgraph::{
    graph::{DiGraph, NodeIndex},
    visit::EdgeRef,
    Direction,
};
use std::{
    collections::HashMap,
    fmt::Debug,
    fs::read_to_string,
    rc::Rc,
    sync::Arc,
};

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

    pub fn invert(&self) -> Self {
        Self {
            field: self.field,
            operator: if self.operator == Operator::Greater {
                Operator::Less
            } else {
                Operator::Greater
            },
            value: if self.operator == Operator::Greater {
                self.value + 1
            } else {
                self.value - 1
            },
        }
    }

    pub fn to_range(&self) -> Range {
        match self.operator {
            Operator::Greater => Range {
                start: self.value + 1,
                size: 4000 - self.value,
            },
            Operator::Less => Range {
                start: 1,
                size: self.value - 1,
            },
        }
    }

    pub fn to_part_range(&self) -> PartRange {
        let range = self.to_range();
        match self.field {
            Field::X => PartRange {
                x: range,
                ..PartRange::default()
            },
            Field::M => PartRange {
                m: range,
                ..PartRange::default()
            },
            Field::A => PartRange {
                a: range,
                ..PartRange::default()
            },
            Field::S => PartRange {
                s: range,
                ..PartRange::default()
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Rule<T>
where
    T: Clone + PartialEq + Eq + Debug,
{
    condition: Option<Condition>,
    target: Stage<T>,
}

impl From<&str> for Rule<Arc<str>> {
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

impl<T> Rule<T>
where
    T: Clone + PartialEq + Eq + Debug,
{
    pub fn should_apply(&self, part: &Part) -> bool {
        if let Some(condition) = self.condition {
            condition.matches(part)
        } else {
            true
        }
    }

    pub fn get_stage(&self) -> Stage<T> {
        self.target.clone()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Workflow<T>
where
    T: Clone + PartialEq + Eq + Debug,
{
    name: Arc<str>,
    rules: Vec<Rule<T>>,
}

impl From<&str> for Workflow<Arc<str>> {
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

impl<T> Workflow<T>
where
    T: Clone + PartialEq + Eq + Debug,
{
    pub fn get_next_stage(&self, part: &Part) -> Stage<T> {
        self.rules
            .iter()
            .find(|rule| rule.should_apply(part))
            .unwrap()
            .get_stage()
    }
}

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
enum Stage<T>
where
    T: Clone + PartialEq + Eq + Debug,
{
    Accept,
    Reject,
    Workflow(T),
}

impl From<&str> for Stage<Arc<str>> {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::Workflow(value.into()),
        }
    }
}

impl Default for Stage<Arc<str>> {
    fn default() -> Self {
        Self::Workflow("in".into())
    }
}

impl<T> Stage<T>
where
    T: Clone + PartialEq + Eq + Debug,
{
    pub fn accepted(&self) -> bool {
        *self == Self::Accept
    }
}

struct Input {
    workflows: Vec<Workflow<usize>>,
    parts: Vec<Part>,
    starting_workflow: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Node {
    workflow_idx: usize,
    rule_idx: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
struct Range {
    start: u16,
    size: u16,
}

impl Default for Range {
    fn default() -> Self {
        Self {
            start: 1,
            size: 4000,
        }
    }
}

impl Range {
    pub fn end(&self) -> u16 {
        self.start + self.size
    }

    pub fn overlap(&self, other: &Self) -> Self {
        let start = u16::max(self.start, other.start);
        let end = u16::min(self.end(), other.end());
        let size = if start <= end { end - start } else { 0 };
        Self { start, size }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartRange {
    pub fn overlap(&self, other: &Self) -> Self {
        Self {
            x: self.x.overlap(&other.x),
            m: self.m.overlap(&other.m),
            a: self.a.overlap(&other.a),
            s: self.s.overlap(&other.s),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.x.size == 0 || self.m.size == 0 || self.a.size == 0 || self.s.size == 0
    }

    pub fn size(&self) -> usize {
        self.x.size as usize * self.m.size as usize * self.a.size as usize * self.s.size as usize
    }
}

fn part_ranges(
    graph_and_map: GraphAndMap,
    starting_index: usize,
) -> Vec<PartRange> {
    let graph: DiGraph<Rc<Node>, Option<Condition>> = graph_and_map.graph;
    let node_map: HashMap<Rc<Node>, NodeIndex> = graph_and_map.node_to_index;
    let accept_node: Rc<Node> = graph_and_map.accepted_node;
    let reject_node: Rc<Node> = graph_and_map.rejected_node;
    let mut ranges = Vec::new();
    let mut stack = Vec::new();
    stack.push((
        *node_map
            .get(&Rc::new(Node {
                workflow_idx: starting_index,
                rule_idx: 0,
            }))
            .unwrap(),
        PartRange::default(),
    ));
    while let Some((cur_node_index, cur_range)) = stack.pop() {
        let cur_node_weight = graph.node_weight(cur_node_index).unwrap();
        if cur_node_weight.clone() == accept_node {
            ranges.push(cur_range);
            continue;
        } else if cur_node_weight.clone() == reject_node {
            continue;
        }
        for edge in graph.edges_directed(cur_node_index, Direction::Outgoing) {
            let opt_condition = edge.weight();
            let next_node = edge.target();
            if let Some(condition) = opt_condition {
                let next_range = cur_range.overlap(&condition.to_part_range());
                if !next_range.is_zero() {
                    stack.push((next_node, next_range));
                }
            } else {
                stack.push((next_node, cur_range));
            }
        }
    }
    ranges
}

#[derive(Debug)]
struct GraphAndMap {
    graph: DiGraph<Rc<Node>, Option<Condition>>,
    node_to_index: HashMap<Rc<Node>, NodeIndex>,
    accepted_node: Rc<Node>,
    rejected_node: Rc<Node>,
}

fn make_graph(workflows: &[Workflow<usize>]) -> GraphAndMap {
    let mut graph = DiGraph::new();
    let mut node_map = HashMap::new();
    let accepted_node = Rc::new(Node {
        workflow_idx: usize::MAX,
        rule_idx: usize::MAX,
    });
    let rejected_node = Rc::new(Node {
        workflow_idx: usize::MAX,
        rule_idx: usize::MAX - 1,
    });
    node_map.insert(accepted_node.clone(), graph.add_node(accepted_node.clone()));
    node_map.insert(rejected_node.clone(), graph.add_node(rejected_node.clone()));
    for (workflow_idx, workflow) in workflows.iter().enumerate() {
        for (rule_idx, _) in workflow.rules.iter().enumerate() {
            let node = Node {
                workflow_idx,
                rule_idx,
            };
            node_map.insert(node.into(), graph.add_node(node.into()));
        }
    }
    for (workflow_idx, workflow) in workflows.iter().enumerate() {
        for (rule_idx, rule) in workflow.rules.iter().enumerate() {
            let start_node = Node {
                workflow_idx,
                rule_idx,
            };
            match rule.target {
                Stage::Workflow(workflow_idx) => {
                    let next_node = Node {
                        workflow_idx,
                        rule_idx: 0,
                    };
                    graph.add_edge(
                        *node_map.get(&start_node).unwrap(),
                        *node_map.get(&next_node).unwrap(),
                        rule.condition,
                    );
                }
                Stage::Accept => {
                    graph.add_edge(
                        *node_map.get(&start_node).unwrap(),
                        *node_map.get(&accepted_node).unwrap(),
                        rule.condition,
                    );
                }
                Stage::Reject => {
                    graph.add_edge(
                        *node_map.get(&start_node).unwrap(),
                        *node_map.get(&rejected_node).unwrap(),
                        rule.condition,
                    );
                }
            }
            if let Some(condition) = rule.condition {
                if rule_idx + 1 < workflows[workflow_idx].rules.len() {
                    if let Some(right_node) = node_map.get(&Rc::new(Node {
                        workflow_idx,
                        rule_idx: rule_idx + 1,
                    })) {
                        graph.add_edge(
                            *node_map.get(&start_node).unwrap(),
                            *right_node,
                            Some(condition.invert()),
                        );
                    }
                }
            }
        }
    }

    GraphAndMap {
        graph,
        node_to_index: node_map,
        accepted_node,
        rejected_node,
    }
}

fn convert_to_idx(
    workflows: Vec<Workflow<Arc<str>>>,
    name_map: HashMap<Arc<str>, usize>,
) -> Vec<Workflow<usize>> {
    workflows
        .into_iter()
        .map(|wf| Workflow {
            name: wf.name,
            rules: wf
                .rules
                .into_iter()
                .map(|rule| Rule {
                    condition: rule.condition,
                    target: match rule.target {
                        Stage::Workflow(name) => Stage::Workflow(*name_map.get(&name).unwrap()),
                        Stage::Accept => Stage::Accept,
                        Stage::Reject => Stage::Reject,
                    },
                })
                .collect(),
        })
        .collect()
}

fn parse_workflows(s: &str) -> Vec<Workflow<Arc<str>>> {
    s.lines().map(Workflow::from).collect()
}

fn workflow_name_to_idx(s: &str) -> HashMap<Arc<str>, usize> {
    s.lines()
        .map(Workflow::from)
        .enumerate()
        .map(|(idx, wf)| (wf.name, idx))
        .collect()
}

fn parse_parts(s: &str) -> Vec<Part> {
    s.lines().map(Part::from).collect()
}

fn parse_input(s: &str) -> Input {
    let (workflows, parts) = s.split_once("\n\n").unwrap();
    let name_map = workflow_name_to_idx(workflows);
    let workflows = parse_workflows(workflows);
    let starting_workflow = *name_map.get("in").unwrap();
    let workflows = convert_to_idx(workflows, name_map);
    let parts = parse_parts(parts);
    Input {
        workflows,
        parts,
        starting_workflow,
    }
}

fn accept_part(workflows: &[Workflow<usize>], starting_index: usize, part: &Part) -> bool {
    let mut stage = Stage::Workflow(starting_index);
    while let Stage::Workflow(idx) = stage {
        let workflow = &workflows[idx];
        stage = workflow.get_next_stage(part);
    }
    stage.accepted()
}

fn part1(s: &str) -> u64 {
    let input = parse_input(s);
    input
        .parts
        .iter()
        .filter(|part| accept_part(&input.workflows, input.starting_workflow, part))
        .map(|part| part.total() as u64)
        .sum()
}

fn part2(s: &str) -> usize {
    let input = parse_input(s);
    let graph = make_graph(&input.workflows);
    let ranges = part_ranges(
        graph,
        input.starting_workflow,
    );

    ranges.iter().map(|range| range.size()).sum()
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
        assert_eq!(part2(TEST_INPUT), 167409079868000);
    }

    #[test]
    fn test_part2_basic() {
        assert_eq!(part2("in{x<2001:A,R}

{x=1,m=1,a=1,s=1}"), 2000 * 4000 * 4000 * 4000);
        assert_eq!(part2("in{x<2001:A,A}

{x=1,m=1,a=1,s=1}"), 4000 * 4000 * 4000 * 4000);
        
        assert_eq!(part2("in{x<2001:A,b}
b{m>2000:A,R}

{x=1,m=1,a=1,s=1}"), 2000 * 4000 * 4000 * 4000 + 2000 * 2000 * 4000 * 4000);
    }

    #[test]
    fn test_part2_basic2() {
    }

    #[test]
    fn test_make_graph() {
        let start_node = Rc::new(Node { workflow_idx: 0, rule_idx: 0 });
        let accepted_node = Rc::new(Node {
            workflow_idx: usize::MAX,
            rule_idx: usize::MAX,
        });
        let rejected_node = Rc::new(Node {
            workflow_idx: usize::MAX,
            rule_idx: usize::MAX - 1,
        });
        let mut expected = DiGraph::new();
        let accepted_node = expected.add_node(accepted_node.clone());
        expected.add_node(rejected_node.clone());
        let start_node = expected.add_node(start_node.clone());
        let other_node = expected.add_node(Rc::new(Node { workflow_idx: 0, rule_idx: 1 }));
        expected.add_edge(start_node, accepted_node, Some(Condition { field: Field::X, operator: Operator::Less, value: 2001 }));
        expected.add_edge(start_node, other_node, Some(Condition { field: Field::X, operator: Operator::Greater, value: 2000 }));
        expected.add_edge(other_node, accepted_node, None);
        
        let input = parse_input("in{x<2001:A,A}

{x=1,m=1,a=1,s=1}");
        let graph = make_graph(&input.workflows);
        assert_eq!(graph.graph.node_weights().cloned().collect::<Vec<_>>(), expected.node_weights().cloned().collect::<Vec<_>>());
        assert_eq!(graph.graph.edge_weights().cloned().collect::<Vec<_>>(), expected.edge_weights().cloned().collect::<Vec<_>>());
    }

    #[test]
    fn test_part_ranges() {
        let input = parse_input("in{x<2001:A,A}

{x=1,m=1,a=1,s=1}");
        let graph = make_graph(&input.workflows);
        let ranges = part_ranges(
            graph,
            input.starting_workflow,
        );
        let expected_ranges = vec![
            PartRange { x: Range { start: 1, size: 2000 }, ..PartRange::default() },
            PartRange { x: Range { start: 2001, size: 2000 }, ..PartRange::default() },
        ];
        assert_eq!(ranges, expected_ranges);
    }

    #[test]
    fn test_invert_condition() {
        let condition = Condition { field: Field::X, operator: Operator::Greater, value: 2000 };
        let expected = Condition { field: Field::X, operator: Operator::Less, value: 2001 };
        assert_eq!(condition.invert(), expected);
        assert_eq!(expected.invert(), condition);
    }

    #[test]
    fn test_range_conversion() {
        let condition = Condition {
            field: Field::X,
            operator: Operator::Greater,
            value: 50,
        };
        let expected_range = PartRange {
            x: Range {
                start: 51,
                size: 3950,
            },
            m: Range::default(),
            a: Range::default(),
            s: Range::default(),
        };
        assert_eq!(condition.to_part_range(), expected_range);
        let condition = Condition {
            field: Field::X,
            operator: Operator::Less,
            value: 150,
        };
        let expected_range = PartRange {
            x: Range {
                start: 1,
                size: 149,
            },
            m: Range::default(),
            a: Range::default(),
            s: Range::default(),
        };
        assert_eq!(condition.to_part_range(), expected_range);
    }

    #[test]
    fn test_range_overlap() {
        let a = Range {
            start: 50,
            size: 51,
        };
        let b = Range {
            start: 100,
            size: 10,
        };
        let expected = Range {
            start: 100,
            size: 1,
        };
        assert_eq!(a.overlap(&b), expected);

        let a = Range {
            start: 50,
            size: 51,
        };
        let b = Range {
            start: 150,
            size: 10,
        };
        let expected = Range {
            start: 150,
            size: 0,
        };
        assert_eq!(a.overlap(&b), expected);

        let a = Range::default();
        let b = Range {
            start: 150,
            size: 10,
        };
        assert_eq!(a.overlap(&b), b);
    }
}
