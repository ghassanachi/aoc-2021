use aoc_runner_derive::{aoc, aoc_generator};

fn get_min_fuel<F: FnMut(i64) -> i64>(positions: &[i64], mut calc: F) -> i64 {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    let fuel = (min..=max)
        .map(|p| positions.iter().map(|o| calc((p - o).abs())).sum())
        .min()
        .unwrap();

    fuel
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.split(",").map(|n| n.parse().unwrap()).collect()
}

#[aoc(day7, part1, d71)]
pub fn part1(input: &[i64]) -> i64 {
    fn calc(num: i64) -> i64 {
        num
    }
    get_min_fuel(input, calc)
}

#[aoc(day7, part2, d72)]
pub fn part2(input: &[i64]) -> i64 {
    fn calc(num: i64) -> i64 {
        (num * (num + 1)) / 2
    }
    get_min_fuel(input, calc)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part_1() {
        let input = input_generator(SAMPLE);
        assert_eq!(part1(&input), 37);
    }

    #[test]
    fn test_part_2() {
        let input = input_generator(SAMPLE);
        assert_eq!(part2(&input), 168);
    }
}
