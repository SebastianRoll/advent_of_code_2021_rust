use std::fs::File;
use std::io::{self, BufRead};

pub fn part2(path: &str) -> usize {
    let file = File::open(path).unwrap();
    let vals: Vec<usize> = io::BufReader::new(file)
        .lines()
        .map(|s| usize::from_str_radix(&s.unwrap(), 2).unwrap())
        .collect();
    let oxygen = filter_lines(vals.clone(), 1, true, 12)[0];
    let co2 = filter_lines(vals.clone(), 1, false, 12)[0];
    oxygen * co2
}

fn filter_lines(lines: Vec<usize>, idx: usize, oxygen: bool, len: usize) -> Vec<usize> {
    let num_lines = lines.len();
    if num_lines <= 1 {
        return lines;
    }
    let ones: usize = lines
        .iter()
        .filter(|&i| {
            let expr = i & 2_usize.pow((len - idx) as u32) != 0;
            expr
        })
        .count();
    let target: usize;
    if oxygen {
        target = (ones * 2 >= num_lines) as usize;
    } else {
        target = (ones * 2 < num_lines) as usize;
    }
    let flines = lines
        .iter()
        .filter(|&i| (i & 2_usize.pow((len - idx) as u32) == 0) == (target == 0))
        .map(|i| *i)
        .collect();
    filter_lines(flines, idx + 1, oxygen, len)
}

fn main() {
    let path = "input.txt";
    let file = File::open(path).unwrap();
    let mut counter = [0; 12];
    for line in io::BufReader::new(file).lines() {
        line.unwrap()
            .to_string()
            .chars()
            .enumerate()
            .for_each(|(i, c)| match c {
                '1' => counter[i] += 1,
                '0' => counter[i] -= 1,
                _ => panic!("Invalid input"),
            })
    }
    let gamma_rate = counter.iter().rev().enumerate().fold(0, |acc, (i, x)| {
        if *x > 0 {
            acc + 2_usize.pow(i as u32)
        } else {
            acc
        }
    });

    let epsilon_rate = 0b111111111111 ^ gamma_rate;
    println!("{:?}", counter);
    println!("Gamma rate: {}", gamma_rate);
    println!("Epsilon rate: {}", epsilon_rate);
    println!("Answer: {}", gamma_rate * epsilon_rate);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_lines() {
        let lines: Vec<usize> = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .into_iter()
        .map(|s| usize::from_str_radix(s, 2).unwrap())
        .collect();
        println!("{:?}", lines);
        let ox = filter_lines(lines.clone(), 1, true, 5);
        assert_eq!(ox, [23]);
        let co2 = filter_lines(lines.clone(), 1, false, 5);
        assert_eq!(co2, [10])
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 5852595)
    }
}
