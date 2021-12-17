use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1, d11)]
pub fn part1(nums: &[i64]) -> i64 {
    let mut count = 0;
    let mut prev = None;
    for &val in nums {
        if let Some(p) = prev {
            if p < val {
                count += 1;
            }
        }
        prev = Some(val);
    }
    count
}

#[aoc(day1, part2, d12)]
pub fn part2(nums: &[i64]) -> i64 {
    let mut count = 0;
    let mut prev_sum = nums[0..3].iter().fold(0, |acc, el| acc + el);
    for i in 3..nums.len() {
        let cur_sum = prev_sum - nums[i - 3] + nums[i];
        if prev_sum < cur_sum {
            count += 1;
        }
        prev_sum = cur_sum;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
    const SAMPLE2: &str = "607\n618\n618\n617\n647\n716\n769\n792";

    #[test]
    fn test_part_1() {
        let input = input_generator(SAMPLE1);
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn test_part_2() {
        let input = input_generator(SAMPLE2);
        assert_eq!(part2(&input), 5);
    }
}
