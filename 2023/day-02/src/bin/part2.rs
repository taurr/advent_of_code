use anyhow::{Context, Result};
use day_02::part2::process;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument(level = "trace")]
fn main() -> Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    #[cfg(not(feature = "dhat-heap"))]
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input.txt");
    let result = process(file).context("process part 2")?;
    println!("{}", result);
    Ok(())
}
