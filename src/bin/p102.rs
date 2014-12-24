#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

extern crate common;

use std::io::{BufferedReader, File, IoResult};
use std::num::SignedInt;
use common::Solver;

#[deriving(Copy)]
struct Point(int, int);

#[deriving(Copy)]
struct Line(Point, Point);

#[deriving(Copy)]
struct Triangle(Point, Point, Point);

enum Side { L, R, C }

impl Sub<Point, Point> for Point {
    fn sub(self, other: Point) -> Point {
        Point(self.0 - other.0, self.1 - other.1)
    }
}

impl Mul<Point, int> for Point {
    fn mul(self, other: Point) -> int {
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
            1  => Side::L,
            0  => Side::C,
            -1 => Side::R,
            _  => panic!()
        }
    }
}

impl Triangle {
    fn contains(self, p: Point) -> bool {
        use Side::{L, R, C};
        let s0 = Line(self.0, self.1).side(p);
        let s1 = Line(self.1, self.2).side(p);
        let s2 = Line(self.2, self.0).side(p);
        match (s0, s1, s2) {
            (L, L, L) | (L, L, C) | (L, C, L) | (L, C, C)
                | (C, L, L) | (C, L, C) | (C, C, L) | (C, C, C)
                | (R, R, R) | (R, R, C) | (R, C, R) | (R, C, C)
                | (C, R, R) | (C, R, C) | (C, C, R) => true,
            _ => false
        }
    }
}

fn solve(file: File) -> IoResult<String> {
    let mut br = BufferedReader::new(file);

    let origin = Point(0,0);

    let mut cnt = 0u;
    for line in br.lines() {
        let ns = try!(line)
            .trim()
            .split(',')
            .filter_map(StrExt::parse::<int>)
            .collect::<Vec<_>>();
        let t = Triangle(Point(ns[0], ns[1]), Point(ns[2], ns[3]), Point(ns[4], ns[5]));
        if t.contains(origin) { cnt += 1 }
    }

    Ok(cnt.to_string())
}

fn main() {
    Solver::new_with_file("228", "p102_triangles.txt", solve).run();
}

#[cfg(test)]
mod test {
    use super::{Triangle, Point};

    #[test]
    fn contains() {
        let abc = Triangle(Point(-340, 459), Point(-153, -910), Point(835, -947));
        let xyz = Triangle(Point(-175,  41), Point(-421, -714), Point(574, -645));
        let origin = Point(0, 0);
        assert!(abc.contains(origin));
        assert!(!xyz.contains(origin));
    }
}
