use day7::{parse_input, part_1};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let fs = parse_input(&*input);
    println!("{}", part_1(&fs, 100_000));
    println!("size = {}", fs.size());
    println!("smallest dir to size 30_000_000 = {}", fs.smallest_dir(30_000_000).unwrap().size());
}
