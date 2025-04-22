// Puzzle description: https://adventofcode.com/2024/day/13

use regex::Regex;
use std::ops::{Add, Div, Mul, Sub};

/// 2D point in homogeneous coordinates
/// Represents the point (x/w, y/w)
#[derive(Clone, Copy, Debug)]
struct Point {
    x: i128,
    y: i128,
    w: i128,
}

impl Point {
    const ZERO: Point = Point { x: 0, y: 0, w: 1 };
    fn from_xy(x: i128, y: i128) -> Point {
        Point { x, y, w: 1 }
    }
    fn mul_w(&self, w: i128) -> Point {
        Point {
            x: self.x * w,
            y: self.y * w,
            w: self.w * w,
        }
    }
    fn is_int(&self) -> bool {
        self.w != 0 && self.x % self.w == 0 && self.y % self.w == 0
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        // Convert points to a common w
        let a = self.mul_w(rhs.w);
        let b = rhs.mul_w(self.w);
        Point {
            x: a.x + b.x,
            y: a.y + b.y,
            w: a.w,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        // Convert points to a common w
        let a = self.mul_w(rhs.w);
        let b = rhs.mul_w(self.w);
        Point {
            x: a.x - b.x,
            y: a.y - b.y,
            w: a.w,
        }
    }
}

impl Mul<i128> for Point {
    type Output = Self;
    fn mul(self, rhs: i128) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
            w: self.w,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        if self.w != 0 && other.w != 0 {
            let a = self.mul_w(other.w);
            let b = other.mul_w(self.w);
            a.x == b.x && a.y == b.y
        } else {
            self.x == other.x && self.y == other.y && self.w == other.w
        }
    }
}

impl Div for Point {
    type Output = Option<i128>;
    fn div(self, rhs: Self) -> Self::Output {
        let k = if self.w == 0 {
            None
        } else if self.x != 0 && rhs.x != 0 {
            Some((self.x * rhs.w) / (rhs.x * self.w))
        } else if self.y != 0 && rhs.y != 0 {
            Some((self.y * rhs.w) / (rhs.y * self.w))
        } else {
            None
        };
        k.filter(|&k| rhs * k == self)
    }
}

/// Represents the line equation ax + by + c = 0
struct Line {
    a: i128,
    b: i128,
    c: i128,
}

impl Line {
    /// https://en.wikipedia.org/wiki/Linear_equation#Determinant_form
    /// Modified for homogeneous coordinates
    fn from_points(p1: &Point, p2: &Point) -> Line {
        Line {
            a: p1.y * p2.w - p2.y * p1.w,
            b: p2.x * p1.w - p1.x * p2.w,
            // Numerically dangerous, because p1.x * p2.y - p2.x * p1.y can overflow.
            // (In this case p1.w and p2.w are both 1)
            // A possible fix would be dividing everything with some common denominator perhaps.
            c: p1.w * p2.w * (p1.x * p2.y - p2.x * p1.y),
        }
    }
}

/// Intersection of two lines. If Point.w == 0, lines do not intersect.
/// https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Using_homogeneous_coordinates
fn intersection(l1: &Line, l2: &Line) -> Point {
    Point {
        x: l1.b * l2.c - l2.b * l1.c,
        y: l2.a * l1.c - l1.a * l2.c,
        w: l1.a * l2.b - l2.a * l1.b,
    }
}

/// The steps we can take by each lever conceptually form a line.
/// Drawing one line from the origin, and another from the target,
/// we find their intersection, and check if it is reachable in
/// an integer number of steps.
fn find_steps(target: &Point, step_a: &Point, step_b: &Point) -> Option<(i128, i128)> {
    let line_a = Line::from_points(&Point::ZERO, step_a);
    let line_b = Line::from_points(target, &(*target + *step_b));
    let i = intersection(&line_a, &line_b);
    // The intersection itself lies on integer coordinates, but that does not
    // guarantee it (and the target) can be reached in integer steps.
    if i.is_int() {
        // Normalize intersection point
        let i = Point {
            x: i.x / i.w,
            y: i.y / i.w,
            w: 1,
        };
        let na = i / *step_a;
        let nb = (*target - i) / *step_b;
        na.zip(nb)
    } else {
        None
    }
}

#[derive(Debug)]
struct Machine {
    a: Point,
    b: Point,
    prize: Point,
}

impl Machine {
    const COST_A: i128 = 3;
    const COST_B: i128 = 1;

    fn cheapest(&self) -> Option<i128> {
        let steps_ab = find_steps(&self.prize, &self.a, &self.b);
        let steps_ba = find_steps(&self.prize, &self.b, &self.a);
        let cost_ab = steps_ab.map(|(na, nb)| na * Self::COST_A + nb * Self::COST_B);
        let cost_ba = steps_ba.map(|(nb, na)| na * Self::COST_A + nb * Self::COST_B);
        if let (Some(ab), Some(ba)) = (cost_ab, cost_ba) {
            Some(ab.min(ba))
        } else {
            cost_ab.or(cost_ba)
        }
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    re.captures_iter(input)
        .map(|c| {
            let ax: i128 = c[1].parse().unwrap();
            let ay: i128 = c[2].parse().unwrap();
            let bx: i128 = c[3].parse().unwrap();
            let by: i128 = c[4].parse().unwrap();
            let px: i128 = c[5].parse().unwrap();
            let py: i128 = c[6].parse().unwrap();
            Machine {
                a: Point::from_xy(ax, ay),
                b: Point::from_xy(bx, by),
                prize: Point::from_xy(px, py),
            }
        })
        .collect()
}

const INPUT: &str = "data/y2024/d13/input";

pub fn part1() -> i128 {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let machines = parse_input(&input);
    machines.into_iter().filter_map(|m| m.cheapest()).sum()
}

pub fn part2() -> i128 {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let machines = parse_input(&input).into_iter().map(|m| Machine {
        prize: m.prize + Point::from_xy(10000000000000, 10000000000000),
        ..m
    });
    machines.into_iter().filter_map(|m| m.cheapest()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 33921);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(), 82261957837868);
    }
}
