use std::collections::HashSet;
use std::fs;

use anyhow::Result;

enum Axis {
    X,
    Y,
}

type Point = (usize, usize);
type Fold = (Axis, usize);

pub fn part1(path: &str) -> Result<usize> {
    let contents = fs::read_to_string(path)?;
    let mut iter = contents.lines();
    let points = parse_points(&mut iter)?;
    let folds = parse_folds(&mut iter);
    let first_fold = &folds[0];
    let (mut upper_x, mut upper_y) = (usize::MAX, usize::MAX);
    let new_points = match first_fold {
        (Axis::X, x) => {
            upper_x = *x;
            fold_x(&points, *x)
        }
        (Axis::Y, y) => {
            upper_y = *y;
            fold_y(&points, *y)
        }
    };

    Ok(points
        .union(&new_points)
        .filter(|p| p.0 < upper_x && p.1 < upper_y)
        .count())
}

fn fold_x(points: &HashSet<Point>, x: usize) -> HashSet<Point> {
    points
        .iter()
        .filter(|&p| p.0 > x)
        .map(|&p| (x - (p.0 - x), p.1))
        .collect::<HashSet<Point>>()
}

fn fold_y(points: &HashSet<Point>, y: usize) -> HashSet<Point> {
    points
        .iter()
        .filter(|&p| p.1 > y)
        .map(|&p| (p.0, y - (p.1 - y)))
        .collect::<HashSet<Point>>()
}

pub fn part2(path: &str) -> Result<String> {
    let contents = fs::read_to_string(path)?;
    let mut iter = contents.lines();
    let mut points = parse_points(&mut iter)?;
    let folds = parse_folds(&mut iter);
    let (mut upper_x, mut upper_y) = (usize::MAX, usize::MAX);
    for fold in folds {
        let new_points = match fold {
            (Axis::X, x) => {
                upper_x = x;
                fold_x(&points, x)
            }
            (Axis::Y, y) => {
                upper_y = y;
                fold_y(&points, y)
            }
        };
        points = points
            .union(&new_points)
            .filter(|p| p.0 < upper_x && p.1 < upper_y)
            .cloned()
            .collect::<HashSet<Point>>();
    }
    let code = (0..upper_y)
        .map(|y| {
            (0..upper_x)
                .map(|x| if points.contains(&(x, y)) { '#' } else { '.' })
                .collect::<String>()
                + "\r\n"
        })
        .collect::<String>();
    Ok(code)
}

fn parse_points<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Result<HashSet<Point>> {
    let mut points = HashSet::new();
    while let Some(line) = iter.next() {
        if line.len() == 0 {
            return Ok(points);
        }
        let point = parse_point(line)?;
        points.insert(point);
    }
    Ok(points)
}

fn parse_point(point_str: &str) -> Result<Point> {
    let mut line_iter = point_str.split(",");
    let x = line_iter.next().unwrap();
    let y = line_iter.next().unwrap();
    Ok((x.parse::<usize>()?, y.parse::<usize>()?))
}

fn parse_folds<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Vec<Fold> {
    let mut folds = Vec::new();
    while let Some(line) = iter.next() {
        let expr = line.split_whitespace().last().unwrap();
        // eprintln!("expr = {:?}", expr);
        let mut line_iter = expr.split("=");
        let axis = match line_iter.next().unwrap() {
            "x" => Axis::X,
            "y" => Axis::Y,
            axis => panic!("Invalid axis {}", axis),
        };
        let value = line_iter.next().unwrap().parse::<usize>().unwrap();

        let fold = (axis, value);
        // eprintln!("fold = {:?}", fold);
        folds.push(fold);
    }
    folds
}
