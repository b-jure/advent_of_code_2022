use day10::solution;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Part_2:");
    println!("\n\nPart_1:\n{}", solution(&input));
}