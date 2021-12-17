use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> (String, HashMap<[char; 2], char>) {
    let mut iter = input.lines();
    let start_string = iter.next().unwrap().to_string();
    let mut map = HashMap::new();
    iter.next();
    while let Some(line) = iter.next() {
        let line = line.replace(" -> ", "");
        let chars: Vec<_> = line.chars().collect();
        let key = [chars[0], chars[1]];
        map.insert(key, chars[2]);
    }
    (start_string, map)
}

struct Simulator {
    counts: HashMap<[char; 2], u64>,
    last_letter: char,
}

impl Simulator {
    fn from_str(input: &str) -> Self {
        let chars: Vec<_> = input.chars().collect();
        let mut counts = HashMap::new();
        for i in 1..chars.len() {
            *counts.entry([chars[i - 1], chars[i]]).or_insert(0) += 1;
        }
        Self {
            counts,
            last_letter: *chars.last().unwrap(),
        }
    }

    fn simluate(&mut self, rules: &HashMap<[char; 2], char>, rounds: usize) -> (u64, u64) {
        for _ in 0..rounds {
            self.tick(rules);
        }
        self.min_max()
    }

    fn tick(&mut self, rules: &HashMap<[char; 2], char>) {
        let mut new_counts =
            self.counts
                .iter()
                .fold(HashMap::new(), |mut acc, ([left, right], count)| {
                    let (left, right) = (*left, *right);
                    let between = *rules.get(&[left, right]).unwrap();
                    *acc.entry([left, between]).or_insert(0) += count;
                    *acc.entry([between, right]).or_insert(0) += count;
                    acc
                });
        std::mem::swap(&mut self.counts, &mut new_counts);
    }

    fn char_occurence(&self) -> HashMap<char, u64> {
        let mut occ = self
            .counts
            .iter()
            .fold(HashMap::new(), |mut acc, ([left, _], count)| {
                *acc.entry(*left).or_insert(0) += count;
                acc
            });
        // last letter is never counted so increment by 1;
        *occ.entry(self.last_letter).or_insert(0) += 1;
        occ
    }

    fn min_max(&self) -> (u64, u64) {
        let occurences = self.char_occurence();
        let (mut min, mut max) = (u64::MAX, 0);
        for &count in occurences.values() {
            min = min.min(count);
            max = max.max(count);
        }
        (min, max)
    }
}

#[aoc(day14, part1, day14_1)]
pub fn part1(input: &(String, HashMap<[char; 2], char>)) -> u64 {
    let (start_str, rules) = (input.0.as_str(), &input.1);

    let mut sim = Simulator::from_str(start_str);
    let (min, max) = sim.simluate(rules, 10);
    max - min
}

#[aoc(day14, part2, day14_2)]
pub fn part2(input: &(String, HashMap<[char; 2], char>)) -> u64 {
    let (start_str, rules) = (input.0.as_str(), &input.1);

    let mut sim = Simulator::from_str(start_str);
    let (min, max) = sim.simluate(rules, 40);
    max - min
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_part_1() {
        let input = input_generator(SAMPLE);
        assert_eq!(part1(&input), 1588);
    }

    #[test]
    fn test_part_2() {
        let input = input_generator(SAMPLE);
        assert_eq!(part2(&input), 2188189693529);
    }
}
