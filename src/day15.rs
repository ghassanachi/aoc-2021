use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::BinaryHeap;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<Vec<u64>> {
    input.lines().fold(Vec::new(), |mut acc, line| {
        let row = line
            .chars()
            .map(|n| n.to_digit(10).unwrap() as u64)
            .collect();
        acc.push(row);
        acc
    })
}

#[derive(Debug, PartialEq, Eq)]
struct Position {
    row: usize,
    col: usize,
    weight: u64,
    next: Option<(usize, usize)>,
}

impl Position {
    fn new(grid: &[Vec<u64>], row: usize, col: usize) -> Self {
        Self {
            row,
            col,
            weight: grid[row][col],
            next: None,
        }
    }

    fn end(grid: &[Vec<u64>]) -> Self {
        let (m, n) = (grid.len(), grid[0].len());
        Self::new(grid, m - 1, n - 1)
    }

    fn from_next(&self, grid: &[Vec<u64>], row: usize, col: usize) -> Self {
        let mut out = Self::new(grid, row, col);
        out.weight += self.weight;
        out.next = Some((self.row, self.col));
        out
    }

    fn neighboors(&self, grid: &[Vec<u64>]) -> Vec<Position> {
        let (m, n) = (grid.len() as i32, grid[0].len() as i32);
        let (r, c) = (self.row as i32, self.col as i32);
        [(-1, 0), (1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(|(or, oc)| (or + r, oc + c))
            .filter(|(or, oc)| or >= &0 && or < &m && oc >= &0 && oc < &n)
            .map(|(or, oc)| self.from_next(grid, or as usize, oc as usize))
            .collect()
    }

    fn is_start(&self) -> bool {
        self.row == 0 && self.col == 0
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.weight.partial_cmp(&self.weight)
    }
}

fn shortest_path(grid: &[Vec<u64>]) -> u64 {
    let (m, n) = (grid.len(), grid[0].len());

    let mut pq: BinaryHeap<Position> = BinaryHeap::new();
    let mut visited = vec![vec![false; n]; m];
    pq.push(Position::end(grid));

    let start = loop {
        let pos = pq.pop().unwrap();
        if pos.is_start() {
            break pos;
        }
        if visited[pos.row][pos.col] {
            continue;
        }
        visited[pos.row][pos.col] = true;
        pos.neighboors(grid).into_iter().for_each(|n| pq.push(n));
    };
    start.weight - grid[0][0]
}

#[aoc(day15, part1, day15_1)]
pub fn part1(grid: &Vec<Vec<u64>>) -> u64 {
    shortest_path(grid)
}

fn extend_grid(grid: &Vec<Vec<u64>>, size: u64) -> Vec<Vec<u64>> {
    let mut map = grid.clone();
    // extend vertically
    for offset in 1..size {
        let mut new_block = grid.clone();
        new_block.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|cell| {
                if *cell + offset <= 9 {
                    *cell += offset
                } else {
                    *cell = (*cell + offset) % 9;
                }
            })
        });
        for row in new_block {
            map.push(row);
        }
    }
    // extend horizontally
    for row in 0..map.len() {
        let template = map[row].clone();
        for offset in 1..size {
            let mut new_seg = template.clone();
            new_seg.iter_mut().for_each(|cell| {
                if *cell + offset <= 9 {
                    *cell += offset
                } else {
                    *cell = (*cell + offset) % 9;
                }
            });
            map[row].append(&mut new_seg);
        }
    }
    map
}

#[aoc(day15, part2, day15_2)]
pub fn part2(grid: &Vec<Vec<u64>>) -> u64 {
    let map = extend_grid(grid, 5);
    shortest_path(&map)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test_part_1() {
        let input = input_generator(SAMPLE);
        assert_eq!(part1(&input), 40);
    }

    #[test]
    fn test_part_2() {
        let input = input_generator(SAMPLE);
        assert_eq!(part2(&input), 315);
    }
}
