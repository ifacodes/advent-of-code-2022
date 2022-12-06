use anyhow::{anyhow, Result};
use std::{
    fmt::Display,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug)]
struct Cargo(pub Vec<Vec<char>>);

impl From<Vec<Vec<char>>> for Cargo {
    fn from(v: Vec<Vec<char>>) -> Self {
        Self(v)
    }
}

impl Display for Cargo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let length = self
            .0
            .iter()
            .map(|v| v.len())
            .max()
            .ok_or(std::fmt::Error)?;
        for n in (0..length).rev() {
            for stack in self.0.iter() {
                match stack.get(n) {
                    Some(c) => write!(f, "[{c}] ")?,
                    None => write!(f, "    ")?,
                }
            }
            writeln!(f)?
        }
        writeln!(f, " 1   2   3   4   5   6   7   8   9  ")
    }
}

fn move_cargo(cargo: &[Vec<char>], insts: &[(usize, usize, usize)], part_2: bool) -> Result<Cargo> {
    let mut cargo = cargo.to_owned();
    for (mov, from, to) in insts {
        let mut temp = vec![];
        let amount = cargo[*from].len() - mov;
        temp.extend(cargo[*from].drain((amount)..));
        if part_2 {
            cargo[*to].append(&mut temp)
        } else {
            cargo[*to].extend(temp.drain(..).rev())
        }
    }
    Ok(cargo.into())
}

fn top_cargo(cargo: &[Vec<char>]) {
    let top: String = cargo.iter().map(|s| s.last().unwrap()).collect();
    println!("{top}")
}

pub fn day_5(io: BufReader<impl Read>) -> Result<()> {
    let (cargo, inst) = {
        let mut iter = io.lines().map(|line| line.map_err(|e| anyhow!(e)));
        let cargo: Vec<String> = iter.by_ref().take(8).collect::<Result<Vec<_>>>()?;
        let cargo: Cargo = cargo
            .iter()
            .rev()
            .fold(vec![vec![]; 9], |mut acc, s| {
                s.as_bytes()
                    .chunks(4)
                    .map(|s| unsafe { std::str::from_utf8_unchecked(s) })
                    .map(|s| match s.chars().nth(1) {
                        Some(' ') => None,
                        Some(c) => Some(c),
                        _ => unreachable!(),
                    })
                    .enumerate()
                    .for_each(|(n, cargo)| {
                        if let Some(c) = cargo {
                            acc[n].push(c)
                        }
                    });
                acc
            })
            .into();

        let inst = iter
            .skip(2)
            .map(|inst| {
                inst.map(|s| {
                    let split: Vec<_> = s.split_whitespace().collect();

                    (
                        split[1].parse().expect("should be an integer!"),
                        split[3].parse::<usize>().expect("should be an integer!") - 1,
                        split[5].parse::<usize>().expect("should be an integer!") - 1,
                    )
                })
            })
            .collect::<Result<Vec<(usize, usize, usize)>>>()?;
        (cargo, inst)
    };
    let part_1 = move_cargo(&cargo.0, &inst, false)?;
    print!("Day 5 - {part_1}\nPart 1: ");
    top_cargo(&part_1.0);
    let part_2 = move_cargo(&cargo.0, &inst, true)?;
    print!("Day 5 - {part_2}\nPart 2: ");
    top_cargo(&part_2.0);

    Ok(())
}
