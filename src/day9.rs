use std::fmt::Display;

use anyhow::Result;

const INPUT: &str = include_str!("../input/day9");

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn move_with_vector(&mut self, v: (i32, i32)) {
        self.x += v.0;
        self.y += v.1;
    }
    fn catch_up(&mut self, head: &Knot) {
        if (self.x - head.x).abs() <= 1 && (self.y - head.y).abs() <= 1 {
            return;
        }

        if self.x != head.x {
            if self.x < head.x {
                self.x += 1;
            } else {
                self.x -= 1;
            }
        }
        if self.y != head.y {
            if self.y < head.y {
                self.y += 1;
            } else {
                self.y -= 1;
            }
        }
    }
    fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl Display for Knot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

pub fn parse_input(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .flat_map(|line| {
            let (direction, n) = line.split_once(' ').unwrap();
            (0..n.parse::<i32>().unwrap()).map(move |_| match direction {
                "U" => (0, 1),
                "D" => (0, -1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _ => unreachable!(),
            })
        })
        .collect()
}

pub fn day_9() -> Result<()> {
    let movements: Vec<(i32, i32)> = parse_input(INPUT);
    let mut head = Knot::default();
    let mut tail = Knot::default();
    let mut tail_visited: Vec<(i32, i32)> = vec![(0, 0)];
    for pos in &movements {
        head.move_with_vector(*pos);
        tail.catch_up(&head);
        if !tail_visited.iter().any(|p| *p == tail.pos()) {
            tail_visited.push(tail.pos());
        }
    }

    println!("Day 9 - Part 1: {}", tail_visited.len());

    let mut part_2 = vec![Knot::default(); 10];
    tail_visited = vec![(0, 0)];
    for pos in &movements {
        part_2[0].move_with_vector(*pos);
        for n in 1..10 {
            let head = part_2[n - 1];
            part_2[n].catch_up(&head);
        }
        if !tail_visited
            .iter()
            .any(|p| *p == part_2.last().unwrap().pos())
        {
            tail_visited.push(part_2.last().unwrap().pos());
        }
    }

    println!("Day 9 - Part 2: {}", tail_visited.len());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
    const EXAMPLE2: &str = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";

    #[test]
    fn example() {
        let mut head = Knot::default();
        let mut tail = Knot::default();
        let mut tail_visited: Vec<(i32, i32)> = vec![tail.pos()];
        let movements = parse_input(EXAMPLE);
        for pos in movements {
            head.move_with_vector(pos);
            tail.catch_up(&head);
            if !tail_visited.iter().any(|p| *p == tail.pos()) {
                tail_visited.push(tail.pos());
            }
        }
        println!("{tail_visited:?}");
        assert_eq!(tail_visited.len(), 13)
    }

    #[test]
    fn example2() {
        let mut knots = [Knot::default(); 10];
        let mut tail_visited: Vec<(i32, i32)> = vec![(0, 0)];
        let movements = parse_input(EXAMPLE2);
        for pos in movements {
            knots[0].move_with_vector(pos);
            for n in 1..10 {
                let head = knots[n - 1];
                knots[n].catch_up(&head);
            }
            if !tail_visited.iter().any(|p| *p == knots[9].pos()) {
                tail_visited.push(knots[9].pos());
            }
        }
        println!("{tail_visited:?}");
        assert_eq!(tail_visited.len(), 36)
    }

    #[test]
    fn diagonal_movement() {
        let mut head = Knot::default();
        let mut tail = Knot::default();
        let movements = parse_input(EXAMPLE);
        assert_eq!(movements.len(), 24);
        for pos in movements[..4].iter() {
            head.move_with_vector(*pos);
            tail.catch_up(&head);
        }
        assert_eq!(tail, Knot { x: 3, y: 0 });
        for pos in movements[4..6].iter() {
            head.move_with_vector(*pos);
            tail.catch_up(&head);
        }
        assert_eq!(tail, Knot { x: 4, y: 1 });
        for pos in movements[6..8].iter() {
            head.move_with_vector(*pos);
            tail.catch_up(&head);
        }
        assert_eq!(tail, Knot { x: 4, y: 3 });
        head.move_with_vector(movements[8]);
        tail.catch_up(&head);
        assert_eq!(head, Knot { x: 3, y: 4 });
        assert_eq!(tail, Knot { x: 4, y: 3 });
        head.move_with_vector(movements[9]);
        tail.catch_up(&head);
        assert_eq!(head, Knot { x: 2, y: 4 });
        assert_eq!(tail, Knot { x: 3, y: 4 });
    }

    #[test]
    fn movement() {
        let mut knot = Knot::default();
        knot.move_with_vector((0, -2));
        assert_eq!(Knot { x: 0, y: -2 }, knot)
    }
}
