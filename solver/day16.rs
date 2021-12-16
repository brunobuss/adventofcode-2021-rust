use solver::CompositeSolution;
use std::fmt::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day16Solver {}

impl Day16Solver {}

impl super::Solver for Day16Solver {
    fn solve_both(
        &self,
        reader_provider: &dyn Fn() -> BufReader<File>,
    ) -> Result<CompositeSolution, String> {
        let reader = reader_provider();

        for line in reader.lines() {
            let line_content = match line {
                Ok(l) => l,
                Err(e) => return Err(e.to_string()),
            };

            let mut binary_input = String::new();
            for c in line_content.chars() {
                let d = c.to_digit(16).unwrap() as u8;
                write!(&mut binary_input, "{:04b}", d).unwrap();
            }

            let (packet, _) = Packet::from(&binary_input);
            return Ok(CompositeSolution(
                packet.version_sum().to_string(),
                packet.compute().to_string(),
            ));
        }
        Err("NoInput".to_string())
    }
}

#[derive(Debug)]
enum PacketContent {
    LiteralValue(u64),
    Operator(Vec<Packet>),
}

#[derive(Debug)]
enum OperatorLength {
    Bits(usize),
    Packets(usize),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    content: PacketContent,
}

impl Packet {
    fn version_sum(&self) -> u64 {
        match &self.content {
            PacketContent::LiteralValue(_) => self.version as u64,
            PacketContent::Operator(pks) => {
                pks.iter().map(|p| p.version_sum()).sum::<u64>() + (self.version as u64)
            }
        }
    }

    fn compute(&self) -> u64 {
        match &self.content {
            PacketContent::LiteralValue(v) => *v,
            PacketContent::Operator(pks) => match &self.type_id {
                // Sum
                0 => pks.iter().map(|p| p.compute()).sum::<u64>(),
                // Product
                1 => pks.iter().map(|p| p.compute()).product::<u64>(),
                // Min
                2 => pks.iter().map(|p| p.compute()).min().unwrap(),
                // Max
                3 => pks.iter().map(|p| p.compute()).max().unwrap(),
                // GT
                5 => {
                    if pks[0].compute() > pks[1].compute() {
                        1
                    } else {
                        0
                    }
                }
                // LT
                6 => {
                    if pks[0].compute() < pks[1].compute() {
                        1
                    } else {
                        0
                    }
                }
                // EQ
                7 => {
                    if pks[0].compute() == pks[1].compute() {
                        1
                    } else {
                        0
                    }
                }
                _ => 0,
            },
        }
    }

    fn from(binary_string: &str) -> (Packet, usize) {
        let version = u8::from_str_radix(&binary_string[0..3], 2).unwrap();
        let type_id = u8::from_str_radix(&binary_string[3..6], 2).unwrap();

        let (content, used) = match type_id {
            4 => Packet::parse_literal(&binary_string[6..]),
            _ => Packet::parse_operator(&binary_string[6..]),
        };

        (
            Packet {
                version: version,
                type_id: type_id,
                content: content,
            },
            used + 6,
        )
    }

    fn parse_literal(binary_string: &str) -> (PacketContent, usize) {
        let mut consumed_bytes = 0;
        let mut literal_string = String::new();
        let mut remaining = &binary_string[..];

        while !remaining.is_empty() {
            let cont = &remaining[0..1].chars().next().unwrap();
            let chunk = &remaining[1..5];
            literal_string.push_str(chunk);
            consumed_bytes += 5;
            if *cont == '0' {
                break;
            }
            remaining = &remaining[5..];
        }

        let literal_value = u64::from_str_radix(&literal_string[..], 2).unwrap();
        (PacketContent::LiteralValue(literal_value), consumed_bytes)
    }

    fn parse_operator(binary_string: &str) -> (PacketContent, usize) {
        let mut consumed_bytes = 0;
        let mut sub_packets: Vec<Packet> = Vec::new();
        let mut remaining = &binary_string[..];

        let (length, l_consumed) = Packet::parse_operator_length(remaining);
        consumed_bytes += l_consumed;
        remaining = &remaining[l_consumed..];

        match length {
            OperatorLength::Bits(b) => {
                let mut sub_packets_bits_consumed = 0;
                while sub_packets_bits_consumed < b {
                    let (sub_packet, sub_used) = Packet::from(remaining);
                    sub_packets.push(sub_packet);
                    consumed_bytes += sub_used;
                    remaining = &remaining[sub_used..];
                    sub_packets_bits_consumed += sub_used;
                }
            }
            OperatorLength::Packets(c) => {
                let mut remaining_packets = c;
                while remaining_packets > 0 {
                    let (sub_packet, sub_used) = Packet::from(remaining);
                    sub_packets.push(sub_packet);
                    consumed_bytes += sub_used;
                    remaining = &remaining[sub_used..];
                    remaining_packets -= 1;
                }
            }
        }

        (PacketContent::Operator(sub_packets), consumed_bytes)
    }

    fn parse_operator_length(binary_string: &str) -> (OperatorLength, usize) {
        let mut remaining = &binary_string[..];
        let length_type = &remaining[0..1].chars().next().unwrap();
        remaining = &remaining[1..];

        if *length_type == '0' {
            let bits = usize::from_str_radix(&remaining[0..15], 2).unwrap();
            (OperatorLength::Bits(bits), 16)
        } else {
            let count = usize::from_str_radix(&remaining[0..11], 2).unwrap();
            (OperatorLength::Packets(count), 12)
        }
    }
}
