use std::collections::{HashMap, HashSet};
use std::fs;

pub fn part1(path: &str) -> usize {
    let mut cave_mapping = HashMap::new();
    let lines = fs::read_to_string(path).unwrap();

    lines.lines().for_each(|line| {
        let mut iter = line.split("-");
        let from = iter.next().unwrap();
        let to = iter.next().unwrap();
        cave_mapping.entry(from).or_insert(Vec::new()).push(to);
        cave_mapping.entry(to).or_insert(Vec::new()).push(from);
    });

    let visited = HashSet::new();
    let complete_paths = search(&cave_mapping, "start", &visited, true);
    complete_paths
}

pub fn part2(path: &str) -> usize {
    let mut cave_mapping = HashMap::new();
    let lines = fs::read_to_string(path).unwrap();

    lines.lines().for_each(|line| {
        let mut iter = line.split("-");
        let from = iter.next().unwrap();
        let to = iter.next().unwrap();
        cave_mapping.entry(from).or_insert(Vec::new()).push(to);
        cave_mapping.entry(to).or_insert(Vec::new()).push(from);
    });

    let visited = HashSet::new();
    let complete_paths = search(&cave_mapping, "start", &visited, false);
    complete_paths
}

fn search<'a>(
    cave_mapping: &HashMap<&'a str, Vec<&str>>,
    cavename: &'a str,
    visited: &HashSet<&'a str>,
    passed_double: bool,
) -> usize {
    let mut visited = visited.clone();

    if cavename.to_lowercase() == cavename {
        visited.insert(cavename);
    }
    let mut complete_paths = 0;
    for dest_cave in cave_mapping.get(cavename).unwrap() {
        match (*dest_cave, visited.contains(dest_cave), passed_double) {
            ("end", _, _) => complete_paths += 1,
            ("start", _, _) => continue,
            (_, true, true) => continue,
            (cave, true, false) => complete_paths += search(cave_mapping, cave, &visited, true),
            (cave, _, passed_double) => {
                complete_paths += search(cave_mapping, cave, &visited, passed_double)
            }
        }
    }
    complete_paths
}
