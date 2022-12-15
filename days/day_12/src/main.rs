use std::collections::vec_deque::VecDeque;
use std::collections::HashMap;

type Tuple = (isize, isize);

const START: u8 = b'S';
const END: u8 = b'E';

fn main() {
    let input = include_str!("input.txt");
    let (graph, starting_point, end_point) = get_graph(input);

    let first = bfs(&graph, starting_point, |position, _| position == end_point);
    let second = bfs(&graph, starting_point, |_, symbol| symbol == b'a');

    println!("First {}", first);
    println!("Second {}", second);
}

fn get_graph(input: &str) -> (HashMap<Tuple, u8>, Tuple, Tuple) {
    let mut graph = HashMap::with_capacity(input.len());
    let mut starting_point = (0, 0);
    let mut end_point = (0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, &char) in line.as_bytes().iter().enumerate() {
            let x = x as isize;
            let y = y as isize;

            // Yes, they are actually switched
            if char == START {
                end_point = (x, y);
                graph.insert((x, y), b'a');
            } else if char == END {
                starting_point = (x, y);
                graph.insert((x, y), b'z');
            } else {
                graph.insert((x, y), char);
            }
        }
    }

    (graph, starting_point, end_point)
}

fn bfs<F: Fn(Tuple, u8) -> bool>(graph: &HashMap<Tuple, u8>, start: Tuple, end: F) -> usize {
    let mut frontier: VecDeque<Tuple> = VecDeque::new();
    let mut visited: HashMap<Tuple, Tuple> = HashMap::with_capacity(graph.len());

    frontier.push_back(start);
    visited.insert(start, start);

    let mut p: Tuple = (0, 0);
    while !frontier.is_empty() {
        p = frontier.pop_front().unwrap();
        let height = *graph.get(&p).unwrap();

        if end(p, height) {
            break;
        }

        let neighbours = get_neighbours(p);

        for n in neighbours {
            let neighbour = graph.get(&n);
            match neighbour {
                None => (),
                Some(&n_height) => {
                    if !visited.contains_key(&n) && height - 1 <= n_height {
                        visited.insert(n, p);
                        frontier.push_back(n);
                    }
                }
            };
        }
    }

    let mut path: Vec<Tuple> = Vec::new();
    let mut from = p;

    path.push(from);
    loop {
        let &to = visited.get(&from).unwrap();
        if to == start {
            break;
        }

        path.push(to);
        from = to;
    }

    path.reverse();
    path.len()
}

fn get_neighbours((x, y): Tuple) -> [Tuple; 4] {
    [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)]
}
