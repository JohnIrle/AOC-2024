use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./days/day_04/input_part_1.txt").expect("Could Not read file");
    runner::run(input, part_1, part_2);
}

fn part_1(input: String) -> i32 {
    let chars: Vec<Vec<char>> = input
        .lines() // Split the input into lines
        .map(|line| line.chars().collect()) // Convert each line into a Vec of chars
        .collect();

    let mut total = 0;

    let forward = ['X', 'M', 'A', 'S'];
    let backward = ['S', 'A', 'M', 'X'];

    let rows = chars.len();
    let cols = chars[0].len();

    // Horizontal
    for row in chars.iter() {
        for window in row.windows(forward.len()) {
            match window {
                w if w == forward => total += 1,
                w if w == backward => total += 1,
                _ => (),
            }
        }
    }

    // Vertical
    for col_idx in 0..cols {
        for row_idx in 0..=rows - forward.len() {
            let window: Vec<char> = chars[row_idx..row_idx + forward.len()]
                .iter()
                .map(|r| r[col_idx])
                .collect();

            match window {
                w if w == forward => total += 1,
                w if w == backward => total += 1,
                _ => (),
            }
        }
    }

    // First diagonal
    for row_idx in 0..=chars.len() - forward.len() {
        for col_idx in 0..=chars[0].len() - forward.len() {
            let window: Vec<char> = (0..forward.len())
                .map(|i| chars[row_idx + i][col_idx + i])
                .collect();

            match window {
                w if w == forward => total += 1,
                w if w == backward => total += 1,
                _ => (),
            }
        }
    }

    // Second Diagonal
    for row_idx in 0..=chars.len() - forward.len() {
        for col_idx in forward.len() - 1..chars[0].len() {
            let window: Vec<char> = (0..forward.len())
                .map(|i| chars[row_idx + i][col_idx - i])
                .collect();

            match window {
                w if w == forward => total += 1,
                w if w == backward => total += 1,
                _ => (),
            }
        }
    }

    total
}

fn part_2(input: String) -> i32 {
    let chars: Vec<Vec<char>> = input
        .lines() // Split the input into lines
        .map(|line| line.chars().collect()) // Convert each line into a Vec of chars
        .collect();

    let rows = chars.len();
    let cols = chars[0].len();

    let forward = ['M', 'A', 'S'];
    let backward = ['S', 'A', 'M'];

    let mut total = 0;

    for row_idx in 0..=rows - forward.len() {
        for col_idx in 0..=cols - forward.len() {
            let top_left_to_bottom = [
                chars[row_idx][col_idx],
                chars[row_idx + 1][col_idx + 1],
                chars[row_idx + 2][col_idx + 2],
            ];
            let bottom_left_to_top = [
                chars[row_idx + 2][col_idx],
                chars[row_idx + 1][col_idx + 1],
                chars[row_idx][col_idx + 2],
            ];

            if (top_left_to_bottom == forward || top_left_to_bottom == backward)
                && (bottom_left_to_top == forward || bottom_left_to_top == backward)
            {
                total += 1;
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part_1_returns_18_with_sample() {
        let input = String::from(INPUT);

        let result = part_1(input);

        assert_eq!(result, 18);
    }

    #[test]
    fn part_2_returns_9_with_sample() {
        let input = String::from(INPUT);

        let result = part_2(input);

        assert_eq!(result, 9);
    }
}
