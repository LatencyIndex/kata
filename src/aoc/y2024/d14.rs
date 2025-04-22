use crate::lgl::data::index::Ix2s;
use image::{GrayImage, Luma};
use regex::Regex;
use std::collections::HashMap;

fn vmod(p: Ix2s, modulo: Ix2s) -> Ix2s {
    Ix2s(p.0.rem_euclid(modulo.0), p.1.rem_euclid(modulo.1))
}

#[derive(Debug)]
struct Robot {
    pos: Ix2s,
    vel: Ix2s,
}

impl Robot {
    fn tick(&self, dt: isize, area: Ix2s) -> Self {
        Robot {
            pos: vmod(self.pos + self.vel * dt, area),
            vel: self.vel,
        }
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    re.captures_iter(input)
        .map(|c| Robot {
            pos: Ix2s(c[1].parse().unwrap(), c[2].parse().unwrap()),
            vel: Ix2s(c[3].parse().unwrap(), c[4].parse().unwrap()),
        })
        .collect()
}

fn make_map(robots: &[Robot]) -> HashMap<Ix2s, usize> {
    let mut map = HashMap::new();
    for pos in robots.iter().map(|r| r.pos) {
        *map.entry(pos).or_insert(0) += 1;
    }
    map
}

fn get_quadrant_1d(i: isize, n: isize) -> Option<usize> {
    use std::cmp::Ordering;
    let mid = n / 2;
    match i.cmp(&mid) {
        Ordering::Less => Some(0),
        Ordering::Equal => None,
        Ordering::Greater => Some(1),
    }
}

fn get_quadrant_2d(pos: Ix2s, area: Ix2s) -> Option<usize> {
    let q0 = get_quadrant_1d(pos.0, area.0);
    let q1 = get_quadrant_1d(pos.1, area.1);
    q0.and_then(|q0| q1.map(|q1| q0 + 2 * q1))
}

fn to_img(map: &HashMap<Ix2s, usize>, area: Ix2s, dst: &str) {
    let width = area.0 as u32;
    let height = area.1 as u32;
    let mut img = GrayImage::new(width, height);
    for (pos, count) in map {
        if 0 < *count {
            img.put_pixel(pos.0 as u32, pos.1 as u32, Luma([255]));
        }
    }
    img.save(dst).unwrap();
}

const INPUT: &str = "data/y2024/d14/input";

pub fn part1() -> usize {
    let input = std::fs::read_to_string(INPUT).unwrap();
    let robots = parse_input(&input);
    let area = Ix2s(101, 103);
    let dt = 100;
    let robots: Vec<_> = robots.iter().map(|r| r.tick(dt, area)).collect();
    let robot_map = make_map(&robots);
    let mut qcounts = [0usize; 4];
    for (pos, count) in robot_map.iter() {
        if let Some(q) = get_quadrant_2d(*pos, area) {
            qcounts[q] += count;
        }
    }
    let safety_factor = qcounts.iter().product();
    safety_factor
}

pub fn part2() {
    // Generates the image you're looking for.
    // To find it originally, just generate 10k images and look through them manually.
    let input = std::fs::read_to_string(INPUT).unwrap();
    let robots = parse_input(&input);
    let area = Ix2s(101, 103);
    let dt = 7037;
    let robots: Vec<_> = robots.iter().map(|r| r.tick(dt, area)).collect();
    let robot_map = make_map(&robots);
    to_img(&robot_map, area, format!("{dt}.png").as_str());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 218965032);
    }
}
