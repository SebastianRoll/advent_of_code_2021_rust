use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

struct Example {
    input: Vec<String>,
    output: Vec<String>
}

impl Example {
    fn solve(&self) -> usize {
        /*
        let mut lengths = HashMap::new();
        for digstr in self.input {
            let mut v = lengths.entry(digstr.len()).or_insert(Vec::new());
            v.push(digstr);
        }
        */
        let mut map = HashMap::new();


        // TODO: use vec.sort_by
        let one = self.input.iter()
            .filter(|&s| s.len() == 2)
            .nth(0).unwrap();
        let seven = self.input.iter()
            .filter(|&s| s.len() == 3)
            .nth(0).unwrap();
        let four = self.input.iter()
            .filter(|&s| s.len() == 4)
            .nth(0).unwrap();
        let twofivethree: Vec<String> = self.input.iter()
            .filter(|&s| s.len() == 5)
            .cloned()
            .collect();
        let zerosixnine: Vec<String> = self.input.iter()
            .filter(|&s| s.len() == 6)
            .cloned()
            .collect();
        let eight = self.input.iter()
            .filter(|&s| s.len() == 7)
            .nth(0).unwrap();
        let one_set: HashSet<char> = one.chars().collect();
        let four_set: HashSet<char> = four.chars().collect();
        let seven_set: HashSet<char> = seven.chars().collect();
        let three = twofivethree.iter()
            .filter(|&num| {
                let num_set: HashSet<char> = num.chars().collect();
                let diff: HashSet<char> = num_set.difference(&one_set).map(|&a| a).collect();
                diff.len() == 3
            })
            .nth(0).unwrap();

            let nine = zerosixnine.iter()
            .filter(|&num| {
                let num_set: HashSet<char> = num.chars().collect();
                let diff: HashSet<char> = four_set.difference(&num_set).map(|&a| a).collect();
                diff.len() == 0
            })
            .nth(0).unwrap();
        let nine_set: HashSet<char> = nine.chars().collect();

        let two = twofivethree.iter()
            .filter(|&num| {
                let num_set: HashSet<char> = num.chars().collect();
                let diff: HashSet<char> = nine_set.difference(&num_set).map(|&a| a).collect();
                diff.len() == 2
            })
            .nth(0).unwrap();

        let five = twofivethree.iter()
            .filter(|&num| {
                num != two && num != three
            })
            .nth(0).unwrap();

        let six = zerosixnine.iter()
        .filter(|&num| {
            let num_set: HashSet<char> = num.chars().collect();
            let diff: HashSet<char> = one_set.difference(&num_set).map(|&a| a).collect();
            diff.len() == 1
        })
        .nth(0).unwrap();

        let zero = zerosixnine.iter()
        .filter(|&num| {
            num != six && num != nine
        })
        .nth(0).unwrap();

            /*
        
        4.difference(1).difference(twofivethree) -> if .len() == 1 : then 2 (then 5)
        zerosixnine.difference(7) -> if .len() == zero -> 3, three -> 2, six -> 3
        1.difference(zerosixnine) -> if .len() == 1 : then 6
        */
        let zero_set: HashSet<char> = zero.chars().collect();
        let two_set: HashSet<char> = two.chars().collect();
        let three_set: HashSet<char> = three.chars().collect();
        let five_set: HashSet<char> = five.chars().collect();
        let six_set: HashSet<char> = six.chars().collect();
        let eight_set: HashSet<char> = eight.chars().collect();
        let col = [
            zero_set,
            one_set,
            two_set,
            three_set,
            four_set,
            five_set,
            six_set,
            seven_set,
            eight_set,
            nine_set,
        ];

        
        map.insert(String::from(zero), String::from("zero"));
        map.insert(String::from(one), String::from("one"));
        map.insert(String::from(two), String::from("two"));
        map.insert(String::from(three), String::from("three"));
        map.insert(String::from(four), String::from("four"));
        map.insert(String::from(five), String::from("five"));
        map.insert(String::from(six), String::from("six"));
        map.insert(String::from(seven), String::from("seven"));
        map.insert(String::from(eight), String::from("eight"));
        map.insert(String::from(nine), String::from("nine"));

        let solution: usize = self.output.iter()
        .map(|secret| {
            col.iter().position(|set| {
                let secret_set: HashSet<char> = secret.chars().collect();
                set==&secret_set
            }).unwrap()
        })
        .enumerate()
        .map(|(i, v)| v*10_usize.pow(self.output.len() as u32 - (i+1) as u32))
        .sum();
        solution
    }
}

pub fn part2(path: &str) -> usize {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();
    let examples = lines
        .map(|line| {
            let l = line.unwrap();
            let mut iter = l.split(" | ");
            let input = iter.next().unwrap().to_string().split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
            let o = iter.next().unwrap().to_string();
            let output = o.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
            Example{
                input, output
            }
        })
        .collect::<Vec<Example>>();
    let sol: usize = examples.iter()
    .map(|ex| ex.solve())
    .sum();
    sol
}
/*
0: 6
1: 2
2: 5
3: 5
4: 4
5: 5
6: 6
7: 3
8: 7
*/
