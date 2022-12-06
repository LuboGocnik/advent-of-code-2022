fn main() {
    let input = include_str!("input.txt");

    println!("First: {}", first_part(input));
    println!("Second: {}", second_part(input))
}

fn get_position(rules: &str, me: char) -> usize {
    rules.chars().position(|c| c == me).unwrap()
}

fn first_part(input: &str) -> usize {
    let mut score: usize = 0;
    for line in input.lines() {
        let me = line.chars().nth(2).unwrap();
        let opponent = match line.chars().nth(0).unwrap() {
            'A' => "ZXY",
            'B' => "XYZ",
            _ => "YZX",
        };

        match me {
            'X' => {
                score += 1;
            }
            'Y' => {
                score += 2;
            }
            'Z' => {
                score += 3;
            }
            _ => {
                score += 0;
            }
        };

        score += get_position(&opponent, me) * 3
    }

    score
}

fn second_part(input: &str) -> usize {
    let mut score: usize = 0;
    for line in input.lines() {
        let opponent = line.chars().nth(0).unwrap();
        let me = match line.chars().nth(2).unwrap() {
            'X' => "0BCA",
            'Y' => {
                score += 3;
                "0ABC"
            }
            _ => {
                score += 6;
                "0CAB"
            }
        };

        score += get_position(&me, opponent);
    }
    score
}
