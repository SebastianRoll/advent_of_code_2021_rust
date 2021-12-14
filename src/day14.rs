use std::collections::HashMap;
use std::fs;

pub fn part1(path: &str) -> usize {
    solve(path, 10)
}

pub fn part2(path: &str) -> usize {
    solve(path, 40)
}

fn solve(path: &str, iteration_steps: usize) -> usize {
    let contents = fs::read_to_string(path).unwrap();

    let mut current = contents.lines().nth(0).unwrap().to_owned();

    let mut pair_rules = HashMap::new();
    for line in contents.lines().skip(2) {
        let mut iter = line.split(" -> ");
        pair_rules.insert(iter.next().unwrap(), iter.next().unwrap());
    }

    for step in 1..=iteration_steps {
        eprintln!("step = {:?}", step);
        current = iteration(&current[..], &pair_rules);
    }

    let mut freqs = HashMap::new();
    current
        .chars()
        .for_each(|c| *freqs.entry(c).or_insert(0) += 1);
    let mut counts: Vec<usize> = freqs.values().cloned().collect();
    counts.sort();
    counts[counts.len() - 1] - counts[0]
}

fn iteration(current: &str, pair_rules: &HashMap<&str, &str>) -> String {
    let new = current
        .as_bytes()
        .windows(2)
        .map(|w| {
            let s = format!("{}{}", w[0] as char, w[1] as char); //[w[0] as char, w[1] as char].iter().collect::<String>();

            let mid = pair_rules.get(&s.as_str()).unwrap();
            format!("{}{}", w[0] as char, mid)
        })
        .collect::<String>();
    let last_char: char = current.chars().rev().nth(0).unwrap();
    format!("{}{}", new, last_char)
}
