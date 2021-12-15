/*
Djikstras shortest past algorithm

create unvisited set
each node gets a shortest distance=+inf, start node gets 0
visit all unvisited adjacent nodes and update total distance (start->node) if smaller than existing
mark current node as visited
stop when finish node is visited
*/

use std::collections::{HashMap, HashSet};
use std::fs;
use std::cmp;

/*
#[derive(Eq, Hash)]
struct Node {
    point: Point,
    distance: usize
}
impl Node {
    fn new(point: Point) -> Self {
        Self{
            point,
            distance: usize::MAX
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &self) -> Option(cmp::Ordering) {
        self.distance.partial_cmp(&other.distance)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}
*/
type Point = (usize, usize);

pub fn part1(path: &str) -> usize {
    let contents = fs::read_to_string(path).unwrap();

    let mut unvisited = HashSet::new();
    let mut matrix = Vec::new();
    let mut dists = HashMap::new();
    for (y, line) in contents.lines().enumerate() {
        let row = line.chars().enumerate()
            .map(|(x, c)| {
                let point = (y, x);
                unvisited.insert(point);
                dists.insert(point, usize::MAX);
                c as u8 - 48
            })
            .collect::<Vec<u8>>();
        matrix.push(row);
    }
    let mut current = (0,0);
    dists.insert(current, 0).unwrap(); // starting point

    let y_end = matrix.len()-1;
    let x_end = matrix[0].len()-1;

    let endpoint = (y_end, x_end);
    eprintln!("endpoint = {:?}", endpoint);


    // part 2 stuff
    let mut end_dist: usize = (0..=y_end).zip(0..x_end)
        .map(|(y, x)| usize::from(matrix[y][x]))
        .sum();
    end_dist += (1..y_end).zip(0..x_end)
        .map(|(y, x)| usize::from(matrix[y][x]))
        .sum::<usize>();
    eprintln!("end_dist = {:?}", end_dist);
    dists.insert(endpoint, end_dist);
    // part 2 stuff END

    let mut counter = 0;
    loop {
        if counter % 100 == 0 {
            eprintln!("current = {:?}: {} - {}", current, dists[&current], counter);
        }
        counter += 1;

        if dists[&current] > dists[&endpoint] {
            eprintln!("REMOVING current = {:?}. {} {}", current, dists[&current], dists[&endpoint]);
            unvisited.remove(&current);
            continue
        }
        
        let mut neighbors = Vec::new();
        if current.0 >= 1 {
            neighbors.push((current.0-1, current.1));
        }
        if current.0+1 <= y_end {
            neighbors.push((current.0+1, current.1));
        }
        if current.1 >= 1 {
            neighbors.push((current.0, current.1-1));
        }
        if current.1+1  <= x_end {
            neighbors.push((current.0, current.1+1));
        }
        for target in neighbors.iter()
        .filter(|p| unvisited.contains(p)) {
            // eprintln!("target = {:?}", target);
            let new_dist = cmp::min(dists[&target], dists[&current] + usize::from(matrix[target.0][target.1]));
            // eprintln!("{}, {}, {}", dists[&target], dists[&current], usize::from(matrix[target.0][target.1]));
            // eprintln!("new_dist = {:?}", new_dist);
            dists.insert(*target, new_dist).unwrap();
            
        }
        unvisited.remove(&current);
        // eprintln!("unvisited.len() = {:?}", unvisited.len());

        if !unvisited.contains(&endpoint) {
            break
        }
        {
            let (curr, _dist) = dists.iter()
                .filter(|&(point, _)| unvisited.contains(point))
                .min_by_key(|&(_point, dist)| dist) //: &(Point, usize)| dist)
                .unwrap();
            current = *curr;
        }
    }
    dists[&endpoint]
}

/*
fn shortest_path(matrix: &Vec<Vec<u8>>, current: Point, end: Point, unvisited: &mut HashSet<Point>, dists: &mut HashMap<Point, usize>) -> usize {
    if !unvisited.contains(&end) {
        dists[end]
    }
    for target in [
        (cmp::max(0, current.0-1), current.1),
        (cmp::min(matrix[0].len(), current.0+1), current.1),
        (current.0, cmp::max(0, current.1-1)),
        (current.0, cmp::min(matrix.len(), current.1+1)),
    ] {
        dists[target] = cmp::min(dists[target], dists[current] + matrix[target.y][target.x])
    }
    unvisited.remove(current);

}
*/