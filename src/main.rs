mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use anyhow::*;
use day1::day_1;
use day10::day_10;
use day11::day_11;
use day2::day_2;
use day3::day_3;
use day4::day_4;
use day5::day_5;
use day6::day_6;
use day7::day_7;
use day8::day_8;
use day9::day_9;
use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() -> Result<()> {
    day_1(open_file("day1")?)?;
    day_2(open_file("day2")?)?;
    day_3(open_file("day3")?)?;
    day_4(open_file("day4")?)?;
    day_5(open_file("day5")?)?;
    day_6(open_file("day6")?)?;
    day_7(open_file("day7")?)?;
    day_8()?;
    day_9()?;
    day_10()?;
    day_11()?;
    Ok(())
}

fn open_file(name: &str) -> Result<BufReader<impl Read>> {
    Ok(BufReader::new(File::open(format!("./input/{}", name))?))
}
