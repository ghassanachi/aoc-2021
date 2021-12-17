use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy)]
pub struct Card {
    board: [[(u32, bool); 5]; 5],
    remaining: u32,
}

impl Card {
    fn new(input: &[&str]) -> Self {
        let mut board = [[(0, false); 5]; 5];
        let mut remaining = 0;
        for row in 0..5 {
            let nums: Vec<u32> = input[row]
                .trim()
                .split(" ")
                .filter_map(|n| {
                    if n.trim().is_empty() {
                        None
                    } else {
                        Some(n.parse().unwrap())
                    }
                })
                .collect();

            for col in 0..5 {
                remaining += nums[col];
                board[row][col].0 = nums[col];
            }
        }
        Self { board, remaining }
    }

    fn mark(&mut self, num: u32) -> Option<u32> {
        let (mut fr, mut fc) = (None, None);
        'outer: for row in 0..5 {
            for col in 0..5 {
                if self.board[row][col].0 == num {
                    self.board[row][col].1 = true;
                    self.remaining -= num;
                    fr = Some(row);
                    fc = Some(col);
                    break 'outer;
                }
            }
        }
        if let (Some(r), Some(c)) = (fr, fc) {
            let (mut r_win, mut c_win) = (true, true);
            for row in 0..5 {
                if self.board[row][c].1 == false {
                    r_win = false;
                    break;
                }
            }
            for col in 0..5 {
                if self.board[r][col].1 == false {
                    c_win = false;
                    break;
                }
            }
            if r_win || c_win {
                return Some(self.remaining);
            }
        }
        None
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> (Vec<u32>, Vec<Card>) {
    let lines: Vec<&str> = input.lines().collect();
    let picks: Vec<u32> = lines[0].split(",").map(|n| n.parse().unwrap()).collect();
    let mut cards = Vec::new();
    for i in (2..lines.len()).step_by(6) {
        cards.push(Card::new(&lines[i..i + 5]));
    }
    (picks, cards)
}

#[aoc(day4, part1, d41)]
pub fn part1(input: &(Vec<u32>, Vec<Card>)) -> u32 {
    let (picks, mut boards) = (&input.0, input.1.clone());
    for &pick in picks {
        for board in &mut boards {
            if let Some(r) = board.mark(pick) {
                return r * pick;
            }
        }
    }
    unreachable!("Bingo was never reached");
}

#[aoc(day4, part2, d42)]
pub fn part2(input: &(Vec<u32>, Vec<Card>)) -> u32 {
    let (picks, mut boards) = (&input.0, input.1.clone());

    for &pick in picks {
        if boards.len() == 1 {
            if let Some(r) = boards[0].mark(pick) {
                return r * pick;
            }
        } else {
            boards = boards
                .into_iter()
                .filter_map(|mut b| {
                    if b.mark(pick).is_none() {
                        Some(b)
                    } else {
                        None
                    }
                })
                .collect();
        }
    }
    unreachable!("Bingo was never reached");
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_part_1() {
        let input = input_generator(SAMPLE);
        assert_eq!(part1(&input), 4512);
    }

    #[test]
    fn test_part_2() {
        let input = input_generator(SAMPLE);
        assert_eq!(part2(&input), 1924);
    }
}
