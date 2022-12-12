use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Monkey<'a> {
    items: Vec<usize>,
    operation: (&'a str, &'a str, &'a str),
    test: (usize, usize, usize),
}

fn main() {
    let input = include_str!("input.txt");
    let monkeys = process_input(input);
    let mut first_inspect_count: Vec<usize> = vec![0; monkeys.len()];
    let mut second_inspect_count: Vec<usize> = vec![0; monkeys.len()];

    let mut monkeys_clone_1 = monkeys.clone();
    for _ in 0..20 {
        play_round(
            &mut monkeys_clone_1,
            &mut first_inspect_count,
            |x: usize| x / 3,
        );
    }
    first_inspect_count.sort();
    first_inspect_count.reverse();

    let common_divisor = monkeys
        .iter()
        .map(|m: &Monkey| m.test.0)
        .reduce(|acc, div| acc * div)
        .unwrap();

    let mut monkeys_clone_2 = monkeys.clone();
    for _ in 0..10_000 {
        play_round(
            &mut monkeys_clone_2,
            &mut second_inspect_count,
            |x: usize| x % common_divisor,
        );
    }

    second_inspect_count.sort();
    second_inspect_count.reverse();

    println!("First: {}", first_inspect_count[0] * first_inspect_count[1]);
    println!(
        "Second: {}",
        second_inspect_count[0] * second_inspect_count[1]
    );
}

fn process_input(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let mut lines = input.lines();

    while let Some(_) = lines.next() {
        let items: Vec<usize> = lines.next().unwrap()[18..]
            .split(", ")
            .map(|c| c.parse::<usize>().unwrap())
            .collect();

        let operation: (&str, &str, &str) = lines.next().unwrap()[19..]
            .split_whitespace()
            .collect_tuple()
            .unwrap();

        let divisible = lines.next().unwrap()[21..].parse::<usize>().unwrap();
        let truly = lines.next().unwrap()[29..].parse::<usize>().unwrap();
        let falsy = lines.next().unwrap()[30..].parse::<usize>().unwrap();
        let test = (divisible, truly, falsy);
        lines.next();

        let monkey = Monkey {
            items,
            operation,
            test,
        };

        monkeys.push(monkey);
    }

    monkeys
}

fn play_round<F: Fn(usize) -> usize>(
    monkeys: &mut [Monkey],
    inspect_count: &mut [usize],
    relief_function: F,
) {
    for index in 0..monkeys.len() {
        let items = monkeys[index].items.drain(..).collect::<Vec<_>>();
        let (first, operation, second) = monkeys[index].operation;
        let (divisible, truly, falsy) = monkeys[index].test;

        for mut item in items {
            inspect_count[index] += 1;
            let first = if first == "old" {
                item
            } else {
                first.parse::<usize>().unwrap()
            };
            let second = if second == "old" {
                item
            } else {
                second.parse::<usize>().unwrap()
            };

            match operation {
                "+" => {
                    item = first + second;
                }
                "-" => {
                    item = first - second;
                }
                "*" => {
                    item = first * second;
                }
                _ => {
                    item = first / second;
                }
            }

            item = relief_function(item);

            let other_monkey_index = if item % divisible == 0 { truly } else { falsy };

            monkeys[other_monkey_index].items.push(item);
        }
    }
}
