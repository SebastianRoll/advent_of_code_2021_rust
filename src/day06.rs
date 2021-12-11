use std::fs;

const LANTERN_REPRODUCE_TIME: usize = 7;
const LANTERN_FIRST_GEN_EXTRA_TIME: usize = 2;

fn fish_after_days(path: &str, days: usize) -> u64 {
    let mut firstgen = [0_u64; LANTERN_REPRODUCE_TIME + LANTERN_FIRST_GEN_EXTRA_TIME];
    let mut nthgen = [0_u64; LANTERN_REPRODUCE_TIME];

    fs::read_to_string(path)
        .unwrap()
        .trim()
        .split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .for_each(|day| firstgen[day] += 1);

    for _ in 0..days {
        let new_firstgen_last = firstgen[0] + nthgen[0];
        let new_nthgen_last = firstgen[0] + nthgen[0];

        back_propagate(&mut firstgen);
        back_propagate(&mut nthgen);

        firstgen[firstgen.len() - 1] = new_firstgen_last;
        nthgen[nthgen.len() - 1] = new_nthgen_last;
    }
    let sum: u64 = firstgen.iter().chain(nthgen.iter()).sum();
    sum
}

fn back_propagate(school: &mut [u64]) {
    for i in 0..(school.len() - 1) {
        school[i] = school[i + 1];
    }
}

pub fn part1(path: &str) -> u64 {
    fish_after_days(path, 80)
}

pub fn part2(path: &str) -> u64 {
    fish_after_days(path, 256)
}
