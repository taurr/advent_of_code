use anyhow::{anyhow, Result};
use derive_more::*;
use itertools::Itertools;
use nom::{bits::complete::take, IResult};
use std::cmp::Ordering;

#[derive(Debug, From, Into, PartialEq, Eq)]
pub(crate) struct Packet {
    pub(crate) version: Version,
    pub(crate) type_id: TypeId,
    pub(crate) content: PacketContent,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum PacketContent {
    Literal(Literal),
    Operator(Vec<Packet>),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Add, Sub, Mul, Div)]
pub(crate) struct Literal(pub usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Add)]
pub(crate) struct Version(pub u8);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct TypeId(pub u8);

impl Packet {
    pub fn value(&self) -> usize {
        match &self.content {
            PacketContent::Literal(lit) => lit.0,
            PacketContent::Operator(sub_packets) => match self.type_id.0 {
                0 => sub_packets.iter().map(|p| p.value()).sum(),
                1 => sub_packets.iter().map(|p| p.value()).product(),
                2 => sub_packets.iter().map(|p| p.value()).min().unwrap(),
                3 => sub_packets.iter().map(|p| p.value()).max().unwrap(),
                5 => {
                    if sub_packets[0].value() > sub_packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if sub_packets[0].value() < sub_packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if sub_packets[0].value() == sub_packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("unexpected type_id"),
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Add, AddAssign, Sub, SubAssign)]
struct BitsRead(usize);

const LITERAL_TYPE_ID: TypeId = TypeId(4);

type BitInput<'a> = (&'a [u8], usize);
type BitOutput<'a, T> = IResult<BitInput<'a>, (T, BitsRead)>;

pub(crate) fn parse_input(input: &str) -> Result<Packet> {
    let bytes = hex_chars_to_u8(input);
    let (_, (packet, _)) = packet((&bytes[..], 0)).map_err(|e| anyhow!(e.to_string()))?;
    Ok(packet)
}

fn hex_chars_to_u8(input: &str) -> Vec<u8> {
    input
        .trim()
        .chars()
        .chunks(2)
        .into_iter()
        .filter_map(|itt| {
            itt.filter_map(|c| c.to_digit(16).map(|v| v as u8))
                .reduce(|b, v| b << 4 | v)
        })
        .collect()
}

fn packet(i: BitInput) -> BitOutput<Packet> {
    let (i, (version, vr)) = version(i)?;
    let (i, (type_id, tr)) = type_id(i)?;
    let mut bits = vr + tr;

    if type_id == LITERAL_TYPE_ID {
        let (i, (literal, literal_bits)) = literal(i)?;
        bits += literal_bits;
        Ok((
            i,
            (
                Packet {
                    version,
                    type_id,
                    content: PacketContent::Literal(literal),
                },
                bits,
            ),
        ))
    } else {
        let (i, (sub_packets, sub_packet_bits)) = operator(i)?;
        bits += sub_packet_bits;
        Ok((
            i,
            (
                Packet {
                    version,
                    type_id,
                    content: PacketContent::Operator(sub_packets),
                },
                bits,
            ),
        ))
    }
}

fn operator(i: BitInput) -> BitOutput<Vec<Packet>> {
    let (i, length_type_id): (_, u8) = take(1usize)(i)?;
    if length_type_id == 0 {
        // next 15 bits are the number of bits in the sub-packets
        let (mut i, bits_in_subpackets) = take(15usize)(i)?;
        let mut bits_read = BitsRead(16);

        let mut sub_packets = Vec::new();
        let mut bits_left_in_subpackets = BitsRead(bits_in_subpackets);

        loop {
            let (i2, (sub_packet, bits_in_sub_packet)) = packet(i)?;
            bits_read += bits_in_sub_packet;
            i = i2;
            sub_packets.push(sub_packet);
            match bits_left_in_subpackets.cmp(&bits_in_sub_packet) {
                Ordering::Less => panic!("To few bits - read too much"),
                Ordering::Equal => break,
                Ordering::Greater => {}
            }
            bits_left_in_subpackets -= bits_in_sub_packet;
        }
        Ok((i, (sub_packets, bits_read)))
    } else {
        // next 11 bits are the number of sub-packets
        let (mut i, number_of_sub_packets) = take(11usize)(i)?;
        let mut bits_read = BitsRead(12);
        let mut sub_packets = Vec::with_capacity(number_of_sub_packets);

        for _ in 0..number_of_sub_packets {
            let (i2, (sub_packet, bits_in_sub_packet)) = packet(i)?;
            sub_packets.push(sub_packet);
            bits_read += bits_in_sub_packet;
            i = i2;
        }
        Ok((i, (sub_packets, bits_read)))
    }
}

fn literal(mut i: BitInput) -> BitOutput<Literal> {
    let mut bits_read = BitsRead(0);
    let mut n: usize = 0;

    let n = loop {
        bits_read += BitsRead(5);
        let (i2, t): (_, u8) = take(1usize)(i)?;
        if t == 1 {
            let (i2, partial_bits): (_, usize) = take(4usize)(i2)?;
            n = n << 4 | partial_bits;
            i = i2;
        } else {
            let (i2, partial_bits): (_, usize) = take(4usize)(i2)?;
            n = n << 4 | partial_bits;
            i = i2;
            break n;
        }
    };

    Ok((i, (Literal(n), bits_read)))
}

fn version(i: BitInput) -> BitOutput<Version> {
    let (i, v) = take(3usize)(i)?;
    Ok((i, (Version(v), BitsRead(3))))
}

fn type_id(i: BitInput) -> BitOutput<TypeId> {
    let (i, v) = take(3usize)(i)?;
    Ok((i, (TypeId(v), BitsRead(3))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet1() {
        let bytes = hex_chars_to_u8("D2FE28");
        assert!(matches!(
            packet((&bytes[..], 0)),
            Ok((
                _,
                (
                    Packet {
                        version: Version(6),
                        type_id: TypeId(4),
                        content: PacketContent::Literal(Literal(2021)),
                    },
                    BitsRead(21)
                )
            ))
        ));
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_packet_EE00D40C823060() {
        let bytes = hex_chars_to_u8("EE00D40C823060");
        let (_, (result, _)) = packet((&bytes[..], 0)).unwrap();
        println!("{:#?}", result);
        assert_eq!(result.version, Version(7));
        assert_eq!(result.type_id, TypeId(3));
        match result.content {
            PacketContent::Literal(_) => panic!("unexpected literal"),
            PacketContent::Operator(op) => {
                assert_eq!(op.len(), 3);
                assert!(matches!(
                    op[0],
                    Packet {
                        content: PacketContent::Literal(Literal(1)),
                        ..
                    }
                ));
                assert!(matches!(
                    op[1],
                    Packet {
                        content: PacketContent::Literal(Literal(2)),
                        ..
                    }
                ));
                assert!(matches!(
                    op[2],
                    Packet {
                        content: PacketContent::Literal(Literal(3)),
                        ..
                    }
                ));
            }
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_packet_8A004A801A8002F478() {
        let bytes = hex_chars_to_u8("8A004A801A8002F478");
        let (_, (result, _)) = packet((&bytes[..], 0)).unwrap();
        println!("{:#?}", result);
        assert_eq!(result.version, Version(4));
        if let PacketContent::Operator(ref op) = result.content {
            let op = &op[0];
            assert_eq!(op.version, Version(1));
            if let PacketContent::Operator(ref op) = op.content {
                let op = &op[0];
                assert_eq!(op.version, Version(5));
                if let PacketContent::Operator(ref op) = op.content {
                    let op = &op[0];
                    assert_eq!(op.version, Version(6));
                    match op.content {
                        PacketContent::Literal(_) => {}
                        _ => {
                            panic!("expected operator");
                        }
                    }
                } else {
                    panic!("expected operator");
                }
            } else {
                panic!("expected operator");
            }
        } else {
            panic!("expected operator");
        }
    }
}
