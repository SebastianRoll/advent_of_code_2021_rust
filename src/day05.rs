use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone, Hash)]
struct Point(u32, u32);

#[derive(Debug)]
struct LineSegment(Point, Point);

impl LineSegment {
    fn is_horizontal(&self) -> bool {
        self.0 .1 == self.1 .1
    }

    fn is_vertical(&self) -> bool {
        self.0 .0 == self.1 .0
    }

    fn xrange(&self) -> Vec<u32> {
        if self.0 .0 < self.1 .0 {
            (self.0 .0..=self.1 .0).collect()
        } else {
            (self.1 .0..=self.0 .0).rev().collect()
        }
    }

    fn yrange(&self) -> Vec<u32> {
        if self.0 .1 < self.1 .1 {
            (self.0 .1..=self.1 .1).collect()
        } else {
            (self.1 .1..=self.0 .1).rev().collect()
        }
    }

    fn line_points(&self) -> Vec<Point> {
        if self.is_horizontal() {
            (cmp::min(self.0 .0, self.1 .0)..=cmp::max(self.0 .0, self.1 .0))
                .map(|x| Point(x, self.0 .1))
                .collect()
        } else if self.is_vertical() {
            (cmp::min(self.0 .1, self.1 .1)..=cmp::max(self.0 .1, self.1 .1))
                .map(|y| Point(self.0 .0, y))
                .collect()
        } else {
            self.xrange()
                .iter()
                .zip(self.yrange().iter())
                .map(|(x, y)| Point(*x, *y))
                .collect()
        }
    }
}

impl From<String> for LineSegment {
    fn from(s: String) -> Self {
        let mut points = s
            .split(" -> ")
            .map(|pair| {
                let mut iter = pair.split(",");
                let x = iter.next().unwrap().parse::<u32>().unwrap();
                let y = iter.next().unwrap().parse::<u32>().unwrap();
                Point(x, y)
            })
            .collect::<Vec<Point>>();
        let p1 = points.pop().unwrap();
        let p0 = points.pop().unwrap();
        LineSegment(p0, p1)
    }
}

pub fn part(path: &str, is_part1: bool) -> usize {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let segments: Vec<LineSegment> = lines
        .map(|numstr| LineSegment::from(numstr.unwrap()))
        .filter(|seg| !is_part1 || seg.is_horizontal() || seg.is_vertical())
        .collect();

    let mut freqs = HashMap::new();

    segments
        .iter()
        .flat_map(|seg| seg.line_points())
        .for_each(|p| *freqs.entry((p.0, p.1)).or_insert(0) += 1);

    let count = freqs.iter().filter(|(_k, &v)| v >= 2).count();

    count
}
