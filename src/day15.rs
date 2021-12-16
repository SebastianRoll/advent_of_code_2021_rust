/*
Djikstras shortest past algorithm

create visited set
each node gets a shortest distance=+inf, start node gets 0
visit all visited adjacent nodes and update total distance (start->node) if smaller than existing
mark current node as visited
stop when finish node is visited
*/

use std::cmp;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::ops::Index;

#[derive(Eq, Hash, Ord)]
struct Node {
    point: Point,
    distance: usize,
}
impl Node {
    fn new(point: Point) -> Self {
        Self {
            point,
            distance: usize::MAX,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

struct Cavern {
    board: Vec<Vec<u8>>,
    y_max: usize,
    x_max: usize,
}

impl Cavern {
    fn new(board: Vec<Vec<u8>>) -> Self {
        let y_max = board.len() - 1;
        let x_max = board[0].len() - 1;
        Self {
            board,
            y_max,
            x_max,
        }
    }

    fn get(&self, point: Point) -> u8 {
        let base_y = point.0 % (self.y_max + 1);
        let base_x = point.1 % (self.x_max + 1);
        let risk_added = (point.0 / (self.y_max + 1)) + (point.1 / (self.x_max + 1));
        1u8 + ((usize::from(self.board[base_y][base_x]) - 1 + risk_added) % 9) as u8
    }

    fn width(&self, multiplier: usize) -> usize {
        (self.x_max + 1) * multiplier
    }

    fn height(&self, multiplier: usize) -> usize {
        (self.y_max + 1) * multiplier
    }
}

/*
// TODO: Figure out how I could get indexing to work
impl Index<Point> for Cavern {
    type Output = u8;

    fn index(&self, point: Point) -> &Self::Output {
        let base_y = point.0 % self.y_max;
        let base_x = point.1 % self.x_max;
        let risk_added = (point.0 / self.y_max) + (point.1 / self.x_max) - 2;
        &((usize::from(self.board[base_y][base_x]) + risk_added) as u8 % 9)
    }
}
*/

type Point = (usize, usize);

pub fn part(path: &str, multiplier: usize) -> usize {
    let contents = fs::read_to_string(path).unwrap();

    let mut minheap = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut matrix = Vec::new();
    let mut dists = HashMap::new();
    for (y, line) in contents.lines().enumerate() {
        let row = line
            .chars()
            .enumerate()
            .map(|(x, c)| {
                let point = (y, x);
                c as u8 - 48
            })
            .collect::<Vec<u8>>();
        matrix.push(row);
    }
    let cavern = Cavern::new(matrix);
    let mut current = (0, 0);
    dists.insert(current, 0); // starting point
    minheap.push(Reverse(Node::new(current)));

    let endpoint = (cavern.width(multiplier) - 1, cavern.height(multiplier) - 1);

    while let Some(Reverse(Node { point: current, .. })) = minheap.pop() {
        if visited.contains(&current) {
            continue;
        } else {
            visited.insert(current);
        }

        let mut neighbors = Vec::new();
        if current.0 >= 1 {
            neighbors.push((current.0 - 1, current.1));
        }
        if current.0 + 1 <= cavern.height(multiplier) - 1 {
            neighbors.push((current.0 + 1, current.1));
        }
        if current.1 >= 1 {
            neighbors.push((current.0, current.1 - 1));
        }
        if current.1 + 1 <= cavern.width(multiplier) - 1 {
            neighbors.push((current.0, current.1 + 1));
        }
        for target in neighbors.iter().filter(|p| !visited.contains(p)) {
            let new_dist = cmp::min(
                *dists.entry(*target).or_insert(usize::MAX),
                dists[&current] + usize::from(cavern.get(*target)),
            );
            dists.insert(*target, new_dist).unwrap();
            minheap.push(Reverse(Node {
                point: *target,
                distance: new_dist,
            }));
        }
    }
    dists[&endpoint]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cavern() {
        let contents = fs::read_to_string("data/input15_example.txt").unwrap();

        let mut matrix = Vec::new();
        for (y, line) in contents.lines().enumerate() {
            let row = line
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    let point = (y, x);
                    c as u8 - 48
                })
                .collect::<Vec<u8>>();
            matrix.push(row);
        }
        let cavern = Cavern::new(matrix);
        let base = 10;
        let asserts = [
            ((0, 0), 1),
            ((base, 0), 2),
            ((base + 1, 0), 2),
            ((base + 2, 0), 3),
            ((2 * base, 0), 3),
            ((2 * base + 2, 0), 4),
            ((7 * base + 2, 0), 9),
            ((8, 0), 1),
            ((9, 0), 2),
            ((0, base), 2),
            ((0, base + 2), 7),
            ((0, 2 * base + 2), 8),
            ((2 * base, 2 * base), 5),
            ((4 * base, 4 * base), 9),
            ((5 * base, 5 * base), 2),
        ];
        for (point, exp) in asserts.iter() {
            let res = usize::from(cavern.get(*point));
            assert_eq!(*exp, res);
        }
    }
}
