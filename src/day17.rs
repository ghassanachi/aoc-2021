/// A lot of code was added (and not removed) since I assumed target  could be in the negative x
/// direction.  I removed some of the code but left the rest
use aoc_runner_derive::{aoc, aoc_generator};
use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct Target {
    near_x: i64,
    far_x: i64,
    bottom_y: i64,
    x_range: RangeInclusive<i64>,
    y_range: RangeInclusive<i64>,
}

#[derive(Default)]
struct Vector {
    x: i64,
    y: i64,
}

#[derive(Default)]
pub struct Probe {
    position: Vector,
    velocity: Vector,
}

impl Target {
    // Assumption here, y bottom is negative  (since my input has it that way)
    fn vel_y_range(&self) -> Box<dyn Iterator<Item = i64>> {
        if self.bottom_y.is_negative() {
            Box::new(self.bottom_y..self.bottom_y.abs())
        } else {
            Box::new(quadratic(self.bottom_y).1..self.bottom_y.abs())
        }
    }

    fn vel_x_range(&self) -> Box<dyn Iterator<Item = i64>> {
        if self.near_x.is_negative() {
            Box::new(self.far_x..=-quadratic(self.near_x).1)
        } else {
            Box::new(quadratic(self.near_x).1..=self.far_x)
        }
    }
}

impl Probe {
    fn new(velocity: Vector) -> Self {
        Self {
            velocity,
            ..Default::default()
        }
    }

    // assu
    fn update(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        if self.velocity.x != 0 {
            self.velocity.x += if self.velocity.x > 0 { -1 } else { 1 }
        }
        self.velocity.y -= 1;
    }

    fn in_target(&self, target: &Target) -> bool {
        target.x_range.contains(&self.position.x) && target.y_range.contains(&self.position.y)
    }

    fn missed_target(&self, target: &Target) -> bool {
        if self.position.x.abs() < target.near_x.abs() && self.velocity.x == 0 {
            return true;
        }
        if self.position.x.abs() > target.far_x.abs()
            && self.position.x.is_negative() == target.far_x.is_negative()
        {
            return true;
        }
        if self.velocity.y.is_negative() && self.position.y < target.bottom_y {
            return true;
        }
        false
    }
}

// Solve 1 + 2 + .. n = num with quadratic on (n(n + 1)) / 2 = num
// Discard negative velocity
fn quadratic(num: i64) -> (i64, i64) {
    let num = num as f64;
    let a = (-1.0 + (1.0 + 4.0 * num).sqrt()) / 2.0;
    let b = (-1.0 - (1.0 + 4.0 * num).sqrt()) / 2.0;
    let (a, b) = (a as i64, b as i64);
    (a.min(b), a.max(b))
}

fn to_tuple(input: &str) -> (i64, i64) {
    let input = &input[2..];
    let mut parts = input.split("..").map(|n| n.parse::<i64>().unwrap());
    let (a, b) = (parts.next().unwrap(), parts.next().unwrap());
    (a.min(b), b.max(a))
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Target {
    let input = input.replace("target area: ", "");
    let mut parts = input.split(", ");
    let (x1, x2) = to_tuple(parts.next().unwrap());
    let (y1, y2) = to_tuple(parts.next().unwrap());

    let (near_x, far_x) = if x1.is_negative() {
        (x1.max(x2), x1.min(x2))
    } else {
        (x1.min(x2), x1.max(x2))
    };
    let (bottom_y, top_y) = (y1.min(y2), y1.max(y2));

    Target {
        near_x,
        far_x,
        bottom_y,
        x_range: near_x.min(far_x)..=near_x.max(far_x),
        y_range: bottom_y..=top_y,
    }
}

#[aoc(day17, part1, day17_1)]
pub fn part1(target: &Target) -> u64 {
    (target.bottom_y * (target.bottom_y + 1) / 2) as u64
}

#[aoc(day17, part2, day17_2)]
pub fn part2(target: &Target) -> u64 {
    let mut count = 0;
    let possible_ys: Vec<_> = target.vel_y_range().collect();
    for x in target.vel_x_range() {
        for &y in &possible_ys {
            let mut probe = Probe::new(Vector { x, y });
            loop {
                if probe.in_target(target) {
                    count += 1;
                    break;
                }
                if probe.missed_target(target) {
                    break;
                }
                probe.update();
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_part_1() {
        let input = input_generator(SAMPLE);
        assert_eq!(part1(&input), 45)
    }

    #[test]
    fn test_part_2() {
        let input = input_generator(SAMPLE);
        assert_eq!(part2(&input), 112)
    }
}
