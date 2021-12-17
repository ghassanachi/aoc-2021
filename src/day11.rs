use aoc_runner_derive::{aoc, aoc_generator};
use colored::*;
use prettytable::{Cell, Row, Table};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Octopus {
    Unflashed(u8),
    Flashed,
}

// Implement `Display` for `MinMax`.
impl std::fmt::Display for Octopus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Octopus::Unflashed(val) => write!(f, "{}", val),
            Octopus::Flashed => write!(f, "{}", "0".red()),
        }
    }
}

#[allow(dead_code)]
fn print_table(grid: &[Vec<Octopus>], step: usize, count: u64) {
    let mut table = Table::new();

    table.set_titles(Row::new(vec![
        Cell::new(&step.to_string()),
        Cell::new(&count.to_string()),
    ]));
    for row in grid {
        let formatted: Vec<_> = row.iter().map(|n| Cell::new(&n.to_string())).collect();
        table.add_row(Row::new(formatted));
    }

    table.printstd();
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Vec<Octopus>> {
    input.lines().fold(Vec::new(), |mut acc, line| {
        acc.push(
            line.chars()
                .map(|c| Octopus::Unflashed(c.to_digit(10).unwrap() as u8))
                .collect(),
        );
        acc
    })
}

const ADJENCENT: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[aoc(day11, part1, d111)]
pub fn part1(grid: &[Vec<Octopus>]) -> u64 {
    let mut grid = grid.to_owned();
    let (m, n) = (grid.len(), grid[0].len());
    let mut total_flashes = 0;
    for _ in 0..100 {
        let mut flash_points = Vec::new();
        for row in 0..m {
            for col in 0..n {
                match grid[row].get_mut(col).unwrap() {
                    Octopus::Unflashed(val) => {
                        *val += 1;
                        if *val > 9 {
                            flash_points.push((row, col));
                        }
                    }
                    _ => panic!("SHIT"),
                }
            }
        }

        while let Some((row, col)) = flash_points.pop() {
            if grid[row][col] == Octopus::Flashed {
                continue;
            }
            grid[row][col] = Octopus::Flashed;
            total_flashes += 1;

            let neighbors: Vec<_> = ADJENCENT
                .into_iter()
                .flat_map(|(or, oc)| {
                    let (r, c) = (row as i32 + or, col as i32 + oc);
                    if r >= 0
                        && r < m as i32
                        && c >= 0
                        && c < n as i32
                        && grid[r as usize][c as usize] != Octopus::Flashed
                    {
                        Some((r as usize, c as usize))
                    } else {
                        None
                    }
                })
                .collect();
            for (r, c) in neighbors {
                if let Octopus::Unflashed(val) = grid[r].get_mut(c).unwrap() {
                    *val += 1;
                    if *val > 9 {
                        flash_points.push((r, c));
                    }
                } else {
                    panic!("SHIT");
                }
            }
        }
        for row in 0..m {
            for col in 0..n {
                if grid[row][col] == Octopus::Flashed {
                    grid[row][col] = Octopus::Unflashed(0);
                }
            }
        }
    }
    total_flashes
}

#[aoc(day11, part2, d112)]
pub fn part2(grid: &[Vec<Octopus>]) -> u64 {
    let mut grid = grid.to_owned();
    let (m, n) = (grid.len(), grid[0].len());
    for step in 0.. {
        let mut flash_count = 0;
        let mut flash_points = Vec::new();
        for row in 0..m {
            for col in 0..n {
                match grid[row].get_mut(col).unwrap() {
                    Octopus::Unflashed(val) => {
                        *val += 1;
                        if *val > 9 {
                            flash_points.push((row, col));
                        }
                    }
                    _ => panic!("SHIT"),
                }
            }
        }

        while let Some((row, col)) = flash_points.pop() {
            if grid[row][col] == Octopus::Flashed {
                continue;
            }
            grid[row][col] = Octopus::Flashed;
            flash_count += 1;

            let neighbors: Vec<_> = ADJENCENT
                .into_iter()
                .flat_map(|(or, oc)| {
                    let (r, c) = (row as i32 + or, col as i32 + oc);
                    if r >= 0
                        && r < m as i32
                        && c >= 0
                        && c < n as i32
                        && grid[r as usize][c as usize] != Octopus::Flashed
                    {
                        Some((r as usize, c as usize))
                    } else {
                        None
                    }
                })
                .collect();
            for (r, c) in neighbors {
                if let Octopus::Unflashed(val) = grid[r].get_mut(c).unwrap() {
                    *val += 1;
                    if *val > 9 {
                        flash_points.push((r, c));
                    }
                } else {
                    panic!("SHIT");
                }
            }
        }
        if flash_count == m * n {
            return step + 1;
        }
        for row in 0..m {
            for col in 0..n {
                if grid[row][col] == Octopus::Flashed {
                    grid[row][col] = Octopus::Unflashed(0);
                }
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_part_1() {
        let input = input_generator(SAMPLE);
        assert_eq!(part1(&input), 1_656);
    }

    #[test]
    fn test_part_2() {
        let input = input_generator(SAMPLE);
        assert_eq!(part2(&input), 195);
    }
}
