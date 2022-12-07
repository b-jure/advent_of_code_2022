#![allow(dead_code)]

#[derive(Clone, Copy)]
enum GameResult {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

trait RPS {
    fn stronger(&self) -> Self;
    fn weaker(&self) -> Self;
    fn against(&self, other: &Self) -> GameResult;
}

impl RPS for Shape {
    fn stronger(&self) -> Self {
        use Shape::*;
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn weaker(&self) -> Self {
        use Shape::*;
        match self {
            Paper => Rock,
            Scissors => Paper,
            Rock => Scissors,
        }
    }    

    fn against(&self, other: &Self) -> GameResult {
        use Shape::*;
        use GameResult::*;

        match (self, other) {
            (Rock, Paper) => Lose,
            (Rock, Scissors) => Win,
            (Rock, Rock) => Draw,
            (Scissors, Paper) => Win,
            (Scissors, Rock) => Lose,
            (Scissors, Scissors) => Draw,
            (Paper, Rock) => Win,
            (Paper, Scissors) => Lose,
            (Paper, Paper) => Draw,
        }
    }
}

impl Shape {
    pub fn from_str(str: &str) -> Self {
        use Shape::*;

        match str {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => unreachable!("Invalid format."),
        }
    }

    pub fn match_result(&self, other: &Self) -> usize {
        self.against(other) as usize + *self as usize
    }
}

// Part 1
fn total_score_1(input: &str) -> usize {
    let mut score = 0;

    for line in input.lines() {
        let (input1, input2) = line.trim().split_once(" ").expect("Invalid format.");
        let (opponent, me) = (Shape::from_str(input1), Shape::from_str(input2));
        score += me.match_result(&opponent);
    }

    score
}

// Part 2
fn total_score_2(input: &str) -> usize {
    let mut score = 0;

    for line in input.lines() {
        let (input1, input2) = line.trim().split_once(" ").expect("Invalid format.");
        let opponent = Shape::from_str(input1);
        let me = match input2 {
            "X" => opponent.weaker(),
            "Y" => opponent,
            "Z" => opponent.stronger(),
            _ => unreachable!("Invalid format."),
        };

        score += me.match_result(&opponent);
    }

    score
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Missing input.txt");
    println!("{}", total_score_2(&input));
}
