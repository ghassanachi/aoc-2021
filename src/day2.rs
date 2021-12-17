use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(String, i64)> {
    input
        .lines()
        .map(|n| {
            let mut parts = n.split(" ");
            (
                parts.next().unwrap().to_string(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

#[aoc(day2, part1, d21)]
pub fn part1(commands: &[(String, i64)]) -> i64 {
    let mut depth = 0;
    let mut horizontal = 0;
    for (action, count) in commands {
        match (action.as_str(), count) {
            ("forward", val) => horizontal += val,
            ("up", val) => depth -= val,
            ("down", val) => depth += val,
            _ => unreachable!(),
        }
    }
    depth * horizontal
}

#[aoc(day2, part2, d22)]
pub fn part2(commands: &[(String, i64)]) -> i64 {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    for (action, count) in commands {
        match (action.as_str(), count) {
            ("forward", val) => {
                horizontal += val;
                depth += aim * val
            }
            ("up", val) => aim -= val,
            ("down", val) => aim += val,
            _ => unreachable!(),
        }
    }
    depth * horizontal
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

    #[test]
    fn test_part_1() {
        let input = input_generator(SAMPLE);
        assert_eq!(part1(&input), 150);
    }

    #[test]
    fn test_part_2() {
        let input = input_generator(SAMPLE);
        assert_eq!(part2(&input), 900);
    }
}
