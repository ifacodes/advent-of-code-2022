use anyhow::{anyhow, Result};

const INPUT: &'static str = include_str!("../input/day8");

//const INPUT: &str = "30373\n25512\n65332\n33549\n35390";

fn part_1(width: usize, height: usize, input: &[i32]) -> usize {
    let mut visible = vec![false; input.len()];
    for y in 0..height {
        let mut tree_height = -1;
        for x in 0..width {
            if input[y * width + x] > tree_height {
                visible[y * width + x] = true;
                tree_height = input[y * width + x];
            }
        }
        tree_height = -1;

        for x in (1..width).rev() {
            if input[y * width + x] > tree_height {
                visible[y * width + x] = true;
                tree_height = input[y * width + x];
            }
        }
    }

    for x in 0..width {
        let mut tree_height = -1;
        for y in 0..height {
            if input[y * width + x] > tree_height {
                visible[y * width + x] = true;
                tree_height = input[y * width + x];
            }
        }
        tree_height = -1;
        for y in (1..height).rev() {
            if input[y * width + x] > tree_height {
                visible[y * width + x] = true;
                tree_height = input[y * width + x];
            }
        }
    }
    visible.iter().map(|b| usize::from(*b)).sum::<usize>()
}

fn part_2(width: usize, height: usize, input: &[i32]) -> usize {
    let mut scenic_scores = vec![1; input.len()];

    for x in 0..width {
        for y in 0..width {
            if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                scenic_scores[y * width + x] = 0;
                continue;
            }
            let mut score = 1;
            // check left
            (1..x)
                .rev()
                .take_while(|t| input[y * width + *t] < input[y * width + x])
                .for_each(|_| {
                    score += 1;
                });
            scenic_scores[y * width + x] *= score;
            score = 1;
            // check right
            (x + 1..width - 1)
                .take_while(|t| input[y * width + *t] < input[y * width + x])
                .for_each(|_| {
                    score += 1;
                });
            scenic_scores[y * width + x] *= score;
            score = 1;
            (1..y)
                .rev()
                .take_while(|t| input[*t * width + x] < input[y * width + x])
                .for_each(|_| {
                    score += 1;
                });
            scenic_scores[y * width + x] *= score;
            score = 1;
            (y + 1..height - 1)
                .take_while(|t| input[*t * width + x] < input[y * width + x])
                .for_each(|_| {
                    score += 1;
                });
            scenic_scores[y * width + x] *= score;
        }
    }
    *scenic_scores.iter().max().unwrap()
}

pub fn day_8() -> Result<()> {
    let width = INPUT
        .lines()
        .next()
        .ok_or_else(|| anyhow!("can't get grid width"))?
        .len();
    let height = INPUT.lines().count();

    let mut input_grid = vec![0; width * height];

    INPUT.lines().enumerate().for_each(|(y, lines)| {
        lines.chars().enumerate().for_each(|(x, col)| {
            input_grid[y * width + x] = col as i32 - '0' as i32;
        })
    });

    println!("Day 8 - Part 1: {}", part_1(width, height, &input_grid));
    println!("Day 8 - Part 2: {}", part_2(width, height, &input_grid));

    Ok(())
}
