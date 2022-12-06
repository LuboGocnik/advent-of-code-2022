use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

const PRIORITY: &str = "0abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let input = include_str!("input.txt");

    println!("First: {}", first_part(input));
    println!("Second: {}", second_part(input));
}

fn get_common_char(parts: &[&str]) -> char {
    let mut frequency: HashMap<char, usize> = HashMap::new();
    let len = parts.len();

    for &part in parts {
        let mut occurence: HashSet<char> = HashSet::new();

        part.chars().for_each(|c| {
            if !occurence.contains(&c) {
                *frequency.entry(c).or_insert(0) += 1;
                occurence.insert(c);
            }
        })
    }

    let mut result: char = '0';
    for (&c, &f) in frequency.iter() {
        if f == len {
            result = c;
        }
    }

    result
}

fn first_part(input: &str) -> usize {
    let mut result: usize = 0;

    for line in input.lines() {
        let half = line.len() / 2;
        let char = get_common_char(&[&line[..half], &line[half..]]);
        result += PRIORITY.chars().position(|c| c == char).unwrap();
    }

    result
}

fn second_part(input: &str) -> usize {
    let mut result: usize = 0;

    for (first, second, third) in input.lines().tuples() {
        let char = get_common_char(&[first, second, third]);
        result += PRIORITY.chars().position(|c| c == char).unwrap();
    }

    result
}
