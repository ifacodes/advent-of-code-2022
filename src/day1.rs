use anyhow::Result;
use std::io::{BufRead, BufReader, Read};

pub fn day_1(io: BufReader<impl Read>) -> Result<()> {
    let mut calories: Vec<u32> = io
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
    println!("Day 1 - Part 1: {largest}");

    let top_three: u32 = calories.as_slice()[calories.len() - 3..].iter().sum();
    println!("Day 1 - Part 2: {top_three}");

    Ok(())
}
