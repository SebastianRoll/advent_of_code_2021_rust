use std::collections::HashMap;
use std::fs;

pub fn part1(path: &str) -> usize {
    solve(path, 10)
}

pub fn part2(path: &str) -> usize {
    solve(path, 40)
}

fn solve(path: &str, max_step: usize) -> usize {
    let contents = fs::read_to_string(path).unwrap();

    let current = contents.lines().nth(0).unwrap().to_owned();

    let mut pair_rules = HashMap::new();
    for line in contents.lines().skip(2) {
        let mut iter = line.split(" -> ");
        pair_rules.insert(
            iter.next().unwrap(),
            iter.next().unwrap().chars().nth(0).unwrap(),
        );
    }

    // First we find the 20-step-forward frequencies for every mapping key
    let mut forward_twenty = HashMap::new();
    pair_rules.keys().for_each(|key| {
        let mut freqs_forward = HashMap::new();
        let mut iter = key.chars();
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        iteration(a, b, &pair_rules, 0, max_step / 2, &mut freqs_forward, None);
        forward_twenty.insert(*key, freqs_forward);
    });

    // Then we find 20-step-forward frequencies for the char pairs in our codeword (top of the input file)
    let mut freqs = HashMap::new();
    current.as_bytes().windows(2).for_each(|w| {
        iteration(
            w[0] as char,
            w[1] as char,
            &pair_rules,
            0,
            max_step / 2,
            &mut freqs,
            Some(&forward_twenty),
        );
    });

    // Add the original characters
    for c in current.chars() {
        *freqs.entry(c).or_insert(0) += 1;
    }

    // Final calculation: returns highest char count minus lowest char count
    let mut counts: Vec<usize> = freqs.values().cloned().collect();
    counts.sort();
    counts[counts.len() - 1] - counts[0]
}

fn iteration(
    a: char,
    b: char,
    pair_rules: &HashMap<&str, char>,
    step: usize,
    max_step: usize,
    freqs: &mut HashMap<char, usize>,
    forward_twenty: Option<&HashMap<&str, HashMap<char, usize>>>,
) {
    let comb = format!("{}{}", a, b);

    if step >= max_step {
        if let Some(for_twenty) = forward_twenty {
            let ft = &for_twenty[&comb.as_str()];
            for (c, count) in ft.iter() {
                *freqs.entry(*c).or_insert(0) += count;
            }
        }
        return;
    }

    let mid = pair_rules.get(&comb.as_str()).unwrap();
    *freqs.entry(*mid).or_insert(0) += 1;

    iteration(
        a,
        *mid,
        pair_rules,
        step + 1,
        max_step,
        freqs,
        forward_twenty,
    );
    iteration(
        *mid,
        b,
        pair_rules,
        step + 1,
        max_step,
        freqs,
        forward_twenty,
    );
}
