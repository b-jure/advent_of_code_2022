#![allow(dead_code)]

use std::{fmt::Write, fs};
use regex::Regex;
use std::collections::VecDeque;

type Crate = char;

#[derive(Clone)]
struct Stack {
    crates: VecDeque<Crate>,
}

impl Stack {
    fn new() -> Self {
        Stack { crates: VecDeque::new() }
    }

    fn len(&self) -> usize {
        self.crates.len()
    }

    fn top(&self) -> Option<&Crate> {
        self.crates.front()
    }

    fn push_top(&mut self, val: Crate) {
        self.crates.push_front(val)
    }

    fn push_back(&mut self, val: Crate) {
        self.crates.push_back(val)
    }

    fn pop_top(&mut self) -> Option<Crate> {
        self.crates.pop_front()
    }

    fn append(&mut self, other: &mut VecDeque<Crate>) {
        self.crates.append(other)
    }
}

#[derive(PartialEq, Eq)]
enum Move {
    Default,
    OrdMove,
}

struct Supplies {
    stacks: Vec<Stack>,
}

impl Supplies {
    pub fn new(input: &str) -> Self {
        let last_line = input.lines().find(|line| line.starts_with(" 1")).expect("Invalid input");

        let x = last_line
            .split_whitespace()
            .last()
            .expect("Invalid input")
            .parse::<usize>()
            .expect("Invalid input");

        let mut supplies = Supplies::from(x);

        for line in input.lines() {
            if !line.starts_with(" 1") {
                supplies.find_and_push(line.trim());
            } else {
                break;
            }
        }

        supplies
    }

    fn from(x: usize) -> Self {
        Supplies { stacks: vec![Stack::new(); x] }
    }

    pub fn push_crate_back(&mut self, idx: usize, val: Crate) {
        self.stacks[idx].push_back(val)
    }

    pub fn push_crate_top(&mut self, idx: usize, val: Crate) {
        self.stacks[idx].push_top(val)
    }

    pub fn pop_crate_top(&mut self, idx: usize) -> Option<Crate> {
        self.stacks[idx].pop_top()
    }

    pub fn size(&self) -> usize {
        self.stacks.len()
    }

    pub fn on_top(&self) -> String {
        let capacity = self.stacks.len();
        let mut buffer: String = String::with_capacity(capacity);

        for stack in self.stacks.iter() {
            stack.top().map(|char| buffer.write_char(*char).expect("Invalid char"));
        }

        buffer
    }

    pub fn append(&mut self, idx: usize, other: &mut VecDeque<Crate>) {
        self.stacks[idx].append(other)
    }

    pub fn find_and_push(&mut self, line: &str) {
        const OFFSET: usize = 1;
        const CRATE_WIDTH: usize = 3;

        let mut i = 0;
        let mut offset = 0;
        let len = line.len() - 1;
        let regex = Regex::new(r"(\[[A-Z]\])").unwrap();
    
        while len >= offset {
            let slice: &str;
    
            if i == 0 {
                slice = &line[0..offset + CRATE_WIDTH];
            } else {
                let end = (offset + CRATE_WIDTH).min(len + OFFSET);
                slice = &line[offset..end];
            }
    
            if regex.is_match(slice) {
                let val = parse_crate(slice);
                self.push_crate_back(i, val);
            }
    
            offset += CRATE_WIDTH + OFFSET;
            i += 1;
        }
    }

    pub fn read_instruction(&mut self, instruction: &str) {
        match instruction {
            ins if ins.starts_with("move") => Self::move_ins(self, ins, Move::Default),
            ins if ins.starts_with("ordmove") => Self::move_ins(self, ins, Move::OrdMove),
            _ => (),
        }
    }

    fn move_ins(supplies: &mut Self, instruction: &str, kind: Move) {
        let parts: Vec<&str> = instruction.split_whitespace().collect();

        let amount = parts[1].parse::<usize>().expect("Invalid amount");
        let pop_idx = parts[3].parse::<usize>().expect("Invalid stack index") - 1;
        let push_idx = parts[5].parse::<usize>().expect("Invalid stack index") - 1;

        if kind == Move::Default {
            Self::move_default(supplies, amount, pop_idx, push_idx);
        } else if kind == Move::OrdMove {
            Self::move_ord(supplies, amount, pop_idx, push_idx);
        } else {
            unreachable!("Invalid move type");
        }
    }

    fn move_default(supplies: &mut Self, amount: usize, pop_idx: usize, push_idx: usize) {
        for _ in 0..amount {
            let popped = supplies.pop_crate_top(pop_idx);
            if let Some(val) = popped {
                supplies.push_crate_top(push_idx, val);
            } else {
                break;
            }
        }
    }

    fn move_ord(supplies: &mut Self, amount: usize, pop_idx: usize, push_idx: usize) {
        let mut package = VecDeque::new();

        for _ in 0..amount {
            let popped = supplies.pop_crate_top(pop_idx);
            if let Some(val) = popped {
                package.push_back(val);
            } else {
                break;
            }
        }

        package.append(&mut supplies.stacks[push_idx].crates);
        supplies.stacks[push_idx].crates = package;
    }
}

fn parse_crate(slice: &str) -> Crate {
    slice[1..].chars().next().unwrap()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Invalid file");
    let mut supplies = Supplies::new(&input);
    for line in input.lines() {
        supplies.read_instruction(line);
    }

    println!("{}", supplies.on_top());
}

#[cfg(test)]
mod test {
    use crate::Supplies;

    #[test]
    fn move_default() {
        let input = std::fs::read_to_string("input2.txt").unwrap();
        let mut supplies = Supplies::new(&input);
        for line in input.lines() {
            supplies.read_instruction(line);
        }
        assert_eq!(supplies.on_top(), "MHGSNWSVF".to_string());
    }

    #[test]
    fn move_ord() {
        let input = std::fs::read_to_string("input3.txt").unwrap();
        let mut supplies = Supplies::new(&input);
        for line in input.lines() {
            supplies.read_instruction(line);
        }
        assert_eq!(supplies.on_top(), "JHGMNWWVF".to_string());
    }
}