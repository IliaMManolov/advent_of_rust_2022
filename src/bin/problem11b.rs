use std::{collections::VecDeque, fs, str::Lines};
use itertools::Itertools;

#[derive(Default, Clone, Debug)]

enum Operation {
    Plus(u64),
    Mult(u64),
    #[default]
    PlusSelf,
    MultSelf,
}

#[derive(Default, Clone, Debug)]
struct Monkey {
    items: VecDeque<u64>,
    op: Operation,
    test_divider: u64,
    true_monkey: usize,
    false_monkey: usize,
    inspect_cnt: u64,
}

fn parse_input(lines: &mut Lines) -> (Vec<Monkey>, u64) {
    let mut monkeys = Vec::new();
    let mut curr_monkey = Monkey::default();
    let mut divider_cap: u64 = 1;

    for line in lines {
        if line.starts_with("Monkey") {
            if !line.split_whitespace().contains(&"0:") {
                monkeys.push(curr_monkey.clone());
            }
        } else if line.starts_with("  Starting") {
            curr_monkey.items = line
                .split_whitespace()
                .skip(2)
                .map(|x: &str| {
                    x.to_string()
                        .chars()
                        .filter(|c| c.is_digit(10))
                        .collect::<String>()
                        .parse::<u64>()
                        .expect("Saw an item that wasn't a number")
                })
                .collect();
        } else if line.starts_with("  Operation") {
            let old_cnt = line.split_whitespace().filter(|x| x == &"old").count();
            if old_cnt == 2 {
                curr_monkey.op = if line.contains('+') {
                    Operation::PlusSelf
                } else {
                    Operation::MultSelf
                };
            } else {
                let amount_by = line
                    .split_whitespace()
                    .last()
                    .expect("The line has at least 1 word")
                    .parse::<u64>()
                    .expect("The last word on the line is a number");

                curr_monkey.op = if line.contains('+') {
                    Operation::Plus(amount_by)
                } else {
                    Operation::Mult(amount_by)
                };
            }
        } else if line.starts_with("  Test") {
            curr_monkey.test_divider = line
                .split_whitespace()
                .last()
                .expect("The line has at least 1 word")
                .parse::<u64>()
                .expect("The last word on the line is a number");

            divider_cap *= curr_monkey.test_divider;
        } else if line.starts_with("    If true") {
            curr_monkey.true_monkey = line
                .split_whitespace()
                .last()
                .expect("The line has at least 1 word")
                .parse::<usize>()
                .expect("The last word on the line is a number");
        } else if line.starts_with("    If false") {
            curr_monkey.false_monkey = line
                .split_whitespace()
                .last()
                .expect("The line has at least 1 word")
                .parse::<usize>()
                .expect("The last word on the line is a number");
        }
    }

    monkeys.push(curr_monkey.clone());

    (monkeys, divider_cap)
}

fn apply_round(monkeys: &mut Vec<Monkey>, divider_cap: u64) {
    for throwing_index in 0..monkeys.len() {
        let throwing_monkey = &mut monkeys[throwing_index].clone();

        while let Some(item) = throwing_monkey.items.pop_front() {
            let new_item = match throwing_monkey.op {
                Operation::Plus(x) => item + x,
                Operation::Mult(x) => item * x,
                Operation::PlusSelf => item + item,
                Operation::MultSelf => item * item,
            } % divider_cap;

            // The key is to keep just the modulos

            let target_index = if new_item % throwing_monkey.test_divider == 0 {
                throwing_monkey.true_monkey
            } else {
                throwing_monkey.false_monkey
            };
            throwing_monkey.inspect_cnt += 1;

            let target_monkey = &mut monkeys[target_index];

            target_monkey.items.push_back(new_item);
        }

        throwing_monkey.clone_into(&mut monkeys[throwing_index]);
    }
}

fn main() {
    let file_path = "inputs/problem11.in";
    let contents = fs::read_to_string(file_path).expect("Should be able to read file");
    let mut lines = contents.lines();

    let (mut monkeys, divider_cap) = parse_input(&mut lines);

    for _ in 0..10000 {
        apply_round(&mut monkeys, divider_cap);
    }

    let top_two = monkeys
        .into_iter()
        .map(|monkey| monkey.inspect_cnt)
        .sorted()
        .rev()
        .take(2)
        .collect_vec();
    let monkey_business = top_two[0] * top_two[1];

    println!("{monkey_business}");
}
