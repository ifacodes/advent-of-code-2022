use anyhow::{anyhow, Result};
use std::io::{BufRead, BufReader, Error, Read};

pub fn day_6(io: BufReader<impl Read>) -> Result<()> {
    let buffer = io.lines().take(1).collect::<Result<String, Error>>()?;
    let part_1 = buffer
        .as_bytes()
        .windows(4)
        .position(|x| !(1..x.len()).any(|i| x[i..].contains(&x[i - 1])))
        .ok_or_else(|| anyhow!("Couldn't find the index!"))?
        + 4;

    let part_2 = buffer
        .as_bytes()
        .windows(14)
        .position(|x| !(1..x.len()).any(|i| x[i..].contains(&x[i - 1])))
        .ok_or_else(|| anyhow!("Couldn't find the index!"))?
        + 14;

    println!("Day 6 - Part 1: {part_1}\nDay 6 - Part 2: {part_2}");
    Ok(())
}
