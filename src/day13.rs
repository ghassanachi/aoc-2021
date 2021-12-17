use aoc_runner_derive::{aoc, aoc_generator};
use prettytable::{Cell as TableCell, Row, Table};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    Empty,
    Full,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, " "),
            Cell::Full => write!(f, "#"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Fold {
    Y(usize),
    X(usize),
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> (Vec<Vec<Cell>>, Vec<Fold>) {
    let (m, n) = find_size(input);
    let mut matrix = vec![vec![Cell::Empty; n]; m];
    let mut folds = Vec::new();

    let mut iter = input.lines();

    while let Some(point) = iter.next() {
        if point.is_empty() {
            break;
        }
        let mut coord = point.split(',');
        let (col, row): (usize, usize) = (
            coord.next().unwrap().parse().unwrap(),
            coord.next().unwrap().parse().unwrap(),
        );
        matrix[row][col] = Cell::Full;
    }

    while let Some(line) = iter.next() {
        let line = line.replace("fold along", "");
        let mut parts = line.trim().split('=');
        let fold = match (parts.next().unwrap(), parts.next().unwrap()) {
            ("y", val) => Fold::Y(val.parse().unwrap()),
            ("x", val) => Fold::X(val.parse().unwrap()),
            _ => panic!("invalid fold input"),
        };
        folds.push(fold);
    }
    (matrix, folds)
}

fn find_size(input: &str) -> (usize, usize) {
    let (mut row_size, mut col_size) = (0, 0);
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let mut coord = line.split(',');
        let (col, row) = (
            coord.next().unwrap().parse().unwrap(),
            coord.next().unwrap().parse().unwrap(),
        );
        row_size = row_size.max(row);
        col_size = col_size.max(col);
    }
    (row_size + 1, col_size + 1)
}

#[allow(dead_code)]
fn get_table(grid: &[Vec<Cell>]) -> Table {
    let mut table = Table::new();
    for row in grid {
        let formatted: Vec<_> = row.iter().map(|n| TableCell::new(&n.to_string())).collect();
        table.add_row(Row::new(formatted));
    }
    table
}

fn fold(matrix: &[Vec<Cell>], fold: Fold) -> Vec<Vec<Cell>> {
    match fold {
        Fold::Y(along) => fold_y(matrix, along),
        Fold::X(along) => fold_x(matrix, along),
    }
}

fn fold_y(matrix: &[Vec<Cell>], along: usize) -> Vec<Vec<Cell>> {
    let (m, n) = (matrix.len(), matrix[0].len());
    let (before, after) = (along, m - 1 - along);
    let new_m = before.max(after);
    let overlap_size = before.min(after);

    let mut out = vec![vec![Cell::Empty; n]; new_m];

    let no_overlap: Vec<_> = if new_m == before {
        (0..before - after).collect()
    } else {
        (m - (after - before)..m).rev().collect()
    };

    let mut row = 0;
    for &rc in &no_overlap {
        out[row] = matrix[rc].clone();
        row += 1
    }

    let overlap = (along - overlap_size..along).zip((along + 1..along + 1 + overlap_size).rev());
    for (br, ar) in overlap {
        for col in 0..n {
            if matrix[br][col] == Cell::Full || matrix[ar][col] == Cell::Full {
                out[row][col] = Cell::Full;
            }
        }
        row += 1;
    }
    out
}

fn fold_x(matrix: &[Vec<Cell>], along: usize) -> Vec<Vec<Cell>> {
    let (m, n) = (matrix.len(), matrix[0].len());
    let (before, after) = (along, n - 1 - along);
    let new_n = before.max(after);
    let overlap_size = before.min(after);

    let mut out = vec![vec![Cell::Empty; new_n]; m];

    let no_overlap: Vec<_> = if new_n == before {
        (0..before - after).collect()
    } else {
        (n - (after - before)..n).rev().collect()
    };

    let mut col = 0;
    for &cc in &no_overlap {
        for row in 0..m {
            out[row][col] = matrix[row][cc];
        }
        col += 1
    }

    let overlap = (along - overlap_size..along).zip((along + 1..along + 1 + overlap_size).rev());
    for (bc, ac) in overlap {
        for row in 0..m {
            if matrix[row][bc] == Cell::Full || matrix[row][ac] == Cell::Full {
                out[row][col] = Cell::Full;
            }
        }
        col += 1;
    }
    out
}

#[aoc(day13, part1, d131)]
pub fn part1(input: &(Vec<Vec<Cell>>, Vec<Fold>)) -> u64 {
    let (matrix, folds) = input;
    let res = fold(matrix, folds[0]);
    res.into_iter()
        .flatten()
        .filter(|c| c == &Cell::Full)
        .count() as u64
}

#[aoc(day13, part2, d132)]
pub fn part2(input: &(Vec<Vec<Cell>>, Vec<Fold>)) -> String {
    let (matrix, folds) = input;
    let mut res: Vec<Vec<Cell>> = matrix.to_owned();
    for f in folds {
        res = fold(&res, *f);
    }
    let mut table = get_table(&res);
    table.set_format(*prettytable::format::consts::FORMAT_CLEAN);
    // NOTICE: Uncomment below to print the table to stdout and see the password
    //table.printstd();
    "CJCKBAPB".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_part_1() {
        let input = input_generator(SAMPLE);
        assert_eq!(part1(&input), 17);
    }
}
