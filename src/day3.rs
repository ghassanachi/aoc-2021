use aoc_runner_derive::{aoc, aoc_generator};

fn get_frequency(nums: &[u32], bits: usize) -> Vec<(u32, u32)> {
    let mut counts = vec![(0, 0); bits];
    for &num in nums {
        for pos in 0..bits {
            if (num & (1 << pos)) == 0 {
                counts[pos as usize].0 += 1;
            } else {
                counts[pos as usize].1 += 1;
            }
        }
    }
    counts
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> (Vec<u32>, usize) {
    let mut len = 0;
    let nums: Vec<_> = input
        .lines()
        .map(|n| {
            len = n.len();
            u32::from_str_radix(n, 2).unwrap()
        })
        .collect();
    assert!(!nums.is_empty());
    (nums, len)
}

#[aoc(day3, part1, d31)]
pub fn part1(input: &(Vec<u32>, usize)) -> u32 {
    let (nums, bits) = input;
    let bits = *bits;
    let counts = get_frequency(&nums, 12);
    let mut gamma = 0;
    let mut epsilon = 0;
    for pos in 0..bits {
        if counts[pos].1 > counts[pos].0 {
            gamma |= 1 << pos;
        } else {
            epsilon |= 1 << pos;
        }
    }
    gamma * epsilon
}

#[aoc(day3, part2, d32)]
pub fn part2(input: &(Vec<u32>, usize)) -> u32 {
    let (nums, bits) = input;
    let bits = *bits;
    let (mut o2, mut co2) = (nums.clone().to_owned(), nums.clone().to_owned());
    for pos in (0..bits).rev() {
        let counts = get_frequency(&o2, bits);
        o2 = o2
            .into_iter()
            .filter(|n| {
                if counts[pos].1 >= counts[pos].0 {
                    return n & (1 << pos) != 0;
                } else {
                    return n & (1 << pos) == 0;
                }
            })
            .collect();
        if o2.len() <= 1 {
            break;
        }
    }

    for pos in (0..bits).rev() {
        let counts = get_frequency(&co2, bits);
        co2 = co2
            .into_iter()
            .filter(|n| {
                if counts[pos].1 < counts[pos].0 {
                    return n & (1 << pos) != 0;
                } else {
                    return n & (1 << pos) == 0;
                }
            })
            .collect();
        if co2.len() <= 1 {
            break;
        }
    }
    assert!(!o2.is_empty(), "o2 is empty");
    assert!(!co2.is_empty(), "co2 is empty");
    let (o2, co2) = (o2[0], co2[0]);
    o2 * co2
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    #[test]
    fn test_part_1() {
        let input = input_generator(SAMPLE);
        assert_eq!(part1(&input), 198);
    }

    #[test]
    fn test_part_2() {
        let input = input_generator(SAMPLE);
        assert_eq!(part2(&input), 230);
    }
}
