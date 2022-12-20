use std::cmp::{max, min};
use std::collections::HashMap;

type Tuple = (isize, isize);
type Map = HashMap<Tuple, char>;

enum SandOption {
    CONTINUE,
    STOP,
}

const SAND: Tuple = (500, 0);

fn main() {
    let input = include_str!("input.txt");

    let points = parse_input(input);
    let mut map = create_map(&points);

    let first = pour_sand(&mut map, sand_first);
    println!("First {}", first);

    let second = pour_sand(&mut map, sand_second) + first + 1;
    println!("Second {}", second);
}

fn parse_input(input: &str) -> Vec<Vec<Tuple>> {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|tuple| {
                    let raw_tuple = tuple.split_once(",").unwrap();

                    (
                        raw_tuple.0.parse::<isize>().unwrap(),
                        raw_tuple.1.parse::<isize>().unwrap(),
                    )
                })
                .collect()
        })
        .collect()
}

fn create_map(points: &Vec<Vec<Tuple>>) -> Map {
    let mut map = HashMap::new();

    for points in points {
        let mut points = points.iter();

        let mut point_wrap = None;
        let mut prev_point_wrap = None;

        loop {
            if prev_point_wrap == None {
                prev_point_wrap = points.next();
                point_wrap = points.next();
            }

            let prev_point = *prev_point_wrap.unwrap();
            let point = *point_wrap.unwrap();

            if point.0 == prev_point.0 {
                for y in min(point.1, prev_point.1)..max(point.1, prev_point.1) + 1 {
                    let x = point.0;
                    map.insert((x, y), '#');
                }
            }
            if point.1 == prev_point.1 {
                for x in min(point.0, prev_point.0)..max(point.0, prev_point.0) + 1 {
                    let y = point.1;
                    map.insert((x, y), '#');
                }
            }

            match points.next() {
                None => {
                    break;
                }
                Some(p) => {
                    (point_wrap, prev_point_wrap) = (Some(p), point_wrap);
                }
            }
        }
    }

    map
}

fn pour_sand<F: Fn(&Map, &Tuple, isize) -> (SandOption, Tuple)>(
    map: &mut Map,
    sand_fn: F,
) -> isize {
    let y = map.keys().map(|(_, y)| *y).max().unwrap();
    let mut counter = 0;
    'outer: loop {
        let mut sand_position = SAND;

        loop {
            match sand_fn(map, &sand_position, y + 1) {
                (SandOption::STOP, _) => {
                    break 'outer;
                }
                (SandOption::CONTINUE, new_position) => {
                    if new_position == sand_position {
                        map.insert(sand_position, 'O');
                        break;
                    } else {
                        sand_position = new_position
                    }
                }
            }
        }
        counter += 1;
    }
    counter
}

fn sand_first(map: &Map, sand: &Tuple, floor: isize) -> (SandOption, Tuple) {
    if sand.1 >= floor {
        return (SandOption::STOP, *sand);
    }
    (SandOption::CONTINUE, check_sand_position(map, sand))
}

fn sand_second(map: &Map, sand: &Tuple, floor: isize) -> (SandOption, Tuple) {
    if sand.1 == floor {
        return (SandOption::CONTINUE, *sand);
    }

    let new_position = check_sand_position(map, sand);
    if new_position == SAND {
        return (SandOption::STOP, *sand);
    }

    (SandOption::CONTINUE, new_position)
}

fn check_sand_position(map: &Map, sand: &Tuple) -> Tuple {
    let down: Tuple = (sand.0, sand.1 + 1);
    let left: Tuple = (sand.0 - 1, sand.1 + 1);
    let right: Tuple = (sand.0 + 1, sand.1 + 1);

    return match [down, left, right].iter().find(|p| map.get(&p) == None) {
        Some((x, y)) => (*x, *y),
        None => *sand,
    };
}
