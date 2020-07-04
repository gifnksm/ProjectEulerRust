//! [Problem 102](https://projecteuler.net/problem=102) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
    ops::{Mul, Sub},
};

#[derive(Copy, Clone)]
struct Point(i32, i32);

#[derive(Copy, Clone)]
struct Line(Point, Point);

#[derive(Copy, Clone)]
struct Triangle(Point, Point, Point);

enum Side {
    L,
    R,
    C,
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point(self.0 - other.0, self.1 - other.1)
    }
}

impl Mul<Point> for Point {
    type Output = i32;

    fn mul(self, other: Point) -> i32 {
        self.0 * other.0 + self.1 * other.1
    }
}

impl Point {
    fn normal(self) -> Point {
        Point(-self.1, self.0)
    }
}

impl Line {
    fn side(self, p: Point) -> Side {
        match ((p - self.0) * (self.1 - self.0).normal()).signum() {
            1 => Side::L,
            0 => Side::C,
            -1 => Side::R,
            _ => panic!(),
        }
    }
}

impl Triangle {
    fn contains(self, p: Point) -> bool {
        use crate::Side::{C, L, R};
        let s0 = Line(self.0, self.1).side(p);
        let s1 = Line(self.1, self.2).side(p);
        let s2 = Line(self.2, self.0).side(p);
        match (s0, s1, s2) {
            (L, L, L)
            | (L, L, C)
            | (L, C, L)
            | (L, C, C)
            | (C, L, L)
            | (C, L, C)
            | (C, C, L)
            | (C, C, C)
            | (R, R, R)
            | (R, R, C)
            | (R, C, R)
            | (R, C, C)
            | (C, R, R)
            | (C, R, C)
            | (C, C, R) => true,
            _ => false,
        }
    }
}

fn solve(file: File) -> io::Result<String> {
    let origin = Point(0, 0);

    let mut cnt = 0;
    for line in BufReader::new(file).lines() {
        let ns = line?
            .trim()
            .split(',')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<_>>();
        let t = Triangle(
            Point(ns[0], ns[1]),
            Point(ns[2], ns[3]),
            Point(ns[4], ns[5]),
        );
        if t.contains(origin) {
            cnt += 1
        }
    }

    Ok(cnt.to_string())
}

common::problem!("228", "p102_triangles.txt", solve);

#[cfg(test)]
mod test {
    use super::{Point, Triangle};

    #[test]
    fn contains() {
        let abc = Triangle(Point(-340, 459), Point(-153, -910), Point(835, -947));
        let xyz = Triangle(Point(-175, 41), Point(-421, -714), Point(574, -645));
        let origin = Point(0, 0);
        assert!(abc.contains(origin));
        assert!(!xyz.contains(origin));
    }
}
