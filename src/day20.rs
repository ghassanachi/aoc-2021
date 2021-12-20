use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
pub struct Image {
    img: Vec<Vec<bool>>,
    lookup: [bool; 512],
    world_alight: bool,
}

impl Image {
    fn new() -> Self {
        Self {
            img: Vec::new(),
            lookup: [false; 512],
            world_alight: false,
        }
    }

    fn get(&self, row: i64, col: i64) -> bool {
        if row < 0 || col < 0 || row as usize >= self.img.len() || col as usize >= self.img[0].len()
        {
            return self.world_alight;
        }
        let (row, col) = (row as usize, col as usize);
        self.img[row][col]
    }

    fn enhance_pixel(&self, window: [bool; 9]) -> bool {
        let mut idx = 0;
        for bit_set in window {
            idx <<= 1;
            if bit_set {
                idx |= 1;
            }
        }
        self.lookup[idx]
    }

    fn enhance(&mut self) {
        let (m, n) = (self.img.len() as i64, self.img[0].len() as i64);
        let mut new_image = Vec::new();
        let mut window = [false; 9];

        for row in -2..m {
            let mut new_row = Vec::new();
            for col in -2..n {
                for r in 0..3 {
                    for c in 0..3 {
                        let idx = (r * 3 + c) as usize;
                        window[idx] = self.get(row + r, col + c);
                    }
                }
                new_row.push(self.enhance_pixel(window));
            }
            new_image.push(new_row);
        }
        self.img = new_image;
        if self.lookup[0] && !self.lookup[511] {
            self.world_alight = !self.world_alight;
        }
    }

    fn run_enhancement(&mut self, rounds: usize) {
        for _ in 0..rounds {
            self.enhance();
        }
    }

    fn count_lit(&self) -> u64 {
        self.img
            .iter()
            .map(|n| n.iter().filter(|&n| *n).count())
            .sum::<usize>() as u64
    }

    #[allow(dead_code)]
    fn print(&self) {
        let mut out = String::new();
        for row in 0..self.img.len() {
            for col in 0..self.img[0].len() {
                out.push(if self.img[row][col] { '#' } else { '.' });
            }
            out.push('\n');
        }
        println!("{}", out);
    }
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Image {
    let mut image = Image::new();
    let mut iter = input.lines();
    for (i, c) in iter.next().unwrap().chars().enumerate() {
        if c == '#' {
            image.lookup[i] = true;
        }
    }
    iter.next();
    while let Some(line) = iter.next() {
        let row: Vec<_> = line.chars().map(|n| n == '#').collect();
        image.img.push(row);
    }
    image
}

#[aoc(day20, part1, day20_1)]
pub fn part1(input: &Image) -> u64 {
    let mut image = input.clone();
    image.run_enhancement(2);
    image.count_lit()
}

#[aoc(day20, part2, day20_2)]
pub fn part2(input: &Image) -> u64 {
    let mut image = input.clone();
    image.run_enhancement(50);
    image.count_lit()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str =
        "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###\
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.\
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....\
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..\
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....\
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn test_part_1() {
        let input = input_generator(SAMPLE);
        assert_eq!(part1(&input), 35)
    }

    #[test]
    fn test_part_2() {
        let input = input_generator(SAMPLE);
        assert_eq!(part2(&input), 3351)
    }
}
