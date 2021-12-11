use std::fs::File;
use std::io::{self, BufRead};

// returns the number of times values in collection increases from the previous value
fn count_increases(collection: &[usize]) -> usize {
    collection.windows(2).filter(|w| w[1] > w[0]).count()
}

fn count_increases_part2(collection: &[usize]) -> usize {
    collection
        .windows(3)
        .map(|w| w.iter().sum::<usize>())
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count()
}

fn load_depths_from_file(filepath: &str) -> Vec<usize> {
    let file = File::open(filepath).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|depth| depth.unwrap().parse::<usize>().unwrap())
        .collect()
}

pub fn part1(filepath: &str) -> usize {
    let depths = load_depths_from_file(filepath);
    count_increases(&depths)
}

pub fn part2(filepath: &str) -> usize {
    let depths = load_depths_from_file(filepath);
    count_increases_part2(&depths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advent_of_code_sample() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_increases(&depths), 7)
    }

    #[test]
    fn test_advent_of_code_sample_part2() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_increases_part2(&depths), 5)
    }
}
