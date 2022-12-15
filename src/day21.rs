use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

struct Dice {
    next: u64,
    total: u64,
}

impl Dice {
    fn new() -> Self {
        Self { next: 1, total: 0 }
    }

    fn next(&mut self) -> u64 {
        let next = self.next;
        self.next = if self.next == 100 { 1 } else { self.next + 1 };
        self.total += 1;
        next
    }

    fn get(&mut self, num: usize) -> u64 {
        let t = (0..num).fold(0, |mut acc, _| {
            acc += self.next();
            acc
        });
        t
    }
}

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> (u64, u64) {
    let mut iter = input.lines();
    let mut next_digit = || {
        iter.next()
            .unwrap()
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap() as u64
    };
    (next_digit(), next_digit())
}

#[aoc(day21, part1, day21_1)]
pub fn part1(input: &(u64, u64)) -> u64 {
    let (mut p_one_pos, mut p_two_pos) = (input.0, input.1);
    let (mut p_one_score, mut p_two_score) = (0, 0);
    let mut dice = Dice::new();
    loop {
        p_one_pos = p_one_pos + dice.get(3);
        p_one_pos = if p_one_pos % 10 != 0 {
            p_one_pos % 10
        } else {
            10
        };
        p_one_score += p_one_pos;
        if p_one_score >= 1000 {
            return p_two_score * dice.total;
        }
        p_two_pos = p_two_pos + dice.get(3);
        p_two_pos = if p_two_pos % 10 != 0 {
            p_two_pos % 10
        } else {
            10
        };
        p_two_score += p_two_pos;
        if p_two_score >= 1000 {
            return p_two_score * dice.total;
        }
    }
}

const FREQ: [(u64, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn recurse(
    pos: u64,
    other_pos: u64,
    score: u64,
    other_score: u64,
    cache: &mut HashMap<(u64, u64, u64, u64), (u64, u64)>,
) -> (u64, u64) {
    let key = (pos, other_pos, score, other_score);
    if let Some(val) = cache.get(&key) {
        return *val;
    }

    if other_score >= 21 {
        return (0, 1);
    }

    let mut ways = (0, 0);
    for (roll, freq) in FREQ {
        let new_position = if (pos + roll) % 10 != 0 {
            (pos + roll) % 10
        } else {
            10
        };
        let new_score = score + new_position;

        let outcome = recurse(other_pos, new_position, other_score, new_score, cache);

        ways.0 += freq * outcome.1;
        ways.1 += freq * outcome.0;
    }

    cache.insert(key, ways);
    ways
}

#[aoc(day21, part2, day21_e)]
pub fn part2(input: &(u64, u64)) -> u64 {
    let (score_1, score_2) = recurse(input.0, input.1, 0, 0, &mut HashMap::new());
    score_1.max(score_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn test_part_1() {
        let input = input_generator(SAMPLE);
        assert_eq!(part1(&input), 739785);
    }

    #[test]
    fn test_part_2() {
        let input = input_generator(SAMPLE);
        assert_eq!(part2(&input), 444356092776315);
    }
}
