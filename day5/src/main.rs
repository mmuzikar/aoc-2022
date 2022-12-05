use std::ops::Range;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Stack {
    crates: Vec<char>,
}

impl Stack {
    fn construct_stacks(input: &[&str]) -> Vec<Stack> {
        let stack_ids = input
            .last()
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>().expect("Failed to convert into stack id"))
            .collect::<Vec<_>>();

        let mut stacks = (0..stack_ids.len())
            .map(|_| Stack { crates: vec![] })
            .collect::<Vec<_>>();

        let str_indices = {
            let mut indices = Vec::<usize>::with_capacity(stack_ids.len());
            indices.push(1);
            for _ in 1..stack_ids.len() {
                indices.push(indices.last().unwrap() + 4)
            }

            indices
        };

        let columns = &input[..input.len() - 1]
            .iter()
            .map(|s| {
                let mut ids = Vec::<char>::with_capacity(stack_ids.len());
                for i in &str_indices {
                    let id = s.chars().nth(*i).unwrap();
                    ids.push(id);
                }
                ids
            })
            .collect::<Vec<_>>();

        for column in columns.iter().rev() {
            for id in 0..stack_ids.len() {
                if column[id].is_alphabetic() {
                    stacks[id].crates.push(column[id]);
                }
            }
        }

        stacks
    }
}

#[derive(Debug)]
struct Move {
    amount: u8,
    from: usize,
    to: usize,
}

impl Move {
    fn try_parsing(s: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }

        if let Some(groups) = RE.captures(s) {
            return Some(Move {
                amount: groups[1].parse().ok()?,
                from: groups[2].parse().ok()?,
                to: groups[3].parse().ok()?,
            });
        }

        None
    }
}

fn simulate(stacks: &mut Vec<Stack>, moves: &Vec<Move>, retain_order: bool) {
    for m in moves {
        let from = m.from - 1;
        let to =m.to - 1;
        if !retain_order {
            for _ in 0..m.amount {
                let c = stacks[from].crates.pop().expect("Not enough crates");
                stacks[to].crates.push(c);
            }
        } else {
            let range = Range {
                end: stacks[from].crates.len(),
                start: stacks[from].crates.len() - m.amount as usize,
            }; 

            let mut stack = stacks[from]
                .crates
                .drain(range)
                .collect::<Vec<_>>();
            stacks[to].crates.append(&mut stack);
        }
    }

    println!("Top crates: ");
    for stack in stacks {
        print!("{}", stack.crates.last().unwrap())
    }
    println!();
}

fn main() {
    let input = include_str!("input");
    let split_pos = input
        .lines()
        .position(|s| s.is_empty())
        .expect("Failed to find empty line splitting crane from moves");

    let lines = input.lines().collect::<Vec<_>>();
    let (stack_input, moves_input) = lines.split_at(split_pos);

    let moves = moves_input
        .iter()
        .map(|s| Move::try_parsing(s))
        .filter_map(|it| it)
        .collect::<Vec<_>>();

    let mut stacks = Stack::construct_stacks(stack_input);

    simulate(&mut stacks, &moves, true);
}
