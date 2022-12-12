use std::collections::HashMap;

const CRLF: &str = "\r\n\r\n";
const LF: &str = "\n\n";

fn main() {
    let input = include_str!("input.txt");

    let empty_line_string = if input.contains(CRLF) { CRLF } else { LF };
    let empty_line = input.find(empty_line_string).unwrap();

    let (cargo, steps) = (&input[..empty_line], &input[empty_line + 2..]);

    let mut towers9000 = process_cargo(cargo);
    let mut towers9001 = towers9000.clone();
    let steps = process_steps(steps);

    for (count, from, to) in steps {
        let mut batch = Vec::new();
        for _ in 0..count {
            let item9000 = towers9000.get_mut(&from).unwrap().pop().unwrap();
            towers9000.get_mut(&to).unwrap().push(item9000);

            let item9001 = towers9001.get_mut(&from).unwrap().pop().unwrap();
            batch.push(item9001);
        }
        for _ in 0..count {
            let item = batch.pop().unwrap();
            towers9001.get_mut(&to).unwrap().push(item);
        }
    }

    let mut first = String::new();
    let mut second = String::new();

    for i in 0..towers9000.len() {
        first.push(towers9000.get_mut(&(i + 1)).unwrap().pop().unwrap());
    }
    for i in 0..towers9001.len() {
        second.push(towers9001.get_mut(&(i + 1)).unwrap().pop().unwrap());
    }

    println!("First: {}", first);
    println!("Second: {}", second);
}

fn process_cargo(cargo: &str) -> HashMap<usize, Vec<char>> {
    let mut towers = HashMap::new();

    for line in cargo.lines().rev().skip(1) {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(i, c)| match c {
                ' ' => (),
                c => towers.entry(i + 1).or_insert(Vec::new()).push(c),
            });
    }

    towers
}

fn process_steps(input: &str) -> Vec<(usize, usize, usize)> {
    let mut steps = Vec::new();

    for line in input.lines() {
        let v = line
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        steps.push((v[0], v[1], v[2]));
    }

    steps
}
