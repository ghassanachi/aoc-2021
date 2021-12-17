use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec<u64>> {
    input.lines().fold(Vec::new(), |mut acc, l| {
        acc.push(
            l.split("")
                .filter(|c| !c.is_empty())
                .map(|n| n.parse().unwrap())
                .collect(),
        );
        acc
    })
}

fn find_minimas(grid: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let (m, n) = (grid.len(), grid[0].len());
    let mut minimas = Vec::new();
    for row in 0..m {
        for col in 0..n {
            assert!(grid[row].len() == n);
            let current = grid[row][col];
            if row > 0 && grid[row - 1][col] <= current
                || row + 1 < m && grid[row + 1][col] <= current
                || col > 0 && grid[row][col - 1] <= current
                || col + 1 < n && grid[row][col + 1] <= current
            {
                continue;
            }
            minimas.push((row, col));
        }
    }
    minimas
}

#[aoc(day9, part1, d91)]
pub fn part1(grid: &[Vec<u64>]) -> u64 {
    find_minimas(grid)
        .into_iter()
        .map(|(row, col)| grid[row][col] + 1)
        .sum()
}

#[aoc(day9, part2, d92)]
pub fn part2(grid: &[Vec<u64>]) -> u64 {
    let mut grid = grid.to_owned();
    let (m, n) = (grid.len(), grid[0].len());
    let local_minima = find_minimas(&grid);

    let mut result = Vec::new();

    for (r, c) in local_minima {
        let mut queue = vec![(r, c)];
        let mut basin_size = 0;

        while let Some((row, col)) = queue.pop() {
            if grid[row][col] == 9 {
                continue;
            }
            grid[row][col] = 9;
            basin_size += 1;
            if row > 0 {
                queue.push((row - 1, col))
            }
            if row + 1 < m {
                queue.push((row + 1, col))
            }
            if col > 0 {
                queue.push((row, col - 1))
            }
            if col + 1 < n {
                queue.push((row, col + 1))
            }
        }
        result.push(basin_size);
    }
    result.sort();
    result.reverse();
    result[0..3].into_iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_part_1() {
        let input = input_generator(SAMPLE);
        assert_eq!(part1(&input), 15);
    }

    #[test]
    fn test_part_2() {
        let input = input_generator(SAMPLE);
        assert_eq!(part2(&input), 1134);
    }
}
