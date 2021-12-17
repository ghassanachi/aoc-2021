use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Default)]
pub struct Graph {
    start: Option<Cave>,
    edges: HashMap<Cave, HashSet<Cave>>,
}

impl Graph {
    fn new() -> Self {
        Default::default()
    }

    fn add_edge(&mut self, from: Cave, to: Cave) {
        if from.is_start() {
            self.start = Some(from.clone());
        }
        if to.is_start() {
            self.start = Some(to.clone())
        }
        self.edges
            .entry(from.clone())
            .or_insert(HashSet::new())
            .insert(to.clone());
        self.edges.entry(to).or_insert(HashSet::new()).insert(from);
    }

    fn get_edges(&self, from: &Cave) -> &HashSet<Cave> {
        self.edges.get(from).unwrap()
    }

    fn start_cave(&self) -> &Cave {
        assert!(self.start.is_some());
        self.start.as_ref().unwrap()
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum Cave {
    Large(String),
    Small(String),
}

impl Cave {
    fn to_tuple(name: &str) -> (Self, Self) {
        let mut parts = name.split("-");
        (
            Self::from(parts.next().unwrap()),
            Self::from(parts.next().unwrap()),
        )
    }

    fn from(name: &str) -> Self {
        match name.to_uppercase().eq(name) {
            true => Cave::Large(name.to_string()),
            false => Cave::Small(name.to_string()),
        }
    }

    fn name(&self) -> &str {
        match self {
            Cave::Large(name) => &name,
            Cave::Small(name) => &name,
        }
    }

    fn is_start(&self) -> bool {
        self.name() == "start"
    }

    fn is_end(&self) -> bool {
        self.name() == "end"
    }

    fn is_small(&self) -> bool {
        match self {
            Cave::Large(_) => false,
            Cave::Small(_) => true,
        }
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Graph {
    let mut graph = Graph::new();
    for line in input.lines() {
        let (from, to) = Cave::to_tuple(line);
        graph.add_edge(from, to);
    }
    graph
}

fn dfs_1(graph: &Graph, node: &Cave, visited: &mut HashSet<Cave>) -> u64 {
    if visited.contains(node) {
        return 0;
    }
    if node.is_end() {
        return 1;
    }
    if node.is_small() {
        visited.insert(node.clone());
    }
    let mut ways = 0;
    for to in graph.get_edges(node) {
        ways += dfs_1(graph, to, visited);
    }
    visited.remove(node);
    ways
}

fn dfs_2(graph: &Graph, node: &Cave, visited: &mut HashSet<Cave>, visit_used: &mut bool) -> u64 {
    if visited.contains(node) && (node.is_start() || node.is_end() || *visit_used) {
        return 0;
    }
    if node.is_end() {
        return 1;
    }

    let mut double_visit = false;
    if node.is_small() {
        if visited.contains(node) && !*visit_used {
            *visit_used = true;
            double_visit = true
        }
        visited.insert(node.clone());
    }

    let mut ways = 0;
    for to in graph.get_edges(node) {
        ways += dfs_2(graph, to, visited, visit_used);
    }

    if !double_visit {
        visited.remove(node);
    }
    if double_visit {
        *visit_used = false
    }

    ways
}

#[aoc(day12, part1, d121)]
pub fn part1(graph: &Graph) -> u64 {
    let mut visited = HashSet::new();
    dfs_1(graph, graph.start_cave(), &mut visited)
}

#[aoc(day12, part2, d122)]
pub fn part2(graph: &Graph) -> u64 {
    let mut visited = HashSet::new();
    let mut visit_used = false;
    dfs_2(graph, graph.start_cave(), &mut visited, &mut visit_used)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_1: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const SAMPLE_2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const SAMPLE_3: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn test_part_1() {
        let input = input_generator(SAMPLE_1);
        assert_eq!(part1(&input), 10);
        let input = input_generator(SAMPLE_2);
        assert_eq!(part1(&input), 19);
        let input = input_generator(SAMPLE_3);
        assert_eq!(part1(&input), 226);
    }

    #[test]
    fn test_part_2() {
        let input = input_generator(SAMPLE_1);
        assert_eq!(part2(&input), 36);
        let input = input_generator(SAMPLE_2);
        assert_eq!(part2(&input), 103);
        let input = input_generator(SAMPLE_3);
        assert_eq!(part2(&input), 3509);
    }
}
