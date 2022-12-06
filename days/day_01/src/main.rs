fn main() {
    let input = include_str!("input.txt");

    let mut first: u32 = 0;
    let mut second: u32 = 0;
    let mut third: u32 = 0;

    let mut actual: u32 = 0;

    for line in input.lines() {
        match line.parse::<u32>() {
            Ok(calories) => {
                actual += calories;
            }
            Err(_) => {
                if actual >= first {
                    [first, second, third] = [actual, first, second];
                } else if actual >= second {
                    [second, third] = [actual, second];
                } else if actual >= third {
                    third = actual;
                }
                actual = 0;
            }
        }
    }
    println!("First: {}", first);
    println!("Second: {}", first + second + third);
}
