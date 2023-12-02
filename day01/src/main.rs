fn main() {
    let file = include_str!("puzzle_input.txt");
    let result = get_text_calibration_value(file);

    println!("{result}");
}

fn get_text_calibration_value(text: &str) -> u32 {
    let lines = text.lines();

    lines
        .map(find_number)
        .sum()
}

fn find_number(line: &str) -> u32 {
    let patterns = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let digit = line.chars().enumerate().find(|c| c.1.is_numeric());

    let initial_result: (usize, u32) = if let Some(digit_result) = digit {
        (digit_result.0, digit_result.1.to_string().parse::<u32>().unwrap())
    } else {
        (std::usize::MAX, 0)
    };

    let first_digit: (usize, u32) = patterns.iter()
        .fold(initial_result, |result, pattern| {
            if let Some(found_pattern_index) = line.find(pattern.0) {
                if found_pattern_index < result.0 {
                    return (found_pattern_index, pattern.1)
                }
            }

            result
        });

    let last_real_digit = line.chars().rev().enumerate().find(|c| c.1.is_numeric());

    let initial_last_result: (usize, u32) = if let Some(digit_result) = last_real_digit {
        (line.len() - digit_result.0, digit_result.1.to_string().parse::<u32>().unwrap())
    } else {
        (0, 0)
    };

    let last_digit: (usize, u32) = patterns.iter()
        .fold(initial_last_result, |result, pattern| {
            if let Some(found_pattern_index) = line.rfind(pattern.0) {
                if found_pattern_index + 1 > result.0 {
                    return (found_pattern_index, pattern.1);
                }
            }

            result
        });

    first_digit.1 * 10 + last_digit.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_0() {
        let file = include_str!("example_0.txt");
        let result = get_text_calibration_value(file);

        assert_eq!(result, 142);
    }

    #[test]
    fn test_example_1() {
        let file = include_str!("example_1.txt");
        //let file2 = convert_text_calibration_value(file);
        let result = get_text_calibration_value(&file);

        assert_eq!(result, 281);
    }
}