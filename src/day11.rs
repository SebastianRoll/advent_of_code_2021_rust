use std::cmp::min;
use std::fs;

type FlashCount = usize;

fn from_file(path: &str) -> Vec<Vec<u8>> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c as u8 - 48) // ASCII)
                .collect::<Vec<u8>>()
        })
        .collect()
}

pub fn part1(path: &str, steps: usize) -> FlashCount {
    let mut matrix = from_file(path);
    let mut flashes = 0;

    for _ in 1..=steps {
        let flashes_step = step(&mut matrix);

        flashes += flashes_step;
    }
    flashes
}

pub fn part2(path: &str, steps: usize) -> usize {
    let mut matrix = from_file(path);
    let mut flashes = 0;

    let r_len = matrix.len();
    let c_len = matrix[0].len();

    for s in 1..=steps {
        let flashes_step = step(&mut matrix);

        if flashes_step == r_len * c_len {
            return s;
        }

        flashes += flashes_step;
    }
    flashes
}

fn step(matrix: &mut Vec<Vec<u8>>) -> FlashCount {
    let r_len = matrix.len();
    let c_len = matrix[0].len();
    let mut flashes = 0;

    for r in 0..r_len {
        for c in 0..c_len {
            matrix[r][c] += 1;
        }
    }

    for r in 0..r_len {
        for c in 0..c_len {
            if matrix[r][c] > 9 {
                flashes += flash(matrix, r, c);
            }
        }
    }

    flashes
}

fn flash(matrix: &mut Vec<Vec<u8>>, r: usize, c: usize) -> FlashCount {
    let r_len = matrix.len();
    let c_len = matrix[0].len();

    let mut flashes = 1;
    matrix[r][c] = 0;
    for r in r.saturating_sub(1)..=min(r + 1, r_len - 1) {
        for c in c.saturating_sub(1)..=min(c + 1, c_len - 1) {
            match matrix[r][c] {
                // n @ 1..9 => matrix[r][c] += 1,
                n if n >= 1 && n < 9 => matrix[r][c] += 1,
                0 => (),
                _ => flashes += flash(matrix, r, c),
            };
        }
    }
    flashes
}
