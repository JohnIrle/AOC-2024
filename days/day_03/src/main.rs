use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./days/day_03/input_part_1.txt").expect("Could Not read file");
    runner::run(input, part_1, part_2);
}

fn part_1(input: String) -> i32 {
    sum_instructions(input)
}

fn part_2(input: String) -> i32 {
    let re = Regex::new(r"don't\(\)[\s\S]*?do\(\)").unwrap();
    let replaced = re.replace_all(&input, "").to_string();
    sum_instructions(replaced)
}

fn sum_instructions(input: String) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(&input)
        .filter_map(|cap| {
            let num1 = cap[1].parse::<i32>().ok();
            let num2 = cap[2].parse::<i32>().ok();
            if let (Some(num1), Some(num2)) = (num1, num2) {
                Some(num1 * num2)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_should_return_161() {
        let input =
            String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        let result = part_1(input);
        assert_eq!(161, result);
    }

    #[test]
    fn part_2_should_return_48() {
        let input = String::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        let result = part_2(input);
        assert_eq!(48, result);
    }
}
