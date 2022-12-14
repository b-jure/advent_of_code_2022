use day9::{Rope, all_parts};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut rope = Rope::new();
    rope.add_knots(10);

    println!("{}", all_parts(&input, &mut rope));
}
