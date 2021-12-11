mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
//mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

fn main() {
    println!("DAY 1");
    println!("part2: {}", day01::part1("data/input01.txt"));
    println!("part2: {}", day01::part2("data/input01.txt"));

    println!("DAY 2");
    //println!("part1: {}", day02::part1("data/input02.txt"));
    println!("part2: {}", day02::part2("data/input02.txt"));

    println!("DAY 3");
    //println!("part1: {}", day03::part1("data/input03.txt"));
    println!("part2: {}", day03::part2("data/input03.txt"));

    println!("DAY 4");
    println!("part1: {}", day04::part1("data/input04.txt"));
    println!("part2: {}", day04::part2("data/input04.txt"));

    println!("DAY 5");
    //println!("part1: {}", day05::part1("data/input05.txt"));
    println!("part2: {}", day05::part2("data/input05.txt"));

    println!("DAY 6");
    println!("part1: {}", day06::part1("data/input06.txt"));
    println!("part2: {}", day06::part2("data/input06.txt"));

    println!("DAY 7");
    //println!("part1: {}", day07::part1("data/input07.txt"));
    //println!("part2: {}", day07::part2("data/input07.txt"));

    println!("DAY 8");
    //println!("part1: {}", day08::part1("data/input08.txt"));
    println!("part2: {}", day08::part2("data/input08.txt"));

    println!("DAY 9");
    println!("part1: {}", day09::part1("data/input09.txt"));
    println!("part2: {}", day09::part2("data/input09.txt"));

    println!("DAY 10");
    println!("part1: {}", day10::part1());
    println!("part2: {}", day10::part2());

    println!("DAY 11");
    println!("part1: {}", day11::part1("data/input11.txt", 100));
    println!("part2: {}", day11::part2("data/input11.txt", 999));
}
