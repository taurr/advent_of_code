use anyhow::{Context, Result};
use csv::StringRecord;
use itertools::*;
use serde::Deserialize;
use std::path::PathBuf;
use std::{fs::File, path::Path};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Args {
    input: PathBuf,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Input {
    depth: f64,
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;
    let input = read_csv(args.input.as_path())?;

    println!("Day1, Part1: {}", day1_part1(&input));
    println!("Day1, Part2: {}", day1_part2(&input));

    Ok(())
}

fn day1_part1(input: &[Input]) -> usize {
    input
        .iter()
        .tuple_windows()
        .filter(|(a, b)| b.depth > a.depth)
        .count()
}

fn day1_part2(input: &[Input]) -> usize {
    input
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a.depth + b.depth + c.depth)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

fn read_csv(file_path: &Path) -> Result<Vec<Input>> {
    let mut result = vec![];
    let reader = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(reader);
    rdr.set_headers(StringRecord::from(vec!["depth"]));

    for (line, record) in rdr.deserialize().enumerate() {
        let record: Input = record.context(line)?;
        result.push(record);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::{fs::File, io::Write};

    fn create_input(inp: &[&str]) -> Result<PathBuf> {
        let mut pb = std::env::temp_dir();
        pb.push("csvfile.csv");
        let path = pb.as_path();
        let mut file = File::create(path)?;
        for l in inp {
            writeln!(file, "{}", l)?;
        }
        drop(file);
        Ok(pb)
    }

    #[test]
    fn can_read_input() -> Result<()> {
        let path = create_input(&["123", "456"])?;
        assert_eq!(
            vec![Input { depth: 123.0_f64 }, Input { depth: 456.0_f64 },],
            read_csv(path.as_path())?
        );
        Ok(())
    }

    #[test]
    fn example_day1_part1() -> Result<()> {
        let path = create_input(&[
            "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
        ])?;
        let input = read_csv(path.as_path())?;
        assert_eq!(7, day1_part1(&input));
        Ok(())
    }

    #[test]
    fn example_day1_part2() -> Result<()> {
        let path = create_input(&[
            "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
        ])?;
        let input = read_csv(path.as_path())?;
        assert_eq!(5, day1_part2(&input));
        Ok(())
    }
}
