use std::fs;
use hex;
use std::io::{Read, Cursor};
use bitstream_io::{BigEndian, BitReader, BitRead};

#[derive(Debug)]
enum PacketType {
    Literal = 4,
    Operator,
}
impl From<u8> for PacketType {
    fn from(val: u8) -> Self {
        match val {
            4 => Self::Literal,
            _ => Self::Operator,
            // _ => panic!("Invalid type value")
        }
    }
}

pub fn part1(path: &str) -> usize {
    let mut contents = fs::read_to_string(path).unwrap();
    solve(&contents[..])
}

fn solve(contents: &str) -> usize {
    solvu(contents).0
}
fn solvu(contents: &str) -> (usize, usize) {
    let mut buf = contents.trim().to_owned();
    let odd_length = buf.len() % 2 == 1;
    if odd_length {
        buf.push_str("0");
    }
    let buf_u8 = hex::decode(&buf[..]).unwrap();
    /*
    buf
        .inspect(|c| println!("{}", c) )
        .map(|c| hex::decode(format!("0{}", c)).unwrap()[0] )
        .inspect(|c| println!("{}", c) )
        .collect::<Vec<u8>>();
    */
    
    /*
    eprintln!("buf = {:?}", buf_u8);
    for n in buf_u8.iter() {
        eprintln!("{:b}", n);
    }
    println!("OK");
    */
    let mut reader = BitReader::endian(Cursor::new(&buf_u8), BigEndian);
    /*
    loop {
        println!("{:b}", reader.read::<u8>(8).unwrap());
    }
    */
    parse(&mut reader)
}

fn parse<T: BitRead>(reader: &mut T) -> (usize, usize) {
    let mut score = 0_usize;
    let mut read_len = 0_usize;

    read_len += 3;
    let version = reader.read::<u8>(3).unwrap();
    eprintln!("version = {:?}", version);

    score += usize::from(version);

    read_len += 3;
    let type_id = reader.read::<u8>(3).unwrap();
    eprintln!("type_id = {:?}", type_id);
    let packet_type = PacketType::from(type_id);
    eprintln!("packet_type = {:?}", packet_type);
    match packet_type {
        PacketType::Literal => {
            println!("LITERAL");
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
        },
        PacketType::Operator => {
            read_len += 1;
            let length_type_id = reader.read::<u8>(1).unwrap();
            match length_type_id {
                0b0 => {
                    println!("OPERATOR LENGTH");
                    read_len += 15;
                  let mut length = reader.read::<u32>(15).unwrap();
                  /*
                  let mut packet_buf = Vec::new();
                  while length > 8 {
                      length -= 8;
                      let byte = reader.read::<u8>(8).unwrap();
                      packet_buf.push(byte);
                  }
                  if length > 0 {
                      let byte = reader.read::<u8>(length).unwrap();
                      packet_buf.push(byte);
                  }
                  */
                  //let mut subreader = BitReader::endian(Cursor::new(&packet_buf), BigEndian);
                  //subreader.read::<u64>(64-length).unwrap();
                  while length > 0 {
                    let (sc, rl) = parse(reader);
                    score += sc;
                    read_len += rl;
                    length -= rl as u32;
                  }
                },
                0b1 => {
                    println!("OPERATOR COUNT");
                    //let mut packet_buf = Vec::new();
                    read_len += 11;
                    let subpackets = reader.read::<u16>(11).unwrap();
                    eprintln!("subpackets = {:?}", subpackets);
                    for i in 0..subpackets {
                        //lsubpackets.push(reader.read::<u16>(11).unwrap());
                        //let mut subreader = BitReader::endian(Cursor::new(&packet_buf), BigEndian);
                        //packet_buf.read::<u16>(16-length).unwrap();
                        let (sc, rl) = parse(reader);
                        score += sc;
                        read_len += rl;
                        
                    }
                    //reader.read_unary1().unwrap();

                },
                _ => unreachable!("ya")
            }

        },
        _ => panic!("Invalid type id")
    };
    /*
    let b = buf[0];
    let version = b >> 1;
    let b_typeid = (buf[0] << 4) + buf[1];
    eprintln!("typeid = {:?}", b_typeid);
    eprintln!("typeid BIN = {:b}", b_typeid);
    let type_id = b_typeid >> 2 & 0b111;
    eprintln!("version = {:?}", version);
    eprintln!("type_id = {:?}", type_id);
    */
    (score, read_len)
}

//fn process_literal(&reader: BitReader<R: Read, E: Endianness>) {



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_4() {
        let contents = "8A004A801A8002F478";
        assert_eq!(solve(contents), 16)
    }

    #[test]
    fn test_example_5() {
        let contents = "620080001611562C8802118E34";
        assert_eq!(solve(contents), 12)
    }

    #[test]
    fn test_example_6() {
        let contents = "C0015000016115A2E0802F182340";
        assert_eq!(solve(contents), 23)
    }

    #[test]
    fn test_example_7() {
        let contents = "A0016C880162017C3686B18A3D4780";
        assert_eq!(solve(contents), 31)
    }
}