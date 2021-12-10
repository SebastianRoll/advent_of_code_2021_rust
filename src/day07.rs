use std::cmp;
use std::fs;

fn calc(path: &str, calc_func: dyn Fn(&[u32], u32) -> u32) -> u32 {
    let mut bot = u32::MAX;
    let mut top = u32::MIN;
    let positions = fs::read_to_string(path).unwrap()
        .trim()    
        .split(",")
        .map(|c| {
            let pos = c.parse::<u32>().unwrap();
            bot = cmp::min(pos, bot);
            top = cmp::max(pos, top);
            pos
        })
        .collect::<Vec<u32>>();
    println!("{:?}, {}, {}", positions, bot, top);
    let min_fuel: u32 = (bot..top)
        .map(|t| calc_func(&positions, t))
        .min().unwrap();

    println!("{:?}", min_fuel);
    min_fuel
}

fn fuel_cost_part1(positions: &[u32], target_pos: u32) -> u32 {
    positions.iter()
        .map(|&p| abs_diff(p, target_pos))
        .sum()
}


fn fuel_cost_part2(positions: &[u32], target_pos: u32) -> u32 {
    positions.iter()
        .map(|&p| {
            let distance = abs_diff(p, target_pos);
            // using nth partial sum formula 
            distance*(distance+1)/2
        })
        .sum()
}

fn abs_diff(a: u32, b: u32) -> u32 {
    if a > b {a-b} else {b-a}
}

pub fn part1(path: &str) -> u32 {
    calc(path, fuel_cost_part1)
}

pub fn part2(path: &str) -> u32 {
    calc(path, fuel_cost_part2)
}