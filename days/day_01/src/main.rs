use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter::zip;

fn main() {
    let input = read_to_string("./days/day_01/input_part_1.txt").expect("Could Not read file");
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
    let (mut left, mut right) = parse_lines(&input);

    left.sort();
    right.sort();

    zip(left, right).map(|(a, b)| (a - b).abs()).sum()
}

fn part_2(input: String) -> i32 {
    let mut frequency_map = HashMap::new();
    let (left, right) = parse_lines(&input);

    for num in right {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    left.iter().fold(0, |acc, num| {
        acc + num * frequency_map.get(num).unwrap_or(&0)
    })
}

fn parse_lines(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (left, right): (Vec<_>, Vec<_>) = input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let left = parts.next()?;
            let right = parts.next()?;
            Some((left.parse::<i32>().ok()?, right.parse::<i32>().ok()?))
        })
        .unzip();
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_returns_11_with_example_input() {
        let input = String::from(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );

        let total = part_1(input);
        assert_eq!(total, 11);
    }

    #[test]
    fn part2_returns_31_with_example_input() {
        let input = String::from(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );

        let total = part_2(input);
        assert_eq!(total, 31);
    }
}
