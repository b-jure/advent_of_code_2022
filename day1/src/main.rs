#![allow(dead_code)]
use std::fs;

// Part 1
fn max_cal(input: &str) -> usize {
    let mut calories = 0;
    let mut max = 0;

    for line in input.lines() {
        if let Ok(cal) = line.parse::<usize>() {
            calories += cal;
        } else {
            if max < calories {
                max = calories;
            }
            calories = 0;
        }
    }
    
    max
}

// Part 2
fn get_min(m: &[usize; 3]) -> (usize, usize) {
    m.iter()
        .enumerate()
        .skip(1)
        .fold((m[0], 0), |(min, index), (i, val)|  {
            if min > *val { 
                (*val, i)
            } else {
                (min, index)
            }
        })
}

fn top_three(input: &str) -> usize {
    let mut calories = 0;
    let mut max = [0; 3];

    for line in input.lines() {
        if let Ok(cal) = line.parse::<usize>() {
            calories += cal;
        } else {
            let (min, i) = get_min(&max);
            if calories > min {
               max[i] = calories;
            }
            calories = 0;
        }
    }

    max.iter().sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", top_three(&input));
}
