use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./days/day_02/input_part_1.txt").expect("Could Not read file");
    runner::run(input, part_1, part_2);
}

fn part_1(input: String) -> i32 {
    input
        .lines()
        .filter(|line| is_line_safe(parse_level(line)))
        .count() as i32
}

fn part_2(input: String) -> i32 {
    input
        .lines()
        .filter(|line| {
            parse_level(line).iter().enumerate().any(|(i, _)| {
                let sub_line: Vec<_> = parse_level(line)
                    .iter()
                    .enumerate()
                    .filter_map(|(j, x)| if i != j { Some(*x) } else { None })
                    .collect();

                is_line_safe(sub_line)
            })
        })
        .count() as i32
}

fn parse_level(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

fn is_line_safe(level: Vec<i32>) -> bool {
    let mut safe = true;
    let mut is_up = true;
    let mut is_down = true;

    for window in level.windows(2) {
        if let [prev, current] = window {
            safe = safe && ((prev - current).abs() <= 3);
            match prev.cmp(current) {
                std::cmp::Ordering::Less => is_down = false,
                std::cmp::Ordering::Greater => is_up = false,
                std::cmp::Ordering::Equal => {
                    is_up = false;
                    is_down = false;
                }
            }
        }
    }

    safe && (is_up || is_down)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part_1_returns_2_for_example_input() {
        let input = String::from(INPUT);

        let result = part_1(input);

        assert_eq!(2, result);
    }

    #[test]
    fn part_2_returns_4_for_example_input() {
        let input = String::from(INPUT);

        let result = part_2(input);

        assert_eq!(4, result);
    }
}
