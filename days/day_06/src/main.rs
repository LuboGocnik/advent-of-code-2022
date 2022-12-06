fn main() {
    let input = include_str!("input.txt");

    println!("First: {}", solve_puzzle(input, 4));
    println!("Second: {}", solve_puzzle(input, 14));
}

fn solve_puzzle(input: &str, block_size: usize) -> usize {
    let mut result: usize = 0;
    let mut last_chars: Vec<char> = Vec::new();

    for char in input.chars() {
        result += 1;
        if last_chars.len() == block_size - 1 && !last_chars.contains(&char) {
            break;
        } else {
            if last_chars.contains(&char) {
                loop {
                    let removed_char = last_chars.remove(0);
                    if removed_char == char {
                        break;
                    }
                }
            }
            last_chars.push(char)
        }
    }

    result
}
