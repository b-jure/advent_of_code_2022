#![allow(dead_code)]

use error::{ParsingError, Result};
use std::{collections::VecDeque, fmt::Write};
pub mod error;

struct Item {
    worry_level: isize,
}

impl From<isize> for Item {
    fn from(value: isize) -> Self {
        Item { worry_level: value }
    }
}

type Op = Box<dyn Fn(isize) -> isize>;
type Test = Box<dyn Fn(isize) -> usize>;

struct Monkey {
    items: VecDeque<Item>,
    operation: Op,
    test: Test,
    inspected: usize,
}

impl Monkey {
    fn from(buffer: &str, game: &Game) -> Result<Self> {
        let lines: Vec<&str> = buffer.lines().collect();
        assert!(lines.len() == 6);
        
        Monkey::parse_id(lines[0], game)?;
        let items = Monkey::parse_items(lines[1])?;
        let operation = Monkey::parse_operation(lines[2])?;
        let test_args: Vec<&str> = lines.into_iter().skip(3).collect();
        assert!(test_args.len() == 3);

        let test = Monkey::parse_test(test_args)?;
        Ok(Monkey { items, operation, test, inspected: 0 })
    }

    fn push_item(&mut self, item: Item) {
        self.items.push_back(item);
    }

    fn current_item(&self) -> &Item {
        self.items.front().unwrap()
    }

    fn pop_item(&mut self) -> Option<Item> {
        self.inspected += 1;
        self.items.pop_front().map(|mut item| {
            let new = (self.operation)(item.worry_level) / 3;
            item.worry_level = new;
            item
        })
    }

    fn no_items(&self) -> bool {
        self.items.is_empty()
    }
}

struct Game {
    monkeys: Vec<Monkey>,
    round: usize,
}

impl Game {
    pub fn new() -> Self {
        Game { monkeys: Vec::new(), round: 0 }
    }

    pub fn from(input: &str) -> Result<Self> {
        let mut game = Game::new();
        let mut buffer = String::new();
        let last_line = input.lines().last().ok_or(ParsingError)?;
        let mut i = 0;

        'outer: for line in input.lines() {
            while i < 6 {
                i += 1; 
                buffer.write_str(line.trim()).unwrap();
                buffer.write_char('\n').unwrap();
                if last_line != line {
                    continue 'outer;
                } else {
                    break
                }
            }
            let monkey = Monkey::from(&buffer, &game)?;
            game.monkeys.push(monkey);
            i = 0;
            buffer.clear();
        }
        Ok(game)
    }

    /// Returns highest scores (inspected items count), you can define how many scores to retrieve.
    pub fn highest_scores(&mut self, count: usize) -> Option<Vec<isize>> {
        if count >= self.monkeys.len() { 
            return None 
        }

        self.monkeys.sort_by_key(|monkey| monkey.inspected);

        Some(self.monkeys
            .iter()
            .rev()
            .enumerate()
            .take_while(|(i, _)| *i < count)
            .map(|(_, monkey)| monkey.inspected as isize)
            .collect())
    }

    pub fn play_until(&mut self, round: usize) {
        for _ in 0..round {
            self.start_round();
        }
    }

    pub fn start_round(&mut self) {
        for i in 0..self.monkeys.len() {
            self.inspect_items(i);
        }
        self.round += 1;
    }

    pub fn inspect_items(&mut self, current: usize) {
        while !self.monkeys[current].no_items() { 
            let curr_monkey = &self.monkeys[current];
            let other_idx = {
                let curr_item = curr_monkey.current_item();
                let after_inspection = (curr_monkey.operation)(curr_item.worry_level) / 3;
                (curr_monkey.test)(after_inspection)
            };
            self.send_item(current, other_idx);
        }
    }

    fn send_item(&mut self, from: usize, to: usize) {
        self.monkeys[from].pop_item().map(|item| self.monkeys[to].push_item(item));
    }
}


// Text parsing section
impl Monkey {
    fn parse_id(line: &str, game: &Game) -> Result<()> {
        let (name, idx) = line
            .trim_end_matches(|c| c == ':' || c == ' ')
            .split_once(' ').ok_or(ParsingError)?;
        
        let idx = idx.parse::<usize>()?;

        match (name, idx) {
            ("Monkey", i) if i == game.monkeys.len() => Ok(()),
            _ => return Err(ParsingError),
        }
    }

    fn parse_items(line: &str) -> Result<VecDeque<Item>> {
        let mut items = VecDeque::new();
        if let Some(i) = line.find(":") {
            for elem in line[i+1..].trim().split_terminator(",") {
                let int = elem.trim().parse::<isize>()?;
                items.push_front(int.into());
            }
        }
        Ok(items)
    }

    fn parse_operation(line: &str) -> Result<Op> {
        if let Some(i) = line.find("=") {
            let ln: Vec<&str> = line[i+1..].trim().split_whitespace().collect();
            if ln.len() != 3 { return Err(ParsingError) }
            let operator = ln[1];
            let modifier = ln[2];

            let value = if let Ok(val) = modifier.parse::<isize>() { Some(val) } else { None };

            match (operator, modifier) {
                ("+", "old") => Ok(Box::new(|old| old + old)),
                ("-", "old") => Ok(Box::new(|old| old - old)),
                ("*", "old") => Ok(Box::new(|old| old * old)),
                ("/", "old") => Ok(Box::new(|old| old / old)),
                ("+", _) if value.is_some() => Ok(Box::new(move |old| old + value.unwrap())),
                ("-", _) if value.is_some() => Ok(Box::new(move |old| old - value.unwrap())),
                ("*", _) if value.is_some() => Ok(Box::new(move |old| old * value.unwrap())),
                ("/", _) if value.is_some() => Ok(Box::new(move |old| old / value.unwrap())),
                _ => Err(ParsingError),
            }
        } else {
            Err(ParsingError)
        }
    }

    fn parse_test(lines: Vec<&str>) -> Result<Test> {
        match (lines[1].trim(), lines[2].trim()) {
            (line1, line2) if line1.starts_with("If true:") && line2.starts_with("If false:") => {
                // Can rewrite it as --┐
                //                     v 
                //                    let true_idx: usize = line1[line1.len()-1..].parse()?;  <-- This would still work for the input.
                //
                // But this way it works for integers that are > 9
                let true_idx: usize = line1
                    .split_whitespace()
                    .last()
                    .ok_or(ParsingError)?
                    .parse()?;

                let false_idx: usize = line2
                    .split_whitespace()
                    .last()
                    .ok_or(ParsingError)?
                    .parse()?;

                let divisor = Monkey::parse_test_divisor(lines[0].trim())?;

                Ok(Box::new(move |worry_level| {
                    if worry_level % divisor == 0 {
                        true_idx
                    } else {
                        false_idx
                    }
                }))
            },
            _ => {
                Err(ParsingError)
            },
        }
    }

    fn parse_test_divisor(line: &str) -> Result<isize> {
        if line.starts_with("Test:") {
            Ok(line.trim()
                .split_whitespace()
                .last()
                .ok_or(ParsingError)?
                .parse::<isize>()?)
        } else {
            Err(ParsingError)
        }
    }
}

pub fn part1(input: &str) -> Result<isize> {
    let mut game = Game::from(input)?;

    game.play_until(20);

    let highscores = game.highest_scores(2).unwrap();
    assert!(highscores.len() == 2);
    
    Ok(highscores[0] * highscores[1])
}