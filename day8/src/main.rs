use day8::{part_1, part_2, Grid};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let grid = Grid::from(&input);

    println!("\n{}", part_1(&grid));
    println!("\n{}", part_2(&grid));
}
