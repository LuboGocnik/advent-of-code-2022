use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    let mut tail_1_positions: HashSet<(isize, isize)> = HashSet::new();
    let mut tail_9_positions: HashSet<(isize, isize)> = HashSet::new();

    let mut knots = [(0, 0); 10];

    for line in input.lines() {
        let (direction, steps) = (&line[..1], &line[2..]);
        let steps = steps.parse::<usize>().unwrap();

        let (x, y) = match direction {
            "R" => (1, 0),
            "L" => (-1, 0),
            "D" => (0, 1),
            _ => (0, -1),
        };

        for _ in 0..steps {
            knots[0] = (knots[0].0 + x, knots[0].1 + y);

            for i in 1..knots.len() {
                knots[i] = move_knot(knots[i - 1], knots[i]);
            }

            tail_1_positions.insert(knots[1]);
            tail_9_positions.insert(knots[9]);
        }
    }
    println!("First: {}", tail_1_positions.len());
    println!("Second: {}", tail_9_positions.len());
}

fn move_knot(knot1: (isize, isize), knot2: (isize, isize)) -> (isize, isize) {
    let (hx, hy) = knot1;
    let (tx, ty) = knot2;

    if (hx - tx).abs() > 1 && (hy - ty).abs() > 1 {
        return ((hx + tx) / 2, (hy + ty) / 2);
    }

    if (hx - tx).abs() > 1 {
        return ((hx + tx) / 2, hy);
    }
    if (hy - ty).abs() > 1 {
        return (hx, (hy + ty) / 2);
    }

    (tx, ty)
}
