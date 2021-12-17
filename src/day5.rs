use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

pub type Coord = (usize, usize);

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<(Coord, Coord)> {
    input
        .lines()
        .map(|line| {
            line.split_once(" -> ")
                .map(|(s, e)| {
                    let start = s
                        .split_once(",")
                        .map(|(ss, se)| {
                            (ss.parse::<usize>().unwrap(), se.parse::<usize>().unwrap())
                        })
                        .unwrap();
                    let end = e
                        .split_once(",")
                        .map(|(es, ee)| {
                            (es.parse::<usize>().unwrap(), ee.parse::<usize>().unwrap())
                        })
                        .unwrap();
                    (start, end)
                })
                .unwrap()
        })
        .collect()
}

#[aoc(day5, part1, d51)]
pub fn part1(input: &Vec<(Coord, Coord)>) -> usize {
    let mut coords = Vec::new();
    let mut positions = HashSet::new();
    let mut overlap = HashSet::new();
    for &(start, end) in input {
        if start.0 == end.0 {
            for y in start.1.min(end.1)..=start.1.max(end.1) {
                coords.push((start.0, y));
            }
        } else if start.1 == end.1 {
            for x in start.0.min(end.0)..=start.0.max(end.0) {
                coords.push((x, start.1));
            }
        }
    }
    for coord in coords {
        if positions.contains(&coord) {
            overlap.insert(coord);
        }
        positions.insert(coord);
    }
    overlap.len()
}

#[aoc(day5, part2, d52)]
pub fn part2(input: &Vec<(Coord, Coord)>) -> usize {
    let mut coords = Vec::new();
    let mut positions = HashSet::new();
    let mut overlap = HashSet::new();
    for &(start, end) in input {
        if start.0 == end.0 {
            for y in start.1.min(end.1)..=start.1.max(end.1) {
                coords.push((start.0, y));
            }
        } else if start.1 == end.1 {
            for x in start.0.min(end.0)..=start.0.max(end.0) {
                coords.push((x, start.1));
            }
        } else {
            let (mut x, mut y) = start;
            loop {
                coords.push((x, y));
                if x == end.0 && y == end.1 {
                    break;
                }
                if x < end.0 {
                    x += 1;
                } else {
                    x -= 1;
                };
                if y < end.1 {
                    y += 1;
                } else {
                    y -= 1;
                };
            }
        }
    }
    for coord in coords {
        if positions.contains(&coord) {
            overlap.insert(coord);
        }
        positions.insert(coord);
    }
    overlap.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_part_1() {
        let input = input_generator(SAMPLE);
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn test_part_2() {
        let input = input_generator(SAMPLE);
        assert_eq!(part2(&input), 12);
    }
}
