use std::fs::File;
use std::io::{self, BufRead};

pub fn part1() -> usize {
    let file = File::open("data/input10.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    lines
        .map(|line| line.unwrap())
        .filter_map(|comps| corrupt(&comps))
        .map(|corrupt| match corrupt {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!("unknown sign"),
        })
        .sum()
}

pub fn part2() -> usize {
    let file = File::open("data/input10.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut scores = lines
        .map(|line| line.unwrap())
        .filter_map(|comps| incomplete(&comps))
        .map(|incomplete| {
            let mut score = 0;

            for c in incomplete {
                score = score * 5
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!("unknown sign"),
                    };
            }
            score
        })
        .collect::<Vec<usize>>();
    scores.sort();

    let midscore = scores.iter().nth(scores.len() / 2).unwrap();

    *midscore
}

fn corrupt(components: &str) -> Option<char> {
    let mut stack = Vec::new();
    let mut prev;
    for c in components.chars() {
        let match_prev = match c {
            ')' => '(',
            ']' => '[',
            '}' => '{',
            '>' => '<',
            sign => {
                stack.push(sign);
                continue;
            }
        };
        prev = stack.pop().unwrap();
        if prev != match_prev {
            return Some(c);
        }
    }
    None
}

fn incomplete(components: &str) -> Option<Vec<char>> {
    let mut stack = Vec::new();
    let mut prev;
    for c in components.chars() {
        let match_prev = match c {
            ')' => '(',
            ']' => '[',
            '}' => '{',
            '>' => '<',
            sign => {
                stack.push(sign);
                continue;
            }
        };
        prev = stack.pop().unwrap();
        if prev != match_prev {
            return None;
        }
    }
    if stack.len() == 0 {
        return None;
    }

    let completion = stack
        .iter()
        .rev()
        .map(|c| match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => unreachable!("invalid sign: {}", c),
        })
        .collect();

    Some(completion)
}
