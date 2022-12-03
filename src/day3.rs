use std::io::{BufRead, BufReader, Read};

use anyhow::{anyhow, Result};

fn sum_priorities(items: &[char]) -> u32 {
    items
        .iter()
        .map(|c| match c {
            'a'..='z' => ((c.to_ascii_uppercase() as u8) - 64) as u32,
            'A'..='Z' => ((c.to_ascii_uppercase() as u8) - 38) as u32,
            _ => unreachable!(),
        })
        .sum::<u32>()
}

pub fn day_3(io: BufReader<impl Read>) -> Result<()> {
    let rucksacks: Result<Vec<String>> = io
        .lines()
        .map(|line| line.map_err(|e| anyhow!("unable to parse line! {}", e)))
        .collect();

    let rucksacks = rucksacks?;

    let part_1: Result<Vec<char>> = rucksacks
        .iter()
        .map(|rucksack| {
            let (comp_a, comp_b) = rucksack.split_at(rucksack.len() / 2);
            comp_a
                .chars()
                .find_map(|a| comp_b.chars().find(|b| a == *b))
                .ok_or_else(|| anyhow!("couldn't find a matching char!"))
        })
        .collect();

    println!("Day 3 - Part 1: {:?}", sum_priorities(&part_1?));

    let part_2: Result<Vec<char>> = rucksacks
        .chunks_exact(3)
        .map(|chunks| {
            chunks[0]
                .chars()
                .find_map(|a| {
                    chunks[1]
                        .chars()
                        .find_map(|b| chunks[2].chars().find(|c| a == b && b == *c))
                })
                .ok_or_else(|| anyhow!("couldn't find a matching char!"))
        })
        .collect();

    println!("Day 3 - Part 2: {:?}", sum_priorities(&part_2?));

    Ok(())
}
