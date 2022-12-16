use day11::part1;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let monkey_bussiness = part1(&input).unwrap_or_else(|e| {
        println!("Error --> {e}");
        std::process::exit(1);
    });

    println!("Part 1: {}", monkey_bussiness);
}
