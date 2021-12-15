use anyhow::{Context, Result};
use csv::StringRecord;
#[cfg(feature = "profile")]
use dhat::{Dhat, DhatAlloc};
use serde::Deserialize;
use std::path::PathBuf;
use std::{fs::File, path::Path};
use structopt::StructOpt;

#[cfg(feature = "profile")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day15;

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(subcommand)]
    puzzle: Day,
    input: Option<PathBuf>,
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
    #[cfg(feature = "profile")]
    let _dhat = Dhat::start_heap_profiling();

    let args = Args::from_args_safe()?;
    match args.puzzle {
        Day::Day1 => day1::solve_puzzle(args.input.unwrap().as_path()),
        Day::Day2 => day2::solve_puzzle(args.input.unwrap().as_path()),
        Day::Day3 => day3::solve_puzzle(args.input.unwrap().as_path()),
        Day::Day4 => day4::solve_puzzle(args.input.unwrap().as_path()),
        Day::Day5 => day5::solve_puzzle(args.input.unwrap().as_path()),
        Day::Day6 => day6::solve_puzzle(args.input.unwrap().as_path()),
        Day::Day7 => day7::solve_puzzle(args.input.unwrap().as_path()),
        Day::Day8 => day8::solve_puzzle(args.input.unwrap().as_path()),
        Day::Day9 => day9::solve_puzzle(args.input.unwrap().as_path()),
        Day::Day10 => day10::solve_puzzle(args.input.unwrap().as_path()),
        Day::Day11 => day11::solve_puzzle(),
        Day::Day12 => day12::solve_puzzle(),
        Day::Day13 => day13::solve_puzzle(),
        Day::Day14 => day14::solve_puzzle(),
        Day::Day15 => day15::solve_puzzle(),
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
