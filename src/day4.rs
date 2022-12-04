use anyhow::{anyhow, bail, Result};
use std::io::{BufRead, BufReader, Read};

pub fn day_4(io: BufReader<impl Read>) -> Result<()> {
    let pairs: Vec<((u32, u32), (u32, u32))> = io
        .lines()
        .map(|line| {
            line.map_err(|e| anyhow!(e)).and_then(|s| {
                let split: Vec<Option<u32>> =
                    s.split(&['-', ',']).map(|s| s.parse().ok()).collect();
                match ((split[0], split[1]), (split[2], split[3])) {
                    ((Some(a), Some(b)), (Some(c), Some(d))) => Ok(((a, b), (c, d))),
                    _ => bail!("unable to parse line"),
                }
            })
        })
        .collect::<Result<Vec<((u32, u32), (u32, u32))>>>()?;

    let answer: (u32, u32) = pairs.iter().fold((0, 0), |mut acc, ((a, b), (c, d))| {
        if (*a..=*b).contains(c) && (*a..=*b).contains(d)
            || (*c..=*d).contains(a) && (*c..=*d).contains(b)
        {
            acc.0 += 1
        }
        if (*a..=*b).contains(c)
            || (*a..=*b).contains(d)
            || (*c..=*d).contains(a)
            || (*c..=*d).contains(b)
        {
            acc.1 += 1
        }
        acc
    });

    print!(
        "Day 4 - Part 1: {}\nDay 4 - Part 2: {}\n",
        answer.0, answer.1
    );

    Ok(())
}
