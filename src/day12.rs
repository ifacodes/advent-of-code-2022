use std::{
    collections::{HashMap, HashSet},
    fmt::{write, Display},
};

use anyhow::Result;

const EXAMPLE: &str = include_str!("../input/example12");
const INPUT: &str = include_str!("../input/day12");

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Coord {
    fn from((x, y): (usize, usize)) -> Self {
        Coord { x, y }
    }
}

#[derive(Debug)]
struct HeightMap {
    data: Vec<u8>,
    height: usize,
    width: usize,
}

impl HeightMap {
    fn new(input: &str) -> Self {
        Self {
            data: input.split_whitespace().collect::<String>().into_bytes(),
            height: input.lines().count(),
            width: input.lines().next().unwrap().len(),
        }
    }
    fn height_at(&self, at: Coord) -> u8 {
        if at == self.start() {
            return b'a';
        }
        if at == self.end() {
            return b'z';
        }
        self.data[at.y * self.width + at.x]
    }
    fn start(&self) -> Coord {
        let pos = self.data.iter().position(|b| *b == b'S').unwrap();
        (pos % self.width, pos / self.width).into()
    }
    fn end(&self) -> Coord {
        let pos = self.data.iter().position(|b| *b == b'E').unwrap();
        (pos % self.width, pos / self.width).into()
    }
    fn walk(&self) -> i32 {
        let mut steps = 0;
        let mut current_step: HashSet<Coord> = HashSet::new();
        let mut visited: HashMap<Coord, Option<Coord>> = HashMap::new();
        current_step.insert(self.start());
        visited.insert(self.start(), None);
        let mut the_end = false;

        while !the_end {
            let mut next: HashSet<Coord> = HashSet::new();
            println!("Current Step: {current_step:?}");
            for coord in current_step {
                for neighbour in self.viable_neighbours(coord) {
                    println!("{neighbour:?}");
                    if visited.contains_key(&neighbour) {
                        continue;
                    }
                    visited.insert(neighbour, Some(coord));
                    next.insert(neighbour);
                }
            }
            current_step = next;
            steps += 1;
            if current_step.contains(&self.end()) {
                the_end = true;
            }
        }
        steps
    }
    fn viable_neighbours(&self, of: Coord) -> impl Iterator<Item = Coord> + '_ {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(move |(x, y)| {
                Some(Coord {
                    x: of.x.checked_add_signed(x)?,
                    y: of.y.checked_add_signed(y)?,
                })
            })
            .filter(|&x| x.x < self.width && x.y < self.height)
            .filter(move |&x| self.height_at(x) <= self.height_at(of) + 1)
    }
}

impl Display for HeightMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if Coord::from((x, y)) == self.start() {
                    write!(f, "\x1B[38;5;46m")?
                }
                if Coord::from((x, y)) == self.end() {
                    write!(f, "\x1B[38;5;196m")?
                }
                write!(f, "{}", self.data[y * self.width + x] as char)?;
                write!(f, "\x1B[0m")?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

pub fn day_12() -> Result<()> {
    let map = HeightMap::new(INPUT);

    print!("{map}");
    print!("steps: {}", map.walk());
    Ok(())
}
