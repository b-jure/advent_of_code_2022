#![allow(dead_code)]
use std::ops::Deref;
use std::ptr::NonNull;

// Works for arbitrary number of knots.

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "L" => Self::Left,
            "R" => Self::Right,
            "D" => Self::Down,
            "U" => Self::Up,
            _ => unreachable!("Invalid direction bozo."),
        }
    }
}

// Basically a linked-list
pub struct Rope {
    head: Link,
    tail: Link,
}

impl Rope {
    pub fn new() -> Self {
        Rope { 
            head: None,
            tail: None,
        }
    }

    pub fn add_knot(&mut self) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Knot::new(None, None))));
            if let Some(old) = self.head {
                (*old.as_ptr()).head = Some(new);
                (*new.as_ptr()).tail = Some(old);
                (*new.as_ptr()).position = (*old.as_ptr()).position;
            } else {
                self.tail = Some(new);
            }
            self.head = Some(new);
        }
    }

    pub fn add_knots(&mut self, amount: usize) {
        for _ in 0..amount {
            self.add_knot();
        }
    }

    pub fn pop_knot(&mut self) -> Option<Knot> {
        self.head.map(|knot| {
            unsafe {
                let old_head = Box::from_raw(knot.as_ptr());
                self.head = old_head.tail;
                if let Some(new_head) = self.head {
                    (*new_head.as_ptr()).head = None;
                } else {
                    self.tail = None;
                }
                *old_head
            }
        })
    }

    fn head_mut(&mut self) -> Option<&mut Knot> {
        unsafe { self.head.map(|knot| &mut *knot.as_ptr()) }
    }

    pub fn move_head(&mut self, direction: Direction, amount: isize) -> Option<Vec<Pos>> {
        self.head_mut().map(|head| {
            head.move_at_dir(direction, amount);
            let mut positions: Vec<Pos> = vec![];
            let mut temp_head = head;

            while let Some(tail) = temp_head.tail_mut() {
                while let Some(pos) = tail.move_tail() {
                    if tail.tail.is_none() {
                        positions.push(pos);
                    }
                }
                temp_head = tail;
            }
            positions
        })
    }
}

trait GridMovement {
    fn move_at_dir(&mut self, direction: Direction, amount: isize) {
        match direction {
            Direction::Down => self.move_down(amount),
            Direction::Up => self.move_up(amount),
            Direction::Right => self.move_right(amount),
            Direction::Left => self.move_left(amount),
            Direction::UpLeft => {
                self.move_up(amount);
                self.move_left(amount);
            },
            Direction::UpRight => {
                self.move_up(amount);
                self.move_right(amount);
            },
            Direction::DownLeft => {
                self.move_down(amount);
                self.move_left(amount);
            },
            Direction::DownRight => {
                self.move_down(amount);
                self.move_right(amount);
            },
        }
    }

    fn move_right(&mut self, amount: isize);
    fn move_left(&mut self, amount: isize); 
    fn move_up(&mut self, amount: isize); 
    fn move_down(&mut self, amount: isize);
}

#[derive(Clone, Copy, Debug)]
pub struct Knot {
    head: Link,
    tail: Link,
    position: Pos,
}

type Link = Option<NonNull<Knot>>;

impl Knot {
    fn new(head: Link, tail: Link) -> Self {
        Knot { head, tail, position: Pos((0, 0))}
    }

    fn head(&self) -> Option<&Knot> {
        unsafe { self.head.map(|head| head.as_ref()) }
    }

    fn head_mut(&mut self) -> Option<&mut Knot> {
        unsafe { self.head.map(|head| &mut *head.as_ptr()) }
    }

    fn tail_mut(&mut self) -> Option<&mut Knot> {
        unsafe { self.tail.map(|tail| &mut *tail.as_ptr()) }
    }

    fn is_adjacent(&self) -> bool {
        let head = self.head().unwrap();

        let x_diff = (self.position.x() - head.position.x()).abs();
        let y_diff = (self.position.y() - head.position.y()).abs();
            
        x_diff <= 1 && y_diff <= 1
    }

    fn get_axis(&self) -> Option<Direction> {
        use Direction::*;
        let (other_x, other_y) = *self.head().unwrap().position;
        match self.position.0 {
            (x, y) if x == other_x && y > other_y => Some(Down),
            (x, y) if x == other_x && y < other_y => Some(Up),
            (x, y) if y == other_y && x < other_x => Some(Right),
            (x, y) if y == other_y && x > other_x => Some(Left),
            _ => None,
        }
    }

    fn get_diagonal(&self) -> Option<Direction> {
        use Direction::*;
        let head = self.head().unwrap();
        let (h_x, h_y) = *head.position;

        match *self.position {
            (x, y) if x < h_x && y < h_y => Some(UpRight),
            (x, y) if x > h_x && y < h_y => Some(UpLeft),
            (x, y) if x < h_x && y > h_y => Some(DownRight),
            (x, y) if x > h_x && y > h_y => Some(DownLeft),
            _ => None,
        }
    }

    fn move_tail(&mut self) -> Option<Pos> {
        self.get_tail_direction().map(|direction| {
            self.move_at_dir(direction, 1);
            self.position
        })
    }
    
    fn get_tail_direction(&self) -> Option<Direction> {
        if self.is_adjacent() { return None }
        let Some(direction) = self.get_axis() else {
            return self.get_diagonal()
        };
        Some(direction)
    }
}

impl GridMovement for Knot {
    fn move_right(&mut self, amount: isize) {
        self.position.add_x(amount);
    }

    fn move_left(&mut self, amount: isize) {
        self.position.sub_x(amount);
    }

    fn move_up(&mut self, amount: isize) {
        self.position.add_y(amount);
    }

    fn move_down(&mut self, amount: isize) {
        self.position.sub_y(amount);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pos((isize, isize));

impl Pos {
    fn add_x(&mut self, amount: isize) {
        self.0.0 += amount;
    }

    fn sub_x(&mut self, amount: isize) {
        self.0.0 -= amount;
    }

    fn add_y(&mut self, amount: isize) {
        self.0.1 += amount;
    }

    fn sub_y(&mut self, amount: isize) {
        self.0.1 -= amount;
    }

    pub fn x(&self) -> isize {
        self.0.0
    }

    pub fn y(&self) -> isize {
        self.0.1
    }
}

impl Deref for Pos {
    type Target = (isize, isize);
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn all_parts(input: &str, rope: &mut Rope) -> usize {
    use std::collections::HashSet;
    let mut hashset = HashSet::from([(0, 0)]);

    for line in input.lines() {
        if let Some(position) = line.trim().find(" ") {
            let direction = Direction::from(&line[..position]);
            let amount = line[position+1..].parse::<isize>().expect("Invalid amount");

            rope.move_head(direction, amount)
                .map(|positions| 
                        positions
                            .into_iter()
                            .for_each(|pos| { 
                                hashset.insert(*pos); 
                            }));
        } else {
            continue;
        }
    }
    hashset.len()
}