use anyhow::Result;

pub fn solve_puzzle() -> Result<()> {
    println!("Part1: {}", part1()?);
    println!("Part2: {}", part2()?);
    Ok(())
}

fn part1() -> Result<i32> {
    // Due to preservation of energy, whatever speed by which we initially throw upwards, our
    // probe will have that same speed when it comes back to y-position 0.
    // thus, at the next time interval, the y speed will be initial-y + 1. This number must not
    // overshoot our target.

    let target_min_y = -102;
    Ok((0..=(-target_min_y - 1)).sum())
}

fn part2() -> Result<usize> {
    fn step(speed: (i32, i32), pos: (i32, i32)) -> Result<(), ()> {
        if pos.0 > 155 || pos.1 < -102 {
            return Err(());
        }
        if pos.0 >= 135 && pos.0 <= 155 && pos.1 <= -78 && pos.1 >= -102 {
            return Ok(());
        }

        let pos = (pos.0 + speed.0, pos.1 + speed.1);
        let speed = (if speed.0 > 0 { speed.0 - 1 } else { 0 }, speed.1 - 1);
        step(speed, pos)
    }

    let mut count = 0;
    for x in 0..=155 {
        for y in -102..=101 {
            if step((x, y), (0, 0)).is_ok() {
                count += 1;
            }
        }
    }
    Ok(count)
}
