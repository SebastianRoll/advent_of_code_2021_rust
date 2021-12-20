use std::fs;
use std::ops::Add;
use anyhow::{anyhow, Result};
use std::fmt;
use itertools::Itertools;


enum Dir {
    Left,
    Right
}

#[derive(PartialEq, Clone, Debug)]
pub enum SnailNum {
    Pair{
        left: Box<SnailNum>,
        right: Box<SnailNum>,
    },
    Regular(u8)
}

impl From<&str> for SnailNum {
    fn from(contents: &str) -> SnailNum {
        let mut iter =  contents.chars();
        SnailNum::parse_str_segment(&mut iter).expect("Parsing failed")
    }
}

impl fmt::Display for SnailNum{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         match self {
             SnailNum::Pair{left: l, right: r} => write!(f, "[{},{}]", l, r),
             SnailNum::Regular(num) => write!(f, "{}", num)
         }
    }
}

impl SnailNum {
    fn parse_str_segment(iter: &mut impl Iterator<Item = char>) -> Result<SnailNum> {
        while let Some(c) = iter.next() {
            match c {
                '[' => {
                    let snailnum = SnailNum::Pair{
                        left: Box::new(SnailNum::parse_str_segment(iter)?),
                        right: Box::new(SnailNum::parse_str_segment(iter)?)
                    };
                    return Ok(snailnum);
                },
                ']' => continue,
                ',' => continue,
                numchar if numchar.to_digit(10) != Option::None => {
                    // check for multidigit numbers
                    let mut final_num = numchar as u8 - 48;
                    while let Some(numchar_extra) = iter.peekable().peek() {
                        // eprintln!("numchar_extra = {:?}", numchar_extra);
                        if let Some(num) = numchar_extra.to_digit(10) {
                            iter.next().unwrap();
                            final_num = final_num*10 + num as u8
                        }
                        break;
                    }
                    
                    let snailnum = SnailNum::Regular(final_num);
                    return Ok(snailnum);
                }
                ch => {return Err(anyhow!("Invalid char {}", ch));}
            }
        }
        Err(anyhow!("Parsing failed somehow"))
    }

    fn magnitude(&self) -> usize {
        match self {
            &SnailNum::Regular(val) => usize::from(val),
            &SnailNum::Pair{ref left, ref right} => 3*left.magnitude() + 2*right.magnitude()
        }
    }
}


impl Add for SnailNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Pair{
            left: Box::new(self),
            right: Box::new(rhs)
        }
    }
}

#[derive(Copy, Clone)]
enum ExplodeStatus {
    None,
    Explode(u8, u8),
    Exploded(Option<u8>, Option<u8>)
}

// TODO: Move to utils
fn div_up(a: u8, b: u8) -> u8 {
    (a + (b - 1))/b
}

enum SplitStatus {
    Ongoing,
    Complete
}

fn split(snailnum: &mut SnailNum) -> SplitStatus {
    match snailnum {
        SnailNum::Pair{ref mut left, ref mut right} => {
            let status = split(left);
            match status {
                SplitStatus::Ongoing => {
                    let status_right = split(right);
                    return status_right
                },
                SplitStatus::Complete => SplitStatus::Complete
            }
        },
        SnailNum::Regular(val) if *val >= 10 => {
            // eprint!("{} SPLIT\n", val);
            *snailnum = SnailNum::Pair{
                left: Box::new(SnailNum::Regular(*val/2)),
                right: Box::new(SnailNum::Regular(div_up(*val, 2)))
            };
            return SplitStatus::Complete;
        },
        _ => SplitStatus::Ongoing
    }
}

fn explode_down(snailnum: &mut SnailNum, status: ExplodeStatus) {
    match (snailnum, status) {
        (SnailNum::Pair{ref mut left, ref mut right}, ExplodeStatus::Exploded(None, Some(rval))) => {
            match (left.as_ref(), right.as_ref()) {
                (SnailNum::Regular(leftval), _) => {
                    // eprint!("[{},{}] EXPLODED DOWN ØEFT\n", left, right);
                    **left = SnailNum::Regular(leftval + rval);
                    return;// ExplodeStatus::Exploded(None, None);
                },
                (SnailNum::Pair{left: leftchild, right: rightchild}, _) => explode_down(left, ExplodeStatus::Exploded(None, Some(rval))),
                (_, SnailNum::Regular(rightval)) => {
                    // eprint!("[{},{}] EXPLODED DOWN RIGHT\n", left, right);
                    **right = SnailNum::Regular(rightval + rval);
                    return;// ExplodeStatus::Exploded(None, None);
                },
                _ => ()
            }
        },
        (SnailNum::Pair{ref mut left, ref mut right}, ExplodeStatus::Exploded(Some(lval), None)) => {
            match (left.as_ref(), right.as_ref()) {
                (_, SnailNum::Regular(rightval)) => {
                    // eprint!("[{},{}] EEXPLODED DOWN ØEFT\n", left, right);
                    // eprint!("{},{} rightval, lval\n", rightval, lval);
                    **right = SnailNum::Regular(rightval + lval);
                    return;// ExplodeStatus::Exploded(None, None);
                },
                (_, SnailNum::Pair{left: leftchild, right: rightchild}) => explode_down(right, ExplodeStatus::Exploded(Some(lval), None)),
                (SnailNum::Regular(leftval), _) => {
                    // eprint!("[{},{}] EEXPLODED DOWN RIGHT\n", left, right);
                    **left = SnailNum::Regular(leftval + lval);
                    return;// ExplodeStatus::Exploded(None, None);
                },
                _ => ()
            } 
        },
        _ => ()
    }
}

fn part1(path: &str) -> SnailNum {
    let mut contents = fs::read_to_string(path).unwrap();
    contents.lines()
        .map(move |line| SnailNum::from(&line[..]))
        // .inspect(|snailnum| eprintln!("snailnum = {}", snailnum))
        .reduce(|acc, snailnum| {
            eprintln!("acc = {}", acc);
            println!("+");
            eprintln!("snailnum = {}", snailnum);
            let mut sum = acc + snailnum;
            println!("=");
            eprintln!("sum = {}", sum);
            reduce(&mut sum);
            println!("reduced");
            eprintln!("sum = {}", sum);
            println!("");
            println!("");
            sum
        })
        .unwrap()
}

fn part2(path: &str) -> usize {
    let contents = fs::read_to_string(path).unwrap();

    contents.lines()
    .map(|line| SnailNum::from(&line[..]))
    .permutations(2)
    .map(|w| {
        let mut sum = w[0].clone() + w[1].clone();
        reduce(&mut sum);
        sum.magnitude()
    })
    .max().unwrap()
}


fn reduce(snailnum: &mut SnailNum) {
    loop {
        // eprintln!("snailnum = {}", snailnum);
        if let ExplodeStatus::None = _will_explode(snailnum, 0) {
            if let SplitStatus::Ongoing = split(snailnum) {
                break;
            }
            // eprintln!("snailnum Split= {}", snailnum);
        }
        // eprintln!("snailnum Exploded = {}", snailnum);
    } 
    
}

fn will_explode(snailnum: &mut SnailNum, level: usize) {
    loop {
        if let ExplodeStatus::None = _will_explode(snailnum, level) {
            break;
        }
    } 
}

fn _will_explode(snailnum: &mut SnailNum, level: usize) -> ExplodeStatus {
    // check for each step if we can add the exploded values
    // use ref mut to keep mutability while traversing down the nodes
    // return a state that keeps track if exploded values have been added

    match snailnum {
        SnailNum::Regular(_) => { return ExplodeStatus::None; },
        SnailNum::Pair{
            ref left, ref right
        } if level == 4 => {
            match (left.as_ref(), right.as_ref()) {
            (SnailNum::Regular(leftval), SnailNum::Regular(rightval)) => {
                // eprint!("[{},{}] EXPLODED\n", leftval, rightval);
                return ExplodeStatus::Explode(*leftval, *rightval);
            },
            _ => unreachable!("Ah no")
            }
        },
        SnailNum::Pair{ref mut left, ref mut right} => {
            match _will_explode(left, level+1) {
                ExplodeStatus::Explode(lval, rval) => {
                    **left = SnailNum::Regular(0);
                    if let SnailNum::Regular(current_right) = **right {
                        **right = SnailNum::Regular(current_right+rval);
                        return ExplodeStatus::Exploded(Some(lval), None);
                    } else {
                        explode_down(right, ExplodeStatus::Exploded(None, Some(rval)));
                        return ExplodeStatus::Exploded(Some(lval), None);
                    }
                },
                ExplodeStatus::Exploded(None, None) => return ExplodeStatus::Exploded(None, None),
                ExplodeStatus::Exploded(Some(leftval), None) => return ExplodeStatus::Exploded(Some(leftval), None),
                ExplodeStatus::Exploded(None, Some(rightval)) => {
                    if let SnailNum::Regular(current_right) = **right {
                        // eprint!("[{},{}] EXPLODED LEFT\n", left, right);
                        **right = SnailNum::Regular(current_right+rightval);
                        return ExplodeStatus::Exploded(None, None);
                    } else {
                        explode_down(right, ExplodeStatus::Exploded(None, Some(rightval)));
                        return ExplodeStatus::Exploded(None, None);
                    }
                },
                ExplodeStatus::Exploded(Some(leftval), Some(rightval)) => {
                    if let SnailNum::Regular(current_right) = **right {
                        // eprint!("[{},{}] EXPLODED LEFT BOTH\n", left, right);
                        **right = SnailNum::Regular(current_right+rightval);
                    }
                    // eprintln!("snailnum = {}", snailnum);
                    return ExplodeStatus::Exploded(Some(leftval), None);
                },
                ExplodeStatus::None => (),
            };
            match _will_explode(right, level+1) {
                ExplodeStatus::Explode(lval, rval) => {
                    **right = SnailNum::Regular(0);
                    if let SnailNum::Regular(current_left) = **left {
                        // eprint!("[{},{}] EXPLODED RIGHT\n", left, right);
                        **left = SnailNum::Regular(current_left+lval);
                        return ExplodeStatus::Exploded(None, Some(rval));
                    } else {
                        explode_down(left, ExplodeStatus::Exploded(Some(lval), None));
                        return ExplodeStatus::Exploded(None, Some(rval));
                    }
                },
                ExplodeStatus::Exploded(None, None) => return ExplodeStatus::Exploded(None, None),
                ExplodeStatus::Exploded(None, Some(rightval)) => return ExplodeStatus::Exploded(None, Some(rightval)),
                ExplodeStatus::Exploded(Some(leftval), None) => {
                    if let SnailNum::Regular(current_left) = **left {
                        // eprint!("[{},{}] EXPLODED RIGHT BOTH\n", left, right);
                        **left = SnailNum::Regular(current_left+leftval);
                        return ExplodeStatus::Exploded(None, None);
                    } else {
                        explode_down(left, ExplodeStatus::Exploded(Some(leftval), None));
                        return ExplodeStatus::Exploded(None, None);
                    }
                },
                ExplodeStatus::Exploded(Some(leftval), Some(rightval)) => {
                    if let SnailNum::Regular(current_left) = **left {
                        **left = SnailNum::Regular(current_left+leftval);
                    }
                    return ExplodeStatus::Exploded(None, Some(rightval));
                },
                ExplodeStatus::None => ExplodeStatus::None,
                
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_snailnums() {
        use SnailNum::*;
        let lhs = Pair{
            left: Box::new(Pair{left: Box::new(Regular(1)), right: Box::new(Regular(2))}),
            right:  Box::new(Regular(3))
        };

        let rhs = Regular(4);
        let res = lhs + rhs;

        assert_eq!(format!("{}", res).as_str(), "[[[1,2],3],4]");
    }
    
    #[test]
    fn test_snailnum_fromstr() {
        let contents = "[1,2]";
        let snailnum = SnailNum::from(contents);

        assert_eq!(format!("{}", snailnum).as_str(), "[1,2]");
    }

    #[test]
    fn test_snailnum_explode_left() {
        let contents = "[[[[[9,8],1],2],3],4]";
        let mut snailnum = SnailNum::from(contents);
        _will_explode(&mut snailnum, 0);

        assert_eq!(format!("{}", snailnum).as_str(), "[[[[0,9],2],3],4]");
    }

    #[test]
    fn test_snailnum_explode_right() {
        let contents = "[7,[6,[5,[4,[3,2]]]]]";
        let mut snailnum = SnailNum::from(contents);
        _will_explode(&mut snailnum, 0);

        assert_eq!(format!("{}", snailnum).as_str(), "[7,[6,[5,[7,0]]]]");
    }

    #[test]
    fn test_snailnum_explode_others() {
        let mut snailnum = SnailNum::from("[[[[[9,8],1],2],3],4]");
        _will_explode(&mut snailnum, 0);
        assert_eq!(format!("{}", snailnum).as_str(), "[[[[0,9],2],3],4]", "Failed {}", snailnum);

        let mut snailnum = SnailNum::from("[7,[6,[5,[4,[3,2]]]]]");
        _will_explode(&mut snailnum, 0);
        assert_eq!(format!("{}", snailnum).as_str(), "[7,[6,[5,[7,0]]]]", "Failed {}", snailnum);
        
        let mut snailnum = SnailNum::from("[[6,[5,[4,[3,2]]]],1]");
        _will_explode(&mut snailnum, 0);
        assert_eq!(format!("{}", snailnum).as_str(), "[[6,[5,[7,0]]],3]", "Failed {}", snailnum);

        let mut snailnum = SnailNum::from("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        _will_explode(&mut snailnum, 0);
        assert_eq!(format!("{}", snailnum).as_str(), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "Failed {}", snailnum);

        let mut snailnum = SnailNum::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        _will_explode(&mut snailnum, 0);
        assert_eq!(format!("{}", snailnum).as_str(), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]", "Failed {}", snailnum);
    }
    
    #[test]
    fn test_snailnum_explode_full() {
        let mut snailnum = SnailNum::from("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        will_explode(&mut snailnum, 0);
        assert_eq!(format!("{}", snailnum).as_str(), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]", "Failed {}", snailnum);
    }
    
    #[test]
    fn test_snailnum_explode_19() {
        let init = "[[[[5,11],[13,0]],[[15,14],[14,0]]],[[2,[0,[11,4]]],[[[6,7],1],[7,[1,6]]]]]";
        let mut snailnum = SnailNum::from(init);
        _will_explode(&mut snailnum, 0);
        assert_eq!(format!("{}", snailnum).as_str(), "[[[[5,11],[13,0]],[[15,14],[14,0]]],[[2,[11,0]],[[[10,7],1],[7,[1,6]]]]]", "Failed {}", init);
    }
    
    #[test]
    fn test_split() {
        let mut snailnum = SnailNum::from("[[[[0,7],4],[15,[0,13]]],[1,1]]");
        split(&mut snailnum);
        assert_eq!(format!("{}", snailnum).as_str(), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
    }

    #[test]
    fn test_parse2() {
        let mut snailnum = SnailNum::from("[[[[0,7],4],[15,[0,13]]],[1,1]]");
        assert_eq!(format!("{}", snailnum).as_str(), "[[[[0,7],4],[15,[0,13]]],[1,1]]");

    }
    #[test]
    fn test_reduce1() {
        let mut snailnum = SnailNum::from("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        reduce(&mut snailnum);
        assert_eq!(format!("{}", snailnum).as_str(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }
    #[test]
    fn test_reduce2() {
        let mut snailnum = SnailNum::from("[[[[5,0],[[11,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]],[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]]");
        _will_explode(&mut snailnum, 0);
        assert_eq!(format!("{}", snailnum).as_str(), "[[[[5,11],[0,[13,0]]],[[8,[7,7]],[[7,9],[5,0]]]],[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]]");
    }
    
    #[test]
    fn test_example1() {
        let snailnum = part1("data/input18_example1.txt");
        assert_eq!(format!("{}", snailnum).as_str(), "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    }
    
    #[test]
    fn test_example2() {
        let snailnum = part1("data/input18_example2.txt");
        assert_eq!(snailnum.magnitude(), 4140);
        assert_eq!(format!("{}", snailnum).as_str(), "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]");
    }    
    
    #[test]
    fn test_example2_part2() {
        let mag = part2("data/input18_example2.txt");
        assert_eq!(mag, 3993);
    }
    
    #[test]
    fn test_part2() {
        let mag = part2("data/input18.txt");
        assert_eq!(mag, 4735);
    }
    
    #[test]
    fn test_part1() {
        let snailnum = part1("data/input18.txt");
        assert_eq!(snailnum.magnitude(), 3699);
        assert_eq!(format!("{}", snailnum).as_str(), "[[[[7,7],[7,8]],[[7,0],[8,7]]],[[[5,6],[6,6]],[[6,4],[0,6]]]]");
    }


    #[test]
    fn test_example1_iter() {
        let mut contents = fs::read_to_string("data/input18_example1_iter.txt").unwrap();
        let mut iter = contents.lines();

        loop {
            let a: SnailNum;
            if let Some(line) = iter.next() {
                a = SnailNum::from(line);
            } else {
                break;
            }
            
            let b = SnailNum::from(iter.next().unwrap());
            let expected = SnailNum::from(iter.next().unwrap());
            let mut sum = a + b;
            
            reduce(&mut sum);
            eprintln!("sum = {}", sum);
            assert_eq!(format!("{}", sum).as_str(), format!("{}", expected).as_str());
        }
    }
    
    
    #[test]
    fn test_add1() {
        let mut lhs = SnailNum::from("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");
        let mut rhs = SnailNum::from("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]");
        let mut sum = lhs + rhs;
        eprintln!("snailnum = {}", sum);
        reduce(&mut sum);
        assert_eq!(format!("{}", sum).as_str(), "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]");
    }

        
}
