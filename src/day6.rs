use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> [u64; 9] {
    let mut fish_count = [0_u64; 9];
    for fish in input.split(",") {
        let day: usize = fish.parse().unwrap();
        fish_count[day] += 1;
    }
    fish_count
}

#[aoc(day6, part1, d61)]
pub fn part1(input: &[u64; 9]) -> u64 {
    let mut count = input.clone();
    for _ in 0..80 {
        let reset = count[0];
        count.rotate_left(1);
        count[6] += reset;
    }
    count.into_iter().sum()
}

#[aoc(day6, part2, d62)]
pub fn part2(input: &[u64; 9]) -> u64 {
    let mut count = input.clone();
    for _ in 0..256 {
        let reset = count[0];
        count.rotate_left(1);
        count[6] += reset;
    }
    count.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn test_part_1() {
        let input = input_generator(SAMPLE);
        assert_eq!(part1(&input), 5934);
    }

    #[test]
    fn test_part_2() {
        let input = input_generator(SAMPLE);
        assert_eq!(part2(&input), 26984457539);
    }
}
