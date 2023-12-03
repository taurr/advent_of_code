use day_02::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(
        divan::black_box(include_str!("../input.txt",)),
        divan::black_box(CubeCollection::new(12, 13, 14)),
    )
    .unwrap();
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(include_str!("../input.txt",))).unwrap();
}
