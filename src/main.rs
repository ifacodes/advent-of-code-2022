use anyhow::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    day_1()?;

    Ok(())
}

fn day_1() -> Result<()> {
    let br = BufReader::new(File::open("./input/day_1")?);

    let mut calories: Vec<u32> = br
        .lines()
        .map(|line| line.ok().and_then(|s| s.parse::<u32>().ok()))
        .fold(vec![0], |mut acc, calorie| {
            if let Some(n) = calorie {
                if let Some(x) = acc.last_mut() {
                    *x += n
                }
            } else {
                acc.push(0)
            }
            acc
        });

    calories.sort();

    let largest = calories.last().unwrap();
    println!("{largest}");

    let top_three: u32 = calories.as_slice()[calories.len() - 3..].iter().sum();
    println!("{top_three}");

    Ok(())
}
