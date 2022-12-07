#![allow(dead_code)]
use std::collections::HashMap;

const LOWERCASE_OFFSET: usize = 96;
const UPPERCASE_OFFSET: usize = 38;

enum Error {
    InvalidInput,
    Duplicate(Item)
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Item {
    inner: char,
}

impl Item {
    pub fn priority(&self) -> usize {
        let char = self.inner;

        if char.is_lowercase() {
            char as usize - LOWERCASE_OFFSET
        } else {
            char as usize - UPPERCASE_OFFSET
        }
    }

    pub fn from_char(char: char) -> Self {
        match char {
            'a'..='z' | 'A'..='Z' => Item { inner: char },
            _ => unreachable!("Invalid item"),
        }
    }
}

struct Compartment {
    inner: HashMap<Item, usize>,
    index: usize,
} 

impl Compartment {
    pub fn new(index: usize) -> Self {
        Compartment { inner: HashMap::new(), index }
    }

    fn contains(&self, item: &Item) -> bool {
        self.inner.contains_key(item)
    }
}

struct Rucksack {
    compartments: Vec<Compartment>,
}

impl Rucksack {
    pub fn new() -> Self {
        Rucksack {
            compartments: vec![],
        }
    }

    pub fn add_comp(&mut self, num: usize) {
        for _ in 0..num {
            let index = self.size();
            self.compartments.push(Compartment::new(index));
        }
    }

    pub fn rm_comp(&mut self, num: usize) -> Result<(), usize> {
        let len = self.compartments.len();

        if len >= num { 
            for _ in 0..num {
                self.compartments.pop();
            }
            Ok(()) 
        } else {
            Err(len)
        }
    }

    pub fn size(&self) -> usize {
        self.compartments.len()
    }

    pub fn fill_compartments(&mut self, input: &str) -> Result<(), Error> {
        for line in input.lines() {
            let items = self.split_line(line.trim())?;
            for (i, compartment_items) in items.iter().enumerate() {
                if i == 0 { 
                    self.init_first_compartment(compartment_items) 
                } else {
                    self.fill_compartment(compartment_items, i)?
                }
            }
        }
        Ok(())
    }
}

impl Rucksack {
    fn get_chunks(size: usize, line: &str) -> Result<Vec<&str>, Error> {
        if line.len() % size != 0 || size > line.len() {
            Err(Error::InvalidInput)
        } else {
            Ok(line.as_bytes()
                .chunks_exact(size)
                .map(|buf| unsafe { std::str::from_utf8_unchecked(buf) })
                .collect::<Vec<&str>>())
        }
    }

    fn split_line(&self, line: &str) -> Result<Vec<Vec<Item>>, Error> {
        let chunks = Rucksack::get_chunks(line.len()/self.size(), line)?;
        let mut items = vec![vec![]; self.size()];
        for (i, chunk) in chunks.iter().enumerate() {
            for char in chunk.chars() {
                let item = Item::from_char(char);
                items[i].push(item);
            }
        }
        Ok(items)
    }

    fn init_first_compartment(&mut self, items: &[Item]) {
        let mut compartment = HashMap::new();

        items.into_iter()
            .copied()
            .for_each(|item| *compartment.entry(item).or_insert(1) += 1);

        self.compartments[0] = Compartment { inner: compartment, index: 0 };
    }

    fn fill_compartment(&mut self, items: &[Item], index: usize) -> Result<(), Error> {
        let mut compartment = HashMap::new();

        for item in items.into_iter().copied() {
            if self.contains_duplicate(&item) {
                return Err(Error::Duplicate(item))
            } else {
                *compartment.entry(item).or_insert(1) += 1
            }
        }

        self.compartments[index] = Compartment { inner: compartment, index };
        Ok(())
    }

    fn contains_duplicate(&self, item: &Item) -> bool {
        self.compartments.iter().any(|compartment| compartment.contains(&item))
    }
}

// Part 1
pub fn get_prio_sum_1(input: &str) -> usize {
    let mut rucksack = Rucksack::new();
    rucksack.add_comp(2);
    let mut result = 0;

    for line in input.lines() {
        if let Err(Error::Duplicate(item)) = rucksack.fill_compartments(line) {
            result += item.priority();
        }
    }
    
    result
}

use std::collections::HashSet;

// Part 2
pub fn get_prio_sum_2(input: &str) -> usize {
    let mut group: Vec<&str> = Vec::with_capacity(3);
    let mut sets: Vec<HashSet<Item>> = vec![HashSet::new(); 3];
    let mut result = 0;

    for (i, line) in input.lines().enumerate() {
        if (i+1) % 3 == 0 {
            group.push(line.trim());
            for (i, l) in group.iter().enumerate() {
                for char in l.chars() {
                    sets[i].insert(Item::from_char(char));
                }
            }

            result += find_badge(&sets).priority();

            group.clear();
            sets.iter_mut().for_each(|set| set.clear());
        } else {
            group.push(line.trim());
        }
    }

    result
}

fn biggest_set(sets: &Vec<HashSet<Item>>) -> (&HashSet<Item>, usize, usize)  {
    match (sets[0].len(), sets[1].len(), sets[2].len()) {
        (set0, set1, set2) if set0 >= set1 && set0 >= set2 => (&sets[0], 1, 2),
        (set0, set1, set2) if set1 >= set0 && set1 >= set2 => (&sets[1], 0, 2),
        (set0, set1, set2) if set2 >= set1 && set2 >= set0 => (&sets[2], 0, 1),
        _ => unreachable!(),
    }
}

fn find_badge(sets: &Vec<HashSet<Item>>) -> Item {
    let (set, idx_1, idx_2) = biggest_set(sets);
    for item in set.iter() {
        if sets[idx_1].contains(&item) && sets[idx_2].contains(&item) {
            return *item
        }
    }
    unreachable!("Invalid input");
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let result = get_prio_sum_2(&input);
    println!("{result}");
}