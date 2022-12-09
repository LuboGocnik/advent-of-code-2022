use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let (grid, max_x, max_y) = build_grid(input);

    let mut visible_tree_counter = 0;
    let mut max_score = 0;

    for y in 0..max_y {
        for x in 0..max_x {
            let (is_visible, score) = check_tree(&grid, (max_x, max_y), (x, y));
            if is_visible {
                visible_tree_counter += 1;
            }
            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("First: {}", visible_tree_counter);
    println!("Second: {}", max_score);
}

fn build_grid(input: &str) -> (HashMap<(usize, usize), usize>, usize, usize) {
    let mut grid: HashMap<(usize, usize), usize> = HashMap::new();
    let mut y = 0;
    let mut x = 0;
    for line in input.lines() {
        x = 0;
        for char in line.chars() {
            grid.insert((x, y), usize::try_from(char.to_digit(10).unwrap()).unwrap());
            x += 1;
        }
        y += 1;
    }

    (grid, x, y)
}

fn check_tree(
    grid: &HashMap<(usize, usize), usize>,
    (max_x, max_y): (usize, usize),
    (x, y): (usize, usize),
) -> (bool, usize) {
    let mut is_visible = false;
    let mut path_clear = true;

    let mut up = 0;
    let mut down = 0;
    let mut left = 0;
    let mut right = 0;

    let tree_height = *grid.get(&(x, y)).unwrap();

    // right
    path_clear = true;
    for i in x + 1..max_x {
        right += 1;
        let height = *grid.get(&(i, y)).unwrap();
        if height >= tree_height {
            path_clear = false;
            break;
        }
    }
    if path_clear {
        is_visible = true;
    }

    // left
    path_clear = true;
    for i in 1..x + 1 {
        left += 1;
        let height = *grid.get(&(x - i, y)).unwrap();
        if height >= tree_height {
            path_clear = false;
            break;
        }
    }
    if path_clear {
        is_visible = true;
    }

    // down
    path_clear = true;
    for j in y + 1..max_y {
        down += 1;
        let height = *grid.get(&(x, j)).unwrap();
        if height >= tree_height {
            path_clear = false;
            break;
        }
    }
    if path_clear {
        is_visible = true;
    }

    // up
    path_clear = true;
    for j in 1..y + 1 {
        up += 1;
        let height = *grid.get(&(x, y - j)).unwrap();
        if height >= tree_height {
            path_clear = false;
            break;
        }
    }
    if path_clear {
        is_visible = true;
    }

    (is_visible, up * down * left * right)
}
