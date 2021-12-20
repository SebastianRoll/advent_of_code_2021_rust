use bitstream_io::{BigEndian, BitRead, BitReader};
use hex;
use std::fs;
use std::io::Cursor;

#[derive(Debug)]
enum PacketType {
    Literal,
    Sum,
    Product,
    Min,
    Max,
    Gt,
    Lt,
    Equal,
}
impl From<u8> for PacketType {
    fn from(val: u8) -> Self {
        match val {
            4 => Self::Literal,
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Min,
            3 => Self::Max,
            5 => Self::Gt,
            6 => Self::Lt,
            7 => Self::Equal,
            _ => panic!("Invalid type value"),
        }
    }
}
impl PacketType {
    fn op(&self, nums: Vec<usize>) -> usize {
        match self {
            Self::Sum => nums.iter().sum(),
            Self::Product => nums.iter().product(),
            Self::Min => *nums.iter().min().unwrap(),
            Self::Max => *nums.iter().max().unwrap(),
            Self::Gt => (nums[0] > nums[1]) as usize,
            Self::Lt => (nums[0] < nums[1]) as usize,
            Self::Equal => (nums[0] == nums[1]) as usize,
            _ => panic!("Invalid type value"),
        }
    }
}

pub fn part1(path: &str) -> usize {
    let contents = fs::read_to_string(path).unwrap();
    solvu(&contents[..]).1
}
pub fn part2(path: &str) -> usize {
    let contents = fs::read_to_string(path).unwrap();
    solvu(&contents[..]).0
}

fn solve(contents: &str) -> usize {
    solvu(contents).0
}
fn solvu(contents: &str) -> (usize, usize) {
    let mut buf = contents.trim().to_owned();
    let odd_length = buf.len() % 2 == 1;
    if odd_length {
        eprintln!("WARN = {:?}", 1);
        buf.push_str("0");
    }
    let buf_u8 = hex::decode(&buf[..]).unwrap();
    let mut reader = BitReader::endian(Cursor::new(&buf_u8), BigEndian);
    parse(&mut reader)
}

fn parse<T: BitRead>(reader: &mut T) -> (usize, usize) {
    let score: usize;
    let mut read_len = 0_usize;

    read_len += 3;
    let _version = reader.read::<u8>(3).unwrap();
    // eprintln!("version = {:?}", version);

    read_len += 3;
    let type_id = reader.read::<u8>(3).unwrap();
    // eprintln!("type_id = {:?}", type_id);
    let packet_type = PacketType::from(type_id);
    // eprintln!("packet_type = {:?}", packet_type);
    match packet_type {
        PacketType::Literal => {
            let mut literal_val: usize = 0;
            loop {
                read_len += 5;
                let stop = !reader.read_bit().unwrap();
                let val = reader.read::<u8>(4).unwrap();
                literal_val = (literal_val << 4) + usize::from(val);
                if stop {
                    break;
                }
            }
            score = literal_val;
            // eprintln!("LITERAL = {:?}", score);
        }
        op => {
            let (nums, rl) = operator(reader);
            read_len += rl;
            score = op.op(nums);
        }
    };
    (score, read_len)
}

fn operator<T: BitRead>(reader: &mut T) -> (Vec<usize>, usize) {
    let mut read_len = 0_usize;

    let mut nums = Vec::new();
    read_len += 1;
    let length_type_id = reader.read::<u8>(1).unwrap();
    match length_type_id {
        0b0 => {
            // println!("OPERATOR LENGTH");
            read_len += 15;
            let mut length = reader.read::<u32>(15).unwrap();
            while length > 0 {
                let (sc, rl) = parse(reader);
                nums.push(sc);
                length -= rl as u32;
                read_len += rl;
            }
        }
        0b1 => {
            // println!("OPERATOR COUNT");
            read_len += 11;
            let subpackets = reader.read::<u16>(11).unwrap();
            // eprintln!("subpackets = {:?}", subpackets);
            for _ in 0..subpackets {
                let (sc, rl) = parse(reader);
                nums.push(sc);
                read_len += rl;
            }
            //reader.read_unary1().unwrap();
        }
        _ => unreachable!("ya"),
    }
    (nums, read_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_example_4() {
    //     let contents = "8A004A801A8002F478";
    //     assert_eq!(solve(contents), 16)
    // }

    // #[test]
    // fn test_example_5() {
    //     let contents = "620080001611562C8802118E34";
    //     assert_eq!(solve(contents), 12)
    // }

    // #[test]
    // fn test_example_6() {
    //     let contents = "C0015000016115A2E0802F182340";
    //     assert_eq!(solve(contents), 23)
    // }

    // #[test]
    // fn test_example_7() {
    //     let contents = "A0016C880162017C3686B18A3D4780";
    //     assert_eq!(solve(contents), 31)
    // }

    #[test]
    fn test_example_8() {
        let contents = "C200B40A82";
        assert_eq!(solve(contents), 3)
    }
    #[test]
    fn test_example_9() {
        let contents = "04005AC33890";
        assert_eq!(solve(contents), 54)
    }
    #[test]
    fn test_example_10() {
        let contents = "880086C3E88112";
        assert_eq!(solve(contents), 7)
    }
    #[test]
    fn test_example_11() {
        let contents = "CE00C43D881120";
        assert_eq!(solve(contents), 9)
    }
    #[test]
    fn test_example_12() {
        let contents = "D8005AC2A8F0";
        assert_eq!(solve(contents), 1)
    }
    #[test]
    fn test_example_13() {
        let contents = "F600BC2D8F";
        assert_eq!(solve(contents), 0)
    }
    #[test]
    fn test_example_14() {
        let contents = "9C005AC2F8F0";
        assert_eq!(solve(contents), 0)
    }
    #[test]
    fn test_example_15() {
        let contents = "9C0141080250320F1802104A08";
        assert_eq!(solve(contents), 1)
    }
}
