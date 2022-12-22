use anyhow::Result;

const EXAMPLE: &str = include_str!("../input/example11");
const INPUT: &str = include_str!("../input/day11");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Monkey {
    starting_items: Vec<usize>,
    operation: Operation,
    divisible: usize,
    if_true: usize,
    if_false: usize,
    inspected: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Operation {
    Add(Value, Value),
    Mult(Value, Value),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Value {
    Old,
    Value(usize),
}

pub fn day_11() -> Result<()> {
    let input: Vec<String> = INPUT.lines().map(|l| l.to_string()).collect();
    let mut monkeys: Vec<Monkey> = input.chunks(7).map(parse_input).collect();

    let worry_save: usize = monkeys.iter().map(|m| m.divisible).product();

    let mut monkeys_1 = monkeys.clone();
    for _ in 0..20 {
        rounds(&mut monkeys_1, 3)
    }

    let mut part_1 = monkeys_1.iter().map(|m| m.inspected).collect::<Vec<_>>();
    part_1.sort();
    println!(
        "Day 11 - Part 1: {}",
        part_1[part_1.len() - 1] * part_1[part_1.len() - 2]
    );

    for _ in 0..10000 {
        rounds(&mut monkeys, worry_save)
    }

    let mut part_2 = monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>();
    part_2.sort();
    println!(
        "Day 11 - Part 1: {}",
        part_2[part_2.len() - 1] * part_2[part_2.len() - 2]
    );

    Ok(())
}

fn rounds(monkeys: &mut [Monkey], worry_save: usize) {
    for i in 0..monkeys.len() {
        monkeys[i].inspected += monkeys[i].starting_items.len();
        let clone = monkeys[i].clone();

        for mut item in clone.starting_items {
            item = match clone.operation {
                Operation::Add(x, y) => {
                    (match x {
                        Value::Old => item,
                        Value::Value(x) => x,
                    }) + (match y {
                        Value::Old => item,
                        Value::Value(x) => x,
                    })
                }
                Operation::Mult(x, y) => {
                    (match x {
                        Value::Old => item,
                        Value::Value(x) => x,
                    }) * (match y {
                        Value::Old => item,
                        Value::Value(x) => x,
                    })
                }
            } % worry_save;
            if item % clone.divisible == 0 {
                monkeys[clone.if_true].starting_items.push(item);
            } else {
                monkeys[clone.if_false].starting_items.push(item);
            }
        }
        monkeys[i].starting_items.clear()
    }
}

fn parse_input(m: &[String]) -> Monkey {
    Monkey {
        starting_items: m[1]
            .trim_start_matches("  Starting items:")
            .trim()
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect(),
        operation: parse_operation(&m[2]),
        divisible: m[3]
            .trim_start_matches("  Test: divisible by")
            .trim()
            .parse()
            .unwrap(),
        if_true: m[4]
            .trim_start_matches("    If true: throw to monkey")
            .trim()
            .parse()
            .unwrap(),
        if_false: m[5]
            .trim_start_matches("    If false: throw to monkey")
            .trim()
            .parse()
            .unwrap(),
        inspected: 0,
    }
}

fn parse_operation(input: &str) -> Operation {
    let tokens: Vec<_> = input
        .trim_start_matches("  Operation: new =")
        .split_whitespace()
        .collect();
    assert_eq!(tokens.len(), 3);
    let (l, r) = match (tokens[0], tokens[2]) {
        ("old", "old") => (Value::Old, Value::Old),
        ("old", x) => (Value::Old, Value::Value(x.parse().unwrap())),
        (x, "old") => (Value::Value(x.parse().unwrap()), Value::Old),
        (x, y) => (
            Value::Value(x.parse().unwrap()),
            Value::Value(y.parse().unwrap()),
        ),
    };
    match tokens[1] {
        "*" => Operation::Mult(l, r),
        _ => Operation::Add(l, r),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_input() {
        let input: Vec<String> = EXAMPLE.lines().map(|l| l.to_string()).collect();
        let monkeys: Vec<Monkey> = input.chunks(7).map(super::parse_input).collect();

        assert_eq!(
            monkeys,
            &[
                Monkey {
                    starting_items: vec![79, 98],
                    operation: Operation::Mult(Value::Old, Value::Value(19)),
                    divisible: 23,
                    if_true: 2,
                    if_false: 3,
                    inspected: 0
                },
                Monkey {
                    starting_items: vec![54, 65, 75, 74],
                    operation: Operation::Add(Value::Old, Value::Value(6)),
                    divisible: 19,
                    if_true: 2,
                    if_false: 0,
                    inspected: 0
                },
                Monkey {
                    starting_items: vec![79, 60, 97],
                    operation: Operation::Mult(Value::Old, Value::Old),
                    divisible: 13,
                    if_true: 1,
                    if_false: 3,
                    inspected: 0
                },
                Monkey {
                    starting_items: vec![74],
                    operation: Operation::Add(Value::Old, Value::Value(3)),
                    divisible: 17,
                    if_true: 0,
                    if_false: 1,
                    inspected: 0
                },
            ]
        )
    }
}
