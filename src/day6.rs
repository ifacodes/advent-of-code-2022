use anyhow::{anyhow, Result};
use std::io::{BufRead, BufReader, Error, Read};

pub fn day_6(io: BufReader<impl Read>) -> Result<()> {
    let buffer = io.lines().take(1).collect::<Result<String, Error>>()?;
    let (part_1, part_2) = [4, 14]
        .iter()
        .map(|n| {
            buffer
                .as_bytes()
                .windows(*n)
                .position(|x| !(1..x.len()).any(|i| x[i..].contains(&x[i - 1])))
                .expect("Couldn't find the index!")
                + n
        })
        .fold((0, 0), |mut acc, a| {
            if acc.0 == 0 {
                acc.0 = a
            } else {
                acc.1 = a
            }
            acc
        });

    println!("Day 6 - Part 1: {part_1}\nDay 6 - Part 2: {part_2}");
    Ok(())
}
