use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum State {
    On,
    Off,
}

#[derive(Clone, Debug)]
pub struct Switch {
    state: State,
    bounds: ((i64, i64), (i64, i64), (i64, i64)),
}

impl Switch {
    fn new() -> Self {
        Self {
            state: State::Off,
            bounds: ((0, 0), (0, 0), (0, 0)),
        }
    }
    fn from_str(input: &str) -> Self {
        let mut out = Self::new();
        let mut iter = input.split(' ');
        out.state = match iter.next().unwrap() {
            "on" => State::On,
            "off" => State::Off,
            val => panic!("invalid input: {}", val),
        };
        let mut coord = iter.next().unwrap().split(',').map(|n| {
            let mut parts = n[2..].split("..");
            (
                parts.next().unwrap().parse::<i64>().unwrap(),
                parts.next().unwrap().parse::<i64>().unwrap(),
            )
        });

        let (x_start, x_end) = coord.next().unwrap();
        let (y_start, y_end) = coord.next().unwrap();
        let (z_start, z_end) = coord.next().unwrap();
        out.bounds = ((x_start, x_end), (y_start, y_end), (z_start, z_end));
        out
    }

    fn clip(&mut self, min: i64, max: i64) {
        let helper = |bound: &mut (i64, i64)| {
            bound.0 = bound.0.max(min);
            bound.1 = bound.1.min(max);
            if bound.1 < bound.0 {
                bound.1 = bound.0 - 1;
            }
            *bound = (bound.0, bound.1)
        };
        helper(&mut self.bounds.0);
        helper(&mut self.bounds.1);
        helper(&mut self.bounds.2);
    }

    fn coords(&self) -> Box<dyn Iterator<Item = (i64, i64, i64)>> {
        let x_range = self.bounds.0 .0..self.bounds.0 .1 + 1;
        let y_range = self.bounds.1 .0..self.bounds.1 .1 + 1;
        let z_range = self.bounds.2 .0..self.bounds.2 .1 + 1;
        Box::new(itertools::iproduct!(x_range, y_range, z_range))
    }
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Vec<Switch> {
    input.lines().fold(Vec::new(), |mut acc, el| {
        acc.push(Switch::from_str(el));
        acc
    })
}

#[aoc(day22, part1, day22_1)]
pub fn part1(input: &[Switch]) -> u64 {
    let input = input.to_vec();
    let mut toggled = HashSet::new();
    for mut switch in input.into_iter() {
        switch.clip(-50, 50);
        let mut coords = switch.coords();
        if switch.state == State::Off {
            while let Some(key) = coords.next() {
                toggled.remove(&key);
            }
        } else {
            while let Some(key) = coords.next() {
                toggled.insert(key);
            }
        }
    }
    toggled.len() as u64
}

#[aoc(day22, part2, day22_e)]
pub fn part2(input: &[Switch]) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

    #[test]
    fn test_part_1() {
        let input = input_generator(SAMPLE);
        assert_eq!(part1(&input), 590784);
    }

    #[test]
    fn test_part_2() {
        let input = input_generator(SAMPLE);
        assert_eq!(part2(&input), 444356092776315);
    }
}
