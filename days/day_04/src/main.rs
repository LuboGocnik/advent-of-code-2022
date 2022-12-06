fn main() {
    let input = include_str!("input.txt");
    let mut data = Vec::new();

    for line in input.lines() {
        let (raw1, raw2) = split_at_char(line, ',');
        let (f1, t1) = split_at_char(raw1, '-');
        let (f2, t2) = split_at_char(raw2, '-');

        data.push([f1, t1, f2, t2].map(|s| s.parse::<usize>().unwrap()))
    }

    let first: usize = data
        .iter()
        .map(|[f1, t1, f2, t2]| {
            if (f1 <= f2 && t1 >= t2) || (f1 >= f2 && t1 <= t2) {
                1
            } else {
                0
            }
        })
        .sum();

    let second: usize = data
        .iter()
        .map(|[f1, t1, f2, t2]| {
            if (f1 <= f2 && t1 >= f2) || (f2 <= f1 && t2 >= f1) {
                1
            } else {
                0
            }
        })
        .sum();

    println!("First: {}", first);
    println!("Second: {}", second);
}

fn split_at_char(s: &str, ch: char) -> (&str, &str) {
    let index = s.chars().position(|c| c == ch).unwrap();

    (&s[..index], &s[index + 1..])
}
