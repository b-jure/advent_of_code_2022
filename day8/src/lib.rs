#![allow(dead_code)]

const ASCII_OFFSET: usize = 48;

#[derive(Debug, PartialEq, Eq)]
pub struct Tree {
    height: usize,
    id: usize,
}

#[derive(Debug)]
pub struct Grid {
    map: Vec<Vec<Tree>>,
}

impl Grid {
    pub fn new() -> Self {
        Grid { map: vec![] }
    }

    pub fn from(input: &str) -> Self {
        let mut grid = Grid::new();
        let mut id = 0;

        grid.set_vectors(&input);

        input.lines().enumerate().for_each(|(i, line)| {
            line.trim().bytes().for_each(|byte| {
                let height = (byte - 48) as usize;
                grid.add_tree(Tree { height, id }, i);
                id += 1;
            })
        });

        grid
    }

    pub fn set_vectors(&mut self, input: &str) {
        for line in input.lines() {
            self.map.push(Vec::with_capacity(line.len()));
        }
    }

    pub fn add_tree(&mut self, tree: Tree, idx: usize) {
        self.map[idx].push(tree);
    }

    pub fn len(&self) -> usize {
        self.map[0].len()
    }

    pub fn height(&self) -> usize {
        self.map.len()
    }

    pub fn scenic_score(&self, idx_row: usize, idx_col: usize, tree: &Tree) -> usize {
        self.left_to_right(idx_row, idx_col, tree).0
            * self.right_to_left(idx_row, idx_col, tree).0
            * self.top_to_bottom(idx_row, idx_col, tree).0
            * self.bottom_to_top(idx_row, idx_col, tree).0
    }

    pub fn is_visible(&self, idx_row: usize, idx_col: usize, tree: &Tree) -> bool {
        let last_column = self.len() - 1;
        let last_row = self.height() - 1;

        match (idx_row, idx_col) {
            (0, _) | (_, 0) => return true,
            (_, y) if y == last_column => return true,
            (x, _) if x == last_row => return true,
            _ => (),
        }

        let (_, right) = self.right_to_left(idx_row, idx_col, tree);
        let (_, left) = self.left_to_right(idx_row, idx_col, tree);
        let (_, top) = self.top_to_bottom(idx_row, idx_col, tree);
        let (_, bottom) = self.bottom_to_top(idx_row, idx_col, tree);

        right || left || bottom || top
    }

    fn right_to_left(&self, idx_row: usize, idx_col: usize, tree: &Tree) -> (usize, bool) {
        let position = idx_col + 1;
        let is_rev = false;
        self.right_and_left(position, is_rev, idx_row, tree)
    }

    fn left_to_right(&self, idx_row: usize, idx_col: usize, tree: &Tree) -> (usize, bool) {
        let position = self.len() - idx_col;
        let is_rev = true;
        self.right_and_left(position, is_rev, idx_row, tree)
    }

    fn bottom_to_top(&self, idx_row: usize, idx_col: usize, tree: &Tree) -> (usize, bool) {
        let position = idx_row + 1;
        let is_rev = false;
        self.top_and_bottom(position, is_rev, idx_col, tree)
    }

    fn top_to_bottom(&self, idx_row: usize, idx_col: usize, tree: &Tree) -> (usize, bool) {
        let position = self.height() - idx_row;
        let is_rev = true;
        self.top_and_bottom(position, is_rev, idx_col, tree)
    }

    fn right_and_left(
        &self,
        position: usize,
        is_rev: bool,
        idx_row: usize,
        tree: &Tree,
    ) -> (usize, bool) {
        let mut count = 1;
        let iterator: Vec<_>;

        match is_rev {
            true => iterator = self.map[idx_row].iter().rev().skip(position).collect(),
            false => iterator = self.map[idx_row].iter().skip(position).collect(),
        }

        for t in iterator {
            if t.height < tree.height {
                count += 1
            } else {
                return (count, false);
            }
        }

        (count - 1, true)
    }

    fn top_and_bottom(
        &self,
        position: usize,
        is_rev: bool,
        idx_col: usize,
        tree: &Tree,
    ) -> (usize, bool) {
        let mut count = 1;
        let iterator: Vec<&Vec<_>>;

        match is_rev {
            true => iterator = self.map.iter().rev().skip(position).collect(),
            false => iterator = self.map.iter().skip(position).collect(),
        }

        for row in iterator {
            if row[idx_col].height < tree.height {
                count += 1
            } else {
                return (count, false);
            }
        }

        (count - 1, true)
    }
}

pub fn part_1(grid: &Grid) -> usize {
    grid.map.iter().enumerate().fold(0, |acc, (row, vec)| {
        acc + vec.iter().enumerate().fold(0, |accum, (col, tree)| {
            if grid.is_visible(row, col, tree) {
                accum + 1
            } else {
                accum
            }
        })
    })
}

pub fn part_2(grid: &Grid) -> usize {
    grid.map.iter().enumerate().fold(0, |max, (idx_row, row)| {
        max.max(
            row.iter()
                .enumerate()
                .fold(0, |score, (idx_col, tree)| score.max(grid.scenic_score(idx_row, idx_col, tree))
            ))})
}