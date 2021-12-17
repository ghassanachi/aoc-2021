use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().fold(Vec::new(), |mut acc, line| {
        acc.push(line.chars().collect());
        acc
    })
}

#[aoc(day10, part1, d101)]
pub fn part1(input: &[Vec<char>]) -> u64 {
    let mut stack = Vec::new();
    let mut result = 0;
    for line in input {
        for p in line {
            if matches!(p, '(' | '[' | '{' | '<') {
                stack.push(p);
                continue;
            }
            match (stack.pop(), p) {
                (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') | (Some('<'), '>') => {}
                _ => {
                    result += match p {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => panic!("invalid input with char {}", p),
                    };
                }
            }
        }
    }
    result
}

#[aoc(day10, part2, d102)]
pub fn part2(input: &[Vec<char>]) -> u64 {
    let mut scores = Vec::new();
    'outer: for line in input {
        let mut stack = Vec::new();
        for p in line {
            if matches!(p, '(' | '[' | '{' | '<') {
                stack.push(p);
                continue;
            }
            match (stack.pop(), p) {
                (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') | (Some('<'), '>') => {}
                _ => continue 'outer,
            }
        }
        if stack.is_empty() {
            continue;
        }
        let mut score = 0;
        while let Some(o) = stack.pop() {
            score *= 5;
            score += match o {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("invalid input with char {}", o),
            };
        }
        scores.push(score)
    }
    scores.sort();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part_1() {
        let input = input_generator(SAMPLE);
        assert_eq!(part1(&input), 26_397);
    }

    #[test]
    fn test_part_2() {
        let input = input_generator(SAMPLE);
        assert_eq!(part2(&input), 288_957);
    }
}
