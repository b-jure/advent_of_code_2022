use std::collections::{VecDeque, HashMap}; 

#[derive(Clone, Copy)]
pub enum Instruction {
    NoOp,
    Addx(isize),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let value: Vec<&str> = value.split_whitespace().collect();
        match value[0] {
            "addx" => Self::Addx(value[1].parse::<isize>().expect("Invalid instruction")),
            "noop" => Self::NoOp,
            _ => unreachable!("Invalid instruction"),
        }
    }
}

struct Crt {
    pos_x: usize,
    pos_y: usize,
}

impl Crt {
    fn new() -> Self {
        Crt { pos_x: 0, pos_y: 0 }
    }

    fn draw(&mut self, x: isize) -> Result<(), &'static str> {
        self.handle_bounds()?;
        if self.overlaps(x) {
            print!("#");
        } else {
            print!(".")
        }
        self.pos_x += 1;
        Ok(())
    }

    fn handle_bounds(&mut self) -> Result<(), &'static str> {
        if !self.in_bounds_x() {
            print!("\n");
            self.pos_x = 0;
            self.pos_y += 1;
        } else if !self.in_bounds_y() {
            return Err("Out of bounds on y.")
        }
        Ok(())
    }

    fn overlaps(&self, x: isize) -> bool {
        match self.pos_x as isize {
            pos if x - 1 == pos => true,
            pos if x == pos => true,
            pos if x + 1 == pos => true,
            _ => false,
        }
    }

    fn in_bounds_x(&self) -> bool {
        self.pos_x < 40
    }

    fn in_bounds_y(&self) -> bool {
        self.pos_y < 6
    }
}

/// Wrapper struct that stores mapping of our previous states `cycle -> regx`.
struct Log {
    inner: HashMap<usize, isize>,
}

impl Log {
    fn new() -> Self {
        Self::from(HashMap::from([(0, 1)]))
    }

    fn from(map: HashMap<usize, isize>) -> Self {
        Log { inner: map }
    }

    /// Retrieve mapped `regx` values from `entries`.
    /// 
    /// Entries must be `usize` or castable to `usize`, because the `cycle` is of type `usize`.
    fn get_logs<T>(&self, entries: T) -> Vec<isize> 
    where
        T: IntoIterator,
        T::Item: Into<usize>,
    {
        entries.into_iter().fold(vec![], |mut vec, key| { 
            self.inner.get(&key.into()).map(|val| vec.push(*val));
            vec
        })
    }
}

/// Struct representing simple CPU, contains cycle count and registry called regx.
/// 
/// It holds [`Instruction`]s that are to be executed inside a buffer.
/// 
/// After running it saves its previous states inside a log (hashmap).
pub struct Cpu {
    cycle: usize,
    regx: isize,
    buffer: VecDeque<Instruction>,
    executing: bool,
    log: Log,
    crt: Crt,
}

impl Cpu {
    /// [`Cpu`] constructor, default value for `regx` is `1`, `cycle` starts at `0`.
    pub fn new() -> Self {
        Cpu { 
            cycle: 0, 
            regx: 1, 
            buffer: VecDeque::new(), 
            executing: false,
            log: Log::new(),
            crt: Crt::new(),
        }
    }

    /// [`Cpu`] constructor, creates [`Cpu`]˙but also fills up the buffer by parsing the input.
    /// 
    /// `cycle` and `regx` are still at default values of 0 and 1 (respectively).
    pub fn from(input: &str) -> Self {
        let mut cpu = Cpu::new();
        for line in input.lines() {
            let instruction = Instruction::from(line.trim());
            cpu.read_instruction(instruction);
        }
        cpu
    }

    /// Gives out reference to the [`Cpu`] log.
    /// 
    /// Log contains cycles that are mapped to the value of regx.
    pub fn logs<T>(&self, entries: T) -> Vec<isize> 
    where
        T: IntoIterator,
        T::Item: Into<usize>,
    {
        self.log.get_logs(entries)
    }

    /// Draws the sprite at if the position of [`Crt`] overlaps with `regx` value.
    /// 
    /// Returns [`Err`] if the pos_y is outside of [`Crt`] bounds (more than 6 rows).
    pub fn draw(&mut self) -> Result<(), &str>{
        self.crt.draw(self.regx)
    }

    /// Increments cycle count
    #[inline]
    fn next_cycle(&mut self) {
        self.cycle += 1;
    }

    /// Reads the [`Instruction`] and stores it in the buffer at the last position.
    fn read_instruction(&mut self, instruction: Instruction) {
        self.buffer.push_back(instruction);
    }

    /// Runs the [`Cpu`], reading all the [`Instruction`]s and executing them logging all the states cycle-by-cycle.
    pub fn run(&mut self) {
        while let Some(instruction) = self.get_last_instruction() {
            self.executing = true;
            self.execute_instruction(instruction);
        }
    }

    fn get_last_instruction(&mut self) -> Option<Instruction> {
        self.buffer.pop_front()
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        let last_cycle = self.current_cycle();
        while self.executing {
            self.draw().unwrap_or_else(|e| panic!("{e}"));
            self.next_cycle();
            self.log.inner.insert(self.cycle, self.regx);
            self.handle_instruction(&instruction, last_cycle);
        }
    }

    #[inline]
    fn current_cycle(&self) -> usize {
        self.cycle
    }

    fn handle_instruction(&mut self, instruction: &Instruction, last_cycle: usize) {
        match instruction {
            Instruction::Addx(x) => {
                if last_cycle + 2 == self.current_cycle() {
                    self.regx += x;
                    self.executing = false;
                }
            },
            Instruction::NoOp => {
                if last_cycle + 1 == self.current_cycle() {
                    self.executing = false;
                }
            }
        }
    }
}

static CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];

pub fn solution(input: &str) -> isize {
    let mut cpu = Cpu::from(input);
    cpu.run();

    cpu.logs(CYCLES).iter().enumerate().fold(0, |sum, (i, regx)| sum + CYCLES[i] as isize * regx)
}