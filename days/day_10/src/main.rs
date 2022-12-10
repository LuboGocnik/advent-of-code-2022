fn main() {
    let input = include_str!("input.txt");

    let mut cycle = 0isize;
    let mut x_register = 1isize;

    let mut signal_strength_sum = 0isize;

    print!("Second:");
    for line in input.lines() {
        if line == "noop" {
            cycle += 1;
            signal_strength_sum += check_signal(&cycle, &x_register);
        } else {
            cycle += 1;
            signal_strength_sum += check_signal(&cycle, &x_register);

            cycle += 1;
            signal_strength_sum += check_signal(&cycle, &x_register);
            x_register += line[5..].parse::<isize>().unwrap();
        }
    }

    println!("\n\nFirst: {}", signal_strength_sum);
}

const CYCLES: [isize; 6] = [20, 60, 100, 140, 180, 220];
fn check_signal(cycle: &isize, x_register: &isize) -> isize {
    let pixel_position = (cycle - 1) % 40;

    if pixel_position == 0 {
        println!();
    }

    if [x_register - 1, *x_register, x_register + 1].contains(&pixel_position) {
        print!("#");
    } else {
        print!(".");
    }

    if CYCLES.contains(cycle) {
        return cycle * x_register;
    }

    0
}
