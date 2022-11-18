#![feature(test)]
extern crate test;

use anyhow::Result;
use nom::IResult;
use nom::bits::complete::take;
use aoc::bit_ops::BitOps;

fn main() -> Result<()> {
    let input = get_input()?;

    let (part_1, part_2) = solve(&input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);

    Ok(())
}

fn get_input() -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(include_str!("../inputs/day16.inp").trim())
}

fn solve(input: &[u8]) -> (usize, i64) {
    let (_, packet) = Packet::parse((input, 0)).unwrap();

    let part_1 = version_sum(&packet);
    let part_2 = packet.eval();

    (part_1, part_2)
}

fn version_sum(packet: &Packet) -> usize {
    use PacketContents::*;
    packet.version as usize + match &packet.contents {
        Literal(_) => 0,
        Operator(sub_packets) => sub_packets.iter().map(version_sum).sum(),
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Packet {
    version: u8,
    type_id: u8,
    contents: PacketContents,
}

impl Packet {
    fn parse(input: (&[u8], usize)) -> IResult<(&[u8], usize), Self> {
        let (input, version) = take(3_usize)(input)?;
        let (input, (type_id, contents)) = PacketContents::parse(input)?;

        Ok((input, Self { version, type_id, contents }))
    }
    
    fn eval(&self) -> i64 {
        match self.type_id {
            0 => self.contents.subpackets().unwrap().iter().map(Packet::eval).sum(),
            1 => self.contents.subpackets().unwrap().iter().map(Packet::eval).product(),
            2 => self.contents.subpackets().unwrap().iter().map(Packet::eval).min().unwrap(),
            3 => self.contents.subpackets().unwrap().iter().map(Packet::eval).max().unwrap(),
            4 => self.contents.literal().unwrap() as i64,
            5 => {
                match self.contents.subpackets().unwrap() {
                    [a, b] => if a.eval() > b.eval() { 1 } else { 0 },
                    _ => unreachable!("Greater than packets always have exactly two sub-packets"),
                }
            }
            6 => {
                match self.contents.subpackets().unwrap() {
                    [a, b] => if a.eval() < b.eval() { 1 } else { 0 },
                    _ => unreachable!("Less than packets always have exactly two sub-packets"),
                }
            }
            7 => {
                match self.contents.subpackets().unwrap() {
                    [a, b] => if a.eval() == b.eval() { 1 } else { 0 },
                    _ => unreachable!("Equal to packets always have exactly two sub-packets"),
                }
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum PacketContents {
    Literal(u64),
    Operator(Vec<Packet>),
}

impl PacketContents {
    fn read_literal(mut input: (&[u8], usize)) -> IResult<(&[u8], usize), Self> {
        let mut val = 0;
        loop {
            let (remaining, block): (_, u8) = take(5_usize)(input)?;
            input = remaining;

            val = (val << 4) | (block & 0xF) as u64;

            if !block.test_bit(4) {
                break Ok((input, Self::Literal(val)));
            }
        }
    }

    fn bits_read((slice1, offset1): (&[u8], usize), (slice2, offset2): (&[u8], usize)) -> usize {
        // NOTE: this only works because the references are all to the same slice of contiguous input data
        let start = slice1.as_ptr() as usize;
        let end = slice2.as_ptr() as usize;

        (end * 8 + offset2) - (start * 8 + offset1)
    }

    fn read_operator(input: (&[u8], usize), ) -> IResult<(&[u8], usize), Self> {
        use LengthTypeId::*;

        let (mut input, length_type_id) = LengthTypeId::parse(input)?;
        let mut sub_packets: Vec<Packet> = vec![];
        match length_type_id {
            TotalLength(bits) => {
                let start = input;
                loop {
                    let (remainder, packet) = Packet::parse(input)?;
                    input = remainder;
                    sub_packets.push(packet);

                    if Self::bits_read(start, input) == bits as usize {
                        break;
                    }
                }
            }
            NumSubPackets(packets) => {
                for _ in 0..packets {
                    let (remainder, packet) = Packet::parse(input)?;
                    input = remainder;
                    sub_packets.push(packet);
                }
            }
        }

        Ok((input, Self::Operator(sub_packets)))
    }

    fn parse(input: (&[u8], usize)) -> IResult<(&[u8], usize), (u8, Self)> {
        let (input, type_id) = take(3_usize)(input)?;
        Ok(match type_id {
            4 => {
                let (input, literal) = Self::read_literal(input)?;
                (input, (type_id, literal))
            }
            _ => {
                let (input, operator) = Self::read_operator(input)?;
                (input, (type_id, operator))
            }
        })
    }
    
    fn subpackets(&self) -> Option<&[Packet]> {
        use PacketContents::*;

        match self {
            Literal(_) => None,
            Operator(subpackets) => Some(subpackets.as_slice()),
        }
    }

    fn literal(&self) -> Option<u64> {
        use PacketContents::*;

        match self {
            Literal(lit) => Some(*lit),
            Operator(_) => None,
        }
    }
}

#[derive(Debug)]
enum LengthTypeId {
    TotalLength(u16),
    NumSubPackets(u16),
}

impl LengthTypeId {
    fn parse(input: (&[u8], usize)) -> IResult<(&[u8], usize), Self> {
        let (input, length_type_id) = take(1_usize)(input)?;
        match length_type_id {
            0 => {
                let (input, bits) = take(15_usize)(input)?;
                Ok((input, Self::TotalLength(bits)))
            }
            1 => {
                let (input, packets) = take(11_usize)(input)?;
                Ok((input, Self::NumSubPackets(packets)))
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::PacketContents::*;

    #[test]
    fn test_parse_structure_1() {
        let data = hex::decode("D2FE28").unwrap();
        let input = (data.as_slice(), 0);
        let parsed = Packet::parse(input).unwrap().1;

        assert_eq!(parsed, Packet {
            version: 6,
            type_id: 4,
            contents: Literal(2021),
        });
    }

    #[test]
    fn test_parse_structure_2() {
        let data = hex::decode("38006F45291200").unwrap();
        let input = (data.as_slice(), 0);
        let parsed = Packet::parse(input).unwrap().1;

        assert_eq!(parsed, Packet {
            version: 1,
            type_id: 6,
            contents: Operator(vec![
                Packet {
                    version: 6,
                    type_id: 4,
                    contents: Literal(10),
                },
                Packet {
                    version: 2,
                    type_id: 4,
                    contents: Literal(20),
                },
            ]),
        });
    }

    #[test]
    fn test_parse_structure_3() {
        let data = hex::decode("EE00D40C823060").unwrap();
        let input = (data.as_slice(), 0);
        let parsed = Packet::parse(input).unwrap().1;

        assert_eq!(parsed, Packet {
            version: 7,
            type_id: 3,
            contents: Operator(vec![
                Packet {
                    version: 2,
                    type_id: 4,
                    contents: Literal(1),
                },
                Packet {
                    version: 4,
                    type_id: 4,
                    contents: Literal(2),
                },
                Packet {
                    version: 1,
                    type_id: 4,
                    contents: Literal(3),
                },
            ]),
        });
    }

    #[test]
    fn test_packet_version_sum_1() {
        let data = hex::decode("8A004A801A8002F478").unwrap();
        let input = (data.as_slice(), 0);
        let packet = Packet::parse(input).unwrap().1;

        assert_eq!(version_sum(&packet), 16);
    }

    #[test]
    fn test_packet_version_sum_2() {
        let data = hex::decode("620080001611562C8802118E34").unwrap();
        let input = (data.as_slice(), 0);
        let packet = Packet::parse(input).unwrap().1;

        assert_eq!(version_sum(&packet), 12);
    }

    #[test]
    fn test_packet_version_sum_3() {
        let data = hex::decode("C0015000016115A2E0802F182340").unwrap();
        let input = (data.as_slice(), 0);
        let packet = Packet::parse(input).unwrap().1;

        assert_eq!(version_sum(&packet), 23);
    }

    #[test]
    fn test_packet_version_sum_4() {
        let data = hex::decode("A0016C880162017C3686B18A3D4780").unwrap();
        let input = (data.as_slice(), 0);
        let packet = Packet::parse(input).unwrap().1;

        assert_eq!(version_sum(&packet), 31);
    }

    #[test]
    fn test_packet_eval_1() {
        let data = hex::decode("C200B40A82").unwrap();
        let input = (data.as_slice(), 0);
        let packet = Packet::parse(input).unwrap().1;

        assert_eq!(packet.eval(), 3);
    }

    #[test]
    fn test_packet_eval_2() {
        let data = hex::decode("04005AC33890").unwrap();
        let input = (data.as_slice(), 0);
        let packet = Packet::parse(input).unwrap().1;

        assert_eq!(packet.eval(), 54);
    }

    #[test]
    fn test_packet_eval_3() {
        let data = hex::decode("880086C3E88112").unwrap();
        let input = (data.as_slice(), 0);
        let packet = Packet::parse(input).unwrap().1;

        assert_eq!(packet.eval(), 7);
    }

    #[test]
    fn test_packet_eval_4() {
        let data = hex::decode("CE00C43D881120").unwrap();
        let input = (data.as_slice(), 0);
        let packet = Packet::parse(input).unwrap().1;

        assert_eq!(packet.eval(), 9);
    }

    #[test]
    fn test_packet_eval_5() {
        let data = hex::decode("D8005AC2A8F0").unwrap();
        let input = (data.as_slice(), 0);
        let packet = Packet::parse(input).unwrap().1;

        assert_eq!(packet.eval(), 1);
    }

    #[test]
    fn test_packet_eval_6() {
        let data = hex::decode("F600BC2D8F").unwrap();
        let input = (data.as_slice(), 0);
        let packet = Packet::parse(input).unwrap().1;

        assert_eq!(packet.eval(), 0);
    }

    #[test]
    fn test_packet_eval_7() {
        let data = hex::decode("9C005AC2F8F0").unwrap();
        let input = (data.as_slice(), 0);
        let packet = Packet::parse(input).unwrap().1;

        assert_eq!(packet.eval(), 0);
    }

    #[test]
    fn test_packet_eval_8() {
        let data = hex::decode("9C0141080250320F1802104A08").unwrap();
        let input = (data.as_slice(), 0);
        let packet = Packet::parse(input).unwrap().1;

        assert_eq!(packet.eval(), 1);
    }

    #[bench]
    fn bench_solution(b: &mut test::Bencher) {
        let input = get_input().unwrap();
        b.iter(|| solve(&input))
    }

    #[bench]
    fn bench_get_input(b: &mut test::Bencher) {
        b.iter(|| get_input());
    }
}
