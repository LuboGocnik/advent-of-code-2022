use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let empty_line = input.find("\n\n").unwrap();
    let (cargo, steps) = (&input[..empty_line], &input[empty_line + 2..]);

    let mut towers9000 = process_cargo(cargo);
    let mut towers9001 = towers9000.clone();
    let steps = process_steps(steps);

    for (count, from, to) in steps {
        let mut batch = Vec::new();
        for _ in 0..count {
            let item = towers9001.get_mut(&from).unwrap().pop().unwrap();
            batch.push(item);

            let item = towers9000.get_mut(&from).unwrap().pop().unwrap();
            towers9000.get_mut(&to).unwrap().push(item);
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

    let mut lines = cargo.lines().rev();
    lines.next();

    for line in lines {
        let size = (line.len() + 2) / 4;
        let mut line = line.chars();
        for index in 0..size {
            match line.nth(if index == 0 { 1 } else { 3 }).unwrap() {
                ' ' => continue,
                char => towers.entry(index + 1).or_insert(Vec::new()).push(char),
            }
        }
    }

    towers
}

fn process_steps(input: &str) -> Vec<(usize, usize, usize)> {
    let mut steps = Vec::new();
    for line in input.lines() {
        let mut line = line.split_whitespace();

        let count = line.nth(1).unwrap();
        let from = line.nth(1).unwrap();
        let to = line.nth(1).unwrap();

        let [count, from, to] = [count, from, to].map(|c| c.parse::<usize>().unwrap());
        steps.push((count, from, to));
    }

    steps
}
