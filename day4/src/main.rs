#![allow(dead_code)]

struct Pair(Range, Range);

impl Pair {
    pub fn new(range1: Range, range2: Range) -> Self {
        Pair(range1, range2)
    }

    pub fn fully_contains(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    pub fn ranges_overlapping(&self) -> bool {
        self.0.overlaps_with(&self.1) || self.fully_contains()
    }
}

struct Range {
    lower: usize,
    upper: usize,
}

impl Range {
    pub fn new(lower: usize, upper: usize) -> Self {
        Range { lower, upper }
    }

    pub fn contains(&self, other: &Range) -> bool {
        self.lower <= other.lower && self.upper >= other.upper
    }

    pub fn overlaps_with(&self, other: &Range) -> bool {
        self.lower <= other.lower && self.upper >= other.lower
            || self.lower <= other.upper && self.upper >= other.upper
    }
}

fn parse_range(slice: &str) -> Range {
    let i = slice.find("-").expect("Invalid sytnax.");
    let lower = slice[0..i].parse::<usize>().expect("Invalid sytnax.");
    let upper = slice[i+1..].parse::<usize>().expect("Invalid sytnax.");
    Range::new(lower, upper)
}

fn parse_pair(line: &str) -> Pair {
    if let Some(i) = line.find(",") {
        let first_part = &line[..i];
        let second_part = &line[i+1..];
        let range1 = parse_range(first_part);
        let range2 = parse_range(second_part);
        Pair::new(range1, range2)
    } else {
        unreachable!("Invalid syntax")
    }
}

// Part 1
pub fn how_many_pairs_1(input: &str) -> usize {
    let mut result = 0;
    for line in input.lines() {
        let pair = parse_pair(line);
        if pair.fully_contains() { 
            result += 1;
        }
    }
    result
}

// Part 2
pub fn how_many_pairs_2(input: &str) -> usize {
    let mut result = 0;
    for line in input.lines() {
        let pair = parse_pair(line);
        if pair.ranges_overlapping() { 
            result += 1;
        }
    }
    result
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Invalid filename.");
    println!("{}", how_many_pairs_2(&input));
}