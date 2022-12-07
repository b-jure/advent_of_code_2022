use std::{collections::HashSet, fs, hash::Hash};

fn signal_start(input: &str) -> usize {
    let chars: Vec<char> = input.chars().collect();
    let mut marker_chars = &chars[0..14];

    for i in (0..chars.len()).skip(15) {
        if are_unique(marker_chars) {
            return i - 1
        } else {
            marker_chars = &chars[i-14..i];
        }
    }

    unreachable!("There is no solution")
}

fn are_unique<T, E>(elements: T) -> bool 
where 
    T: IntoIterator::<Item = E>,
    E: Hash + Eq,
{
    let mut hashset = HashSet::new();
    elements.into_iter().all(|char| hashset.insert(char))
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", signal_start(&input));
}
