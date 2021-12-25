use self::parser::*;
use anyhow::Result;

mod parser;

pub fn solve_puzzle() -> Result<()> {
    const INPUT: &str = include_str!("../assets/day16.txt");
    println!("Part1: {}", part1(parse_input(INPUT)?)?);
    println!("Part2: {}", part2(parse_input(INPUT)?)?);
    Ok(())
}

fn part1(input: Packet) -> Result<usize> {
    fn version_sum(packet: &Packet) -> usize {
        packet.version.0 as usize
            + match packet.content {
                parser::PacketContent::Literal(_) => 0,
                parser::PacketContent::Operator(ref sub_packets) => {
                    sub_packets.iter().map(version_sum).sum()
                }
            }
    }
    Ok(version_sum(&input))
}

fn part2(input: Packet) -> Result<usize> {
    Ok(input.value())
}

#[cfg(test)]
mod tests {
    use super::parser::*;
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn test_part1_8A004A801A8002F478() -> Result<()> {
        let parsed = parse_input("8A004A801A8002F478")?;
        let result = part1(parsed)?;
        assert_eq!(result, 16);
        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_part1_620080001611562C8802118E34() -> Result<()> {
        let parsed = parse_input("620080001611562C8802118E34")?;
        let result = part1(parsed)?;
        assert_eq!(result, 12);
        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_part1_C0015000016115A2E0802F182340() -> Result<()> {
        let parsed = parse_input("C0015000016115A2E0802F182340")?;
        let result = part1(parsed)?;
        assert_eq!(result, 23);
        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_part1_A0016C880162017C3686B18A3D47800() -> Result<()> {
        let parsed = parse_input("A0016C880162017C3686B18A3D47800")?;
        let result = part1(parsed)?;
        assert_eq!(result, 31);
        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_part2_C200B40A82() -> Result<()> {
        let parsed = parse_input("C200B40A82")?;
        let result = part2(parsed)?;
        assert_eq!(result, 3);
        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_part2_04005AC33890() -> Result<()> {
        let parsed = parse_input("04005AC33890")?;
        let result = part2(parsed)?;
        assert_eq!(result, 54);
        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_part2_880086C3E88112() -> Result<()> {
        let parsed = parse_input("880086C3E88112")?;
        let result = part2(parsed)?;
        assert_eq!(result, 7);
        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_part2_CE00C43D881120() -> Result<()> {
        let parsed = parse_input("CE00C43D881120")?;
        let result = part2(parsed)?;
        assert_eq!(result, 9);
        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_part2_D8005AC2A8F0() -> Result<()> {
        let parsed = parse_input("D8005AC2A8F0")?;
        let result = part2(parsed)?;
        assert_eq!(result, 1);
        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_part2_F600BC2D8F() -> Result<()> {
        let parsed = parse_input("F600BC2D8F")?;
        let result = part2(parsed)?;
        assert_eq!(result, 0);
        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_part2_9C005AC2F8F0() -> Result<()> {
        let parsed = parse_input("9C005AC2F8F0")?;
        let result = part2(parsed)?;
        assert_eq!(result, 0);
        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_part2_9C0141080250320F1802104A08() -> Result<()> {
        let parsed = parse_input("9C0141080250320F1802104A08")?;
        let result = part2(parsed)?;
        assert_eq!(result, 1);
        Ok(())
    }
}
