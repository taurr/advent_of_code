use advent_of_code_2021::*;
use anyhow::Result;
use itertools::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Args {
    input: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;

    let input = read_csv::<Day1>(args.input.as_path())?;
    let count = input
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a.depth + b.depth + c.depth)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count();

    println!("Answer: {}", count);

    Ok(())
}
