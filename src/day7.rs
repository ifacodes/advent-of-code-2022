use anyhow::{anyhow, Result};
use std::{
    borrow::BorrowMut,
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    ops::Add,
    str::from_utf8,
};

#[derive(Debug)]
struct DirectoryTree {
    //current_dir: Vec<&'input [u8]>,
    tree: HashMap<String, usize>,
}

impl DirectoryTree {
    pub fn new() -> Self {
        Self {
            tree: HashMap::new(),
        }
    }

    pub fn populate(&mut self, lines: &[String]) -> Result<()> {
        let mut current_dir = vec![];
        // if cd not .. add current dir
        // skip if ls
        // add all the dir names to the hashmap with a size 0.
        // add all the file sizes to current dir size in the hashmap.
        lines
            .iter()
            .map(|line| line.as_bytes())
            .for_each(|bytes| match bytes[0] {
                b'$' => {
                    if bytes[2] == b'c' {
                        match bytes[5] {
                            b'.' => {
                                current_dir.pop().unwrap();
                            }
                            _ => {
                                let parent = current_dir.join("/");
                                current_dir.push(from_utf8(&bytes[5..]).unwrap());
                                self.tree.entry(current_dir.join("/")).or_insert(0);
                            }
                        }
                    }
                }
                b'd' => {}
                _ => {
                    let (size, _) = from_utf8(bytes).unwrap().split_once(' ').unwrap();
                    (1..current_dir.len() + 1).rev().for_each(|n| {
                        self.tree
                            .entry(current_dir[0..n].join("/"))
                            .and_modify(|s| *s += size.parse::<usize>().unwrap());
                    })
                }
            });

        Ok(())
    }

    fn part_1(&self) -> usize {
        self.tree.values().filter(|v| **v <= 100000).sum()
    }
    fn part_2(&self) -> usize {
        *self
            .tree
            .values()
            .filter(|v| **v >= self.tree["/"] - 40000000)
            .min()
            .unwrap()
    }
}

pub fn day_7(io: BufReader<impl Read>) -> Result<()> {
    let mut tree = DirectoryTree::new();
    tree.populate(
        &io.lines()
            .map(|r| r.map_err(|e| anyhow!(e)))
            .collect::<Result<Vec<String>>>()?,
    )?;
    println!(
        "Day 7 - Part 1: {:#?}\nDay 7 - Part 2: {}",
        tree.part_1(),
        tree.part_2()
    );
    Ok(())
}
