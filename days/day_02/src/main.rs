use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./days/day_02/input_part_1.txt").expect("Could Not read file");
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        match args[1].as_str() {
            "part_1" => {
                let result = part_1(input);
                println!("part 1 solution: {}", result);
            }
            "part_2" => {
                let result = part_2(input);
                println!("part 2 solution: {}", result);
            }
            _ => {
                println!("Usage: <day> <part>");
                std::process::exit(64);
            }
        }
    } else {
        println!("Usage: part_<1 | 2>");
        std::process::exit(64);
    }
}

fn part_1(input: String) -> i32 {
    let lines = input.lines();
    let mut num_safe = 0;
    for line in lines {
        let level: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let mut safe = true;
        let mut is_up = true;
        let mut is_down = true;

        for window in level.windows(2) {
            if let [prev, current] = window {
                safe = safe && ((prev - current).abs() <= 3);
                match prev.cmp(current) {
                    std::cmp::Ordering::Less => is_down = false,
                    std::cmp::Ordering::Greater => is_up = false,
                    _ => {
                        is_up = false;
                        is_down = false;
                    }
                }
            }
        }

        if safe && (is_up || is_down) {
            num_safe += 1;
        }
    }
    num_safe
}
fn part_2(input: String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_returns_2_for_example_input() {
        let input = String::from(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );

        let result = part_1(input);

        assert_eq!(2, result);
    }
}
