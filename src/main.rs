use anyhow::{Context, Result};
use csv::StringRecord;
use serde::Deserialize;
use std::path::PathBuf;
use std::{fs::File, path::Path};
use structopt::StructOpt;

mod day1;
mod day2;
mod day3;

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(subcommand)]
    puzzle: Day,
    input: PathBuf,
}

#[derive(Debug, StructOpt)]
enum Day {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;
    match args.puzzle {
        Day::Day1 => day1::solve_puzzle(args.input.as_path()),
        Day::Day2 => day2::solve_puzzle(args.input.as_path()),
        Day::Day3 => day3::solve_puzzle(args.input.as_path()),
        Day::Day4 => todo!(),
        Day::Day5 => todo!(),
        Day::Day6 => todo!(),
        Day::Day7 => todo!(),
        Day::Day8 => todo!(),
        Day::Day9 => todo!(),
        Day::Day10 => todo!(),
        Day::Day11 => todo!(),
        Day::Day12 => todo!(),
        Day::Day13 => todo!(),
        Day::Day14 => todo!(),
        Day::Day15 => todo!(),
        Day::Day16 => todo!(),
        Day::Day17 => todo!(),
        Day::Day18 => todo!(),
        Day::Day19 => todo!(),
        Day::Day20 => todo!(),
        Day::Day21 => todo!(),
        Day::Day22 => todo!(),
        Day::Day23 => todo!(),
        Day::Day24 => todo!(),
        Day::Day25 => todo!(),
    }
}

fn read_csv<T>(file_path: &Path, headers: &[&str], delimiter: u8) -> Result<Vec<T>>
where
    for<'de> T: Deserialize<'de>,
{
    let mut result = vec![];
    let reader = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::default()
        .delimiter(delimiter)
        .trim(csv::Trim::All)
        .from_reader(reader);
    rdr.set_headers(StringRecord::from(headers));

    for (line, record) in rdr.deserialize().enumerate() {
        let record: T = record.context(line)?;
        result.push(record);
    }
    Ok(result)
}

#[cfg(test)]
fn create_input(dir: &tempfile::TempDir, lines: &[&str]) -> Result<PathBuf> {
    let mut pb = PathBuf::from(dir.path());
    pb.push("csvfile.csv");
    let path = pb.as_path();
    let mut file = File::create(path)?;
    for l in lines {
        use std::io::Write;
        writeln!(file, "{}", l)?;
    }
    drop(file);
    Ok(pb)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestInput {
        val: f64,
    }

    #[test]
    fn can_read_input() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let path = create_input(&dir, &["123", "456"])?;
        assert_eq!(
            vec![TestInput { val: 123.0_f64 }, TestInput { val: 456.0_f64 },],
            read_csv(path.as_path(), &["val"], b'\t')?
        );
        Ok(())
    }
}
