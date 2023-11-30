use std::collections::HashMap;
use nom::{
    bytes::complete::tag,
    character::complete::*,
    multi::{fold_many0, many1},
    IResult,
};

pub(crate) struct ParsedInput {
    pub(crate) start: Vec<char>,
    pub(crate) rules: HashMap<(char, char), char>,
}

pub(crate) fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let (input, start) = many1(genome)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, rules) = fold_many0(rule, HashMap::new, |mut ans, (key, subst)| {
        ans.insert(key, subst);
        ans
    })(input)?;

    Ok((input, ParsedInput { start, rules }))
}

fn genome(input: &str) -> IResult<&str, char> {
    one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")(input)
}

fn rule(input: &str) -> IResult<&str, ((char, char), char)> {
    let (input, key1) = genome(input)?;
    let (input, key2) = genome(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, subst) = genome(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, ((key1, key2), subst)))
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;

    const INPUT: &str = indoc::indoc! {"
            NNCB

            CH -> B
            HH -> N
            CB -> H
            NH -> C
            HB -> C
            HC -> B
            HN -> C
            NN -> C
            BH -> H
            NC -> B
            NB -> B
            BN -> B
            BB -> N
            BC -> B
            CC -> N
            CN -> C
        "};

    #[test]
    fn parse_ok() -> Result<()> {
        let (_, parsed) = parse(INPUT)?;
        assert_eq!(parsed.start, vec!['N', 'N', 'C', 'B']);
        assert_eq!(parsed.rules.get(&('H', 'B')).unwrap(), &'C');
        assert_eq!(parsed.rules.get(&('N', 'C')).unwrap(), &'B');
        assert_eq!(parsed.rules.get(&('C', 'N')).unwrap(), &'C');
        Ok(())
    }

    #[test]
    fn parse_rules() -> Result<()> {
        const INPUT: &str = include_str!("../../assets/day14.txt");
        let (_, parsed) = parse(INPUT)?;
        assert_eq!(parsed.rules.get(&('P', 'H')).unwrap(), &'H');
        Ok(())
    }
}
