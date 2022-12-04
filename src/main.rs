mod day1;
mod day2;
mod day3;
mod day4;

use anyhow::*;
use day1::day_1;
use day2::day_2;
use day3::day_3;
use day4::day_4;
use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() -> Result<()> {
    day_1(open_file("day1")?)?;
    day_2(open_file("day2")?)?;
    day_3(open_file("day3")?)?;
    day_4(open_file("day4")?)?;
    Ok(())
}

fn open_file(name: &str) -> Result<BufReader<impl Read>> {
    Ok(BufReader::new(File::open(format!("./input/{}", name))?))
}
