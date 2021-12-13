use std::fs::File;
use std::io::{self, BufRead};

enum NavCommand {
    Up(usize),
    Down(usize),
    Forward(usize),
}

impl From<String> for NavCommand {
    fn from(s: String) -> Self {
        let mut it = s.split_whitespace();
        let direction = it.next().unwrap();
        let units = it.next().unwrap().parse::<usize>().unwrap();

        match direction {
            "up" => NavCommand::Up(units),
            "down" => NavCommand::Down(units),
            "forward" => NavCommand::Forward(units),
            _ => panic!("Invalid command"),
        }
    }
}

#[derive(Debug)]
struct SubmarineState1 {
    pos: usize,
    debth: usize,
}

impl SubmarineState1 {
    fn new() -> Self {
        SubmarineState1 { pos: 0, debth: 0 }
    }

    fn answer(&self) -> usize {
        self.pos * self.debth
    }

    fn process_command(self, com: NavCommand) -> Self {
        match com {
            NavCommand::Up(x) => Self {
                debth: self.debth - x,
                ..self
            },
            NavCommand::Down(x) => Self {
                debth: self.debth + x,
                ..self
            },
            NavCommand::Forward(x) => Self {
                pos: self.pos + x,
                ..self
            },
        }
    }
}

pub fn part1(path: &str) -> usize {
    let file = File::open(path).unwrap();
    let mut substate = SubmarineState1::new();
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap().to_string();
        let command = NavCommand::from(line);
        substate = substate.process_command(command);
    }
    substate.answer()
}

//////////////////////////////////

#[derive(Debug)]
struct SubmarineState2 {
    pos: usize,
    debth: usize,
    aim: usize,
}

impl SubmarineState2 {
    fn new() -> Self {
        SubmarineState2 {
            pos: 0,
            debth: 0,
            aim: 0,
        }
    }

    fn answer(&self) -> usize {
        self.pos * self.debth
    }

    fn process_command(self, com: NavCommand) -> Self {
        match com {
            NavCommand::Up(x) => Self {
                aim: self.aim - x,
                ..self
            },
            NavCommand::Down(x) => Self {
                aim: self.aim + x,
                ..self
            },
            NavCommand::Forward(x) => Self {
                pos: self.pos + x,
                debth: self.debth + (self.aim * x),
                ..self
            },
        }
    }
}

pub fn part2(path: &str) -> usize {
    let file = File::open(path).unwrap();
    let mut substate = SubmarineState2::new();
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap().to_string();
        let command = NavCommand::from(line);
        substate = substate.process_command(command);
    }
    substate.answer()
}
