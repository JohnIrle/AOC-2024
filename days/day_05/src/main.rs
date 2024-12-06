use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./days/day_05/input_part_1.txt").expect("Could Not read file");
    runner::run(input, part_1, part_2);
}

fn part_1(input: String) -> i32 {
    let (top, bottom) = get_sections(input);

    let rules_map = build_rules_map(top);

    let parsed_updates = parse_updates(bottom);

    let (passing_updates, _) = get_passing_and_non_passing_updates(&rules_map, parsed_updates);

    passing_updates
        .iter()
        .filter_map(|i| i.get(i.len() / 2).copied())
        .sum()
}

fn part_2(input: String) -> i32 {
    let (top, bottom) = get_sections(input);

    let rules_map = build_rules_map(top);

    let parsed_updates = parse_updates(bottom);

    let (_, mut non_passing_updates) =
        get_passing_and_non_passing_updates(&rules_map, parsed_updates);

    for update in &mut non_passing_updates {
        while !does_update_pass(&rules_map, update) {
            for i in 0..update.len() - 1 {
                if let Some(val) = rules_map.get(&update[i + 1]) {
                    if val.contains(&update[i]) {
                        // swap because next item has current in its edges
                        update.swap(i, i + 1);
                    }
                }
            }
        }
    }

    non_passing_updates
        .iter()
        .filter_map(|i| i.get(i.len() / 2).copied())
        .sum()
}

fn parse_updates(section: String) -> Vec<Vec<i32>> {
    let parsed_updates: Vec<Vec<i32>> = section
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .collect()
        })
        .collect();

    parsed_updates
}

fn get_passing_and_non_passing_updates(
    rules_map: &HashMap<i32, Vec<i32>>,
    parsed_updates: Vec<Vec<i32>>,
) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut passing_updates: Vec<Vec<i32>> = vec![];
    let mut non_passing_updates: Vec<Vec<i32>> = vec![];

    for update in parsed_updates {
        let is_successful = does_update_pass(rules_map, &update);
        if is_successful {
            passing_updates.push(update);
        } else {
            non_passing_updates.push(update)
        }
    }
    (passing_updates, non_passing_updates)
}

fn does_update_pass(rules_map: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> bool {
    let mut is_successful = true;
    for i in 0..update.len() - 1 {
        let current_number = &update[i];
        let rest = &update[i + 1..];

        if let Some(after) = rules_map.get(current_number) {
            let all_present = rest.iter().all(|&num| after.contains(&num));

            if !all_present {
                is_successful = false;
                break;
            }
        } else {
            is_successful = false;
            break;
        }
    }
    is_successful
}

fn build_rules_map(top: String) -> HashMap<i32, Vec<i32>> {
    let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();

    for line in top.lines() {
        if let Some((key, value)) = line.split_once('|') {
            if let (Ok(key), Ok(value)) = (key.parse::<i32>(), value.parse::<i32>()) {
                let entry = rules_map.entry(key).or_default();
                entry.push(value);
            }
        }
    }

    rules_map
}

fn get_sections(input: String) -> (String, String) {
    let sections: Vec<&str> = input.split("\n\n").collect();

    (
        String::from(*sections.first().unwrap_or(&"")),
        String::from(*sections.get(1).unwrap_or(&"")),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part_1_returns_143_with_sample() {
        let input = String::from(INPUT);

        let result = part_1(input);

        assert_eq!(result, 143);
    }

    #[test]
    fn part_2_returns_123_with_sample() {
        let input = String::from(INPUT);

        let result = part_2(input);

        assert_eq!(result, 123);
    }
}
