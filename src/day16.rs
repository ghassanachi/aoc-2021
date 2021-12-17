use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct BitStream {
    bit_stream: VecDeque<u8>,
    remainder: Vec<bool>,
}

impl BitStream {
    fn from_hex(input: &str) -> Result<Self, std::num::ParseIntError> {
        let bit_stream = (0..input.len())
            .step_by(2)
            .map(|i| {
                let pair = if i + 1 < input.len() {
                    input[i..=i + 1].to_owned()
                } else {
                    input[i..i + 1].to_owned() + "0"
                };
                u8::from_str_radix(&pair, 16)
            })
            .collect::<Result<VecDeque<u8>, std::num::ParseIntError>>()?;
        Ok(Self {
            bit_stream,
            remainder: Vec::new(),
        })
    }

    fn len(&self) -> usize {
        self.bit_stream.len() * 8 + self.remainder.len()
    }

    fn read(&mut self, amount: u8) -> u32 {
        assert!(amount <= 32, "cannot read more than 32 bits at a time");
        assert!(
            amount as usize <= self.len(),
            "amount is greater than stream length"
        );
        let mut acc = 0;
        for _ in 0..amount {
            if self.remainder.is_empty() {
                let next_byte = self.bit_stream.pop_front().unwrap();
                for pos in 0..8 {
                    self.remainder.push(next_byte & (1 << pos) != 0)
                }
            }

            acc = (acc << 1) | (self.remainder.pop().unwrap() as u32)
        }
        acc
    }
}

#[derive(Debug)]
struct Packet {
    version: u8,
    packet_type: Type,
}

#[derive(Debug)]
enum Type {
    Literal(u64),
    Operator { children: Vec<Packet>, op: Op },
}

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Sum,
    Product,
    Min,
    Max,
    Greater,
    Less,
    Equal,
}

impl Packet {
    fn new(version: u8, packet_type: Type) -> Self {
        Self {
            version,
            packet_type,
        }
    }
    fn from_stream(bits: &mut BitStream) -> Self {
        let version = bits.read(3) as u8;
        let id = bits.read(3) as u8;
        if id == 4 {
            let mut acc = 0 as u64;
            loop {
                let next_bits = bits.read(5) as u64;
                let is_last = next_bits & (1 << 4) == 0;
                for pos in (0..4).rev() {
                    if (next_bits & (1 << pos)) != 0 {
                        acc = (acc << 1) | 1
                    } else {
                        acc <<= 1
                    }
                }
                if is_last {
                    return Self::new(version, Type::Literal(acc));
                }
            }
        }

        let mut children = Vec::new();
        let len_type_id = bits.read(1);

        if len_type_id == 0 {
            let children_len = bits.read(15) as usize;
            let stream_len_before = bits.len();
            'outer: loop {
                let bits_read = stream_len_before - bits.len();
                match bits_read.cmp(&children_len) {
                    std::cmp::Ordering::Equal => break 'outer,
                    std::cmp::Ordering::Greater => panic!("invalid packet"),
                    _ => {}
                }
                children.push(Self::from_stream(bits));
            }
        } else {
            let num_children = bits.read(11) as usize;
            for _ in 0..num_children {
                children.push(Self::from_stream(bits));
            }
        }

        let op = match id {
            0 => Op::Sum,
            1 => Op::Product,
            2 => Op::Min,
            3 => Op::Max,
            5 => Op::Greater,
            6 => Op::Less,
            7 => Op::Equal,
            _ => panic!("invalid op"),
        };

        return Self::new(version, Type::Operator { children, op });
    }

    fn version_sum(&self) -> u64 {
        match &self.packet_type {
            Type::Literal(_) => self.version as u64,
            Type::Operator { children, .. } => {
                let sum: u64 = children.iter().map(|c| c.version_sum()).sum();
                sum + self.version as u64
            }
        }
    }

    fn value(&self) -> u64 {
        match &self.packet_type {
            Type::Literal(val) => *val,
            Type::Operator { children, op } => match op {
                Op::Sum => children.iter().map(|c| c.value()).sum(),
                Op::Product => children.iter().map(|c| c.value()).product(),
                Op::Min => children.iter().map(|c| c.value()).min().unwrap_or(0),
                Op::Max => children.iter().map(|c| c.value()).max().unwrap_or(0),
                _ => {
                    let valid = match children[0].value().cmp(&children[1].value()) {
                        std::cmp::Ordering::Less => *op == Op::Less,
                        std::cmp::Ordering::Equal => *op == Op::Equal,
                        std::cmp::Ordering::Greater => *op == Op::Greater,
                    };
                    if valid {
                        1
                    } else {
                        0
                    }
                }
            },
        }
    }
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> BitStream {
    BitStream::from_hex(input).expect("input should be valid")
}

#[aoc(day16, part1, day16_1)]
pub fn part1(bits: &BitStream) -> u64 {
    let mut bits = bits.clone();
    let packet = Packet::from_stream(&mut bits);
    packet.version_sum()
}

#[aoc(day16, part2, day16_2)]
pub fn part2(bits: &BitStream) -> u64 {
    let mut bits = bits.clone();
    let packet = Packet::from_stream(&mut bits);
    packet.value()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLES_1: [(&str, u64); 5] = [
        ("D2FE28", 6),
        ("8A004A801A8002F478", 16),
        ("620080001611562C8802118E34", 12),
        ("C0015000016115A2E0802F182340", 23),
        ("A0016C880162017C3686B18A3D4780", 31),
    ];

    const SAMPLES_2: [(&str, u64); 9] = [
        ("D2FE28", 2021),
        ("C200B40A82", 3),
        ("04005AC33890", 54),
        ("880086C3E88112", 7),
        ("CE00C43D881120", 9),
        ("D8005AC2A8F0", 1),
        ("F600BC2D8F", 0),
        ("9C005AC2F8F0", 0),
        ("9C0141080250320F1802104A08", 1),
    ];

    #[test]
    fn test_part_1() {
        for (hex, expected) in SAMPLES_1 {
            let input = input_generator(hex);
            assert_eq!(part1(&input), expected)
        }
    }

    #[test]
    fn test_part_2() {
        for (hex, expected) in SAMPLES_2 {
            let input = input_generator(hex);
            assert_eq!(part2(&input), expected)
        }
    }
}
