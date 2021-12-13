use anyhow::Result;

pub fn solve_puzzle() -> Result<()> {
    const INPUT: &str = include_str!("../assets/day.txt");
    println!("Part1: {}", part1(parse_input(INPUT)?)?);
    println!("Part2: {}", part2(parse_input(INPUT)?)?);
    Ok(())
}

fn parse_input(input: &'static str) -> Result<Vec<&'static str>> {
    todo!()
}

fn part1(input: Vec<&'static str>) -> Result<usize> {
    todo!()
}

fn part2(input: Vec<&'static str>) -> Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
    "};

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(0, part1(parse_input(INPUT)?)?);
        Ok(())
    }
}
