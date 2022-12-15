use anyhow::Result;

const INPUT: &str = include_str!("../input/day10");
const EXAMPLE: &str = include_str!("../input/example10");

pub fn day_10() -> Result<()> {
    let mut cycle = 1usize;
    let mut reg_x = 1i32;
    let mut signal_strength = 0i32;
    let mut crt = vec!['.'; 40 * 6];

    for (add_x, mut cycles_needed) in
        INPUT
            .lines()
            .map(|line| line.split_once(' '))
            .map(|t| match t {
                Some((_, x)) => (x.parse::<i32>().unwrap(), 2),
                None => (0, 1),
            })
    {
        while cycles_needed != 0 {
            if cycle == 20
                || cycle == 60
                || cycle == 100
                || cycle == 140
                || cycle == 180
                || cycle == 220
            {
                signal_strength += reg_x * cycle as i32;
            }
            if ((cycle - 1) % 40) as i32 == reg_x - 1 {
                crt[cycle - 1] = '#'
            }
            if ((cycle - 1) % 40) as i32 == reg_x {
                crt[cycle - 1] = '#'
            }
            if ((cycle - 1) % 40) as i32 == reg_x + 1 {
                crt[cycle - 1] = '#'
            }
            cycles_needed -= 1;
            cycle += 1;
        }
        reg_x += add_x;
    }
    println!("Day 10 - Part 1: {signal_strength}");
    println!("Day 10 - Part 2:");
    for y in 0..6 {
        for x in 0..40 {
            print!("{}", &crt[y * 40 + x])
        }
        println!()
    }
    Ok(())
}
