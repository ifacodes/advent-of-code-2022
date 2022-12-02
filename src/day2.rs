use anyhow::Result;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug, PartialEq, Clone, Copy)]
enum RockPaperScissors {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl RockPaperScissors {
    fn wins_against(&self) -> Self {
        match self {
            RockPaperScissors::Rock => RockPaperScissors::Scissors,
            RockPaperScissors::Paper => RockPaperScissors::Rock,
            RockPaperScissors::Scissors => RockPaperScissors::Paper,
        }
    }

    fn loses_against(&self) -> Self {
        match self {
            RockPaperScissors::Rock => RockPaperScissors::Paper,
            RockPaperScissors::Paper => RockPaperScissors::Scissors,
            RockPaperScissors::Scissors => RockPaperScissors::Rock,
        }
    }

    fn score(&self, other: &Self) -> u32 {
        if self.wins_against() == *other {
            *self as u32 + 6
        } else if self.loses_against() == *other {
            *self as u32
        } else {
            *self as u32 + 3
        }
    }
}

pub fn day_2(io: BufReader<impl Read>) -> Result<()> {
    let rounds: Vec<(RockPaperScissors, &str)> = io
        .lines()
        .map(|line| match line.unwrap().as_str() {
            "A X" => (RockPaperScissors::Rock, "X"),
            "A Y" => (RockPaperScissors::Rock, "Y"),
            "A Z" => (RockPaperScissors::Rock, "Z"),
            "B X" => (RockPaperScissors::Paper, "X"),
            "B Y" => (RockPaperScissors::Paper, "Y"),
            "B Z" => (RockPaperScissors::Paper, "Z"),
            "C X" => (RockPaperScissors::Scissors, "X"),
            "C Y" => (RockPaperScissors::Scissors, "Y"),
            "C Z" => (RockPaperScissors::Scissors, "Z"),
            _ => unreachable!(),
        })
        .collect();

    let part_1: u32 = rounds
        .iter()
        .map(|(opp, you)| match *you {
            "X" => RockPaperScissors::Rock.score(opp),
            "Y" => RockPaperScissors::Paper.score(opp),
            "Z" => RockPaperScissors::Scissors.score(opp),
            _ => unreachable!(),
        })
        .reduce(|acc, x| acc + x)
        .unwrap();

    println!("Day 2 - Part 1: {part_1}");

    let part_2 = rounds
        .iter()
        .map(|(opp, you)| match *you {
            "X" => opp.wins_against().score(opp),
            "Y" => opp.score(opp),
            "Z" => opp.loses_against().score(opp),
            _ => unreachable!(),
        })
        .reduce(|acc, x| acc + x)
        .unwrap();

    println!("Day 2 - Part 2: {part_2}");

    Ok(())
}
