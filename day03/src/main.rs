#[derive(Clone, PartialEq)]
enum Field {
    Number(u32, u32),
    Symbol,
    None,
}

struct Engine {
    fields: Vec<Vec<Field>>,
}

impl From<&str> for Engine {
    fn from(value: &str) -> Self {
        let mut fields: Vec<Vec<Field>> = vec![];
        let lines = value.lines();

        let mut uid = 0;

        for line in lines {
            let mut accumulator = 0;
            let mut multiplier = 1;
            let mut count = 0;

            let size = line.char_indices().count();
            let mut index = size;

            let chars = line.chars().rev();

            let mut field_row: Vec<Field> = vec![Field::Symbol; size];

            for char in chars {
                if char.is_numeric() {
                    accumulator += char.to_digit(10).unwrap() * multiplier;
                    multiplier *= 10;
                    count += 1;
                }
                else if count > 0 {
                    for _ in 0..count {
                        index -= 1;

                        field_row[index] = Field::Number(uid, accumulator);
                    }

                    uid +=1;

                    accumulator = 0;
                    multiplier = 1;
                    count = 0;

                    index -= 1;

                    if char == '.' {
                        field_row[index] = Field::None;
                    }
                    else {
                        field_row[index] = Field::Symbol;
                    }
                } else {
                    index -= 1;

                    if char == '.' {
                        field_row[index] = Field::None;
                    }
                    else {
                        field_row[index] = Field::Symbol;
                    }
                }
            }

            if count > 0 {
                for _ in 0..count {
                    index -= 1;
                    field_row[index] = Field::Number(uid, accumulator);
                    uid +=1;
                }
            }

            fields.push(field_row);
        }

        Engine {
            fields
        }
    }
}

impl Engine {
    fn get_sum(&self) -> u32 {
        let height = self.fields.len();
        let width = self.fields.get(0).unwrap().len();

        let mut last_uid = std::u32::MAX;
        let mut accumulator = 0;

        for x in 0..width {
            for y in 0..height {
                if let Some(field) = self.get_field(x, y) {
                    if *field == Field::Symbol{
                        if let Some((uid, value)) = self.get_field_value(x - 1, y - 1, last_uid) {
                            last_uid = uid;
                            accumulator += value;
                        }
                        if let Some((uid, value)) = self.get_field_value(x, y - 1, last_uid) {
                            last_uid = uid;
                            accumulator += value;
                        }
                        if let Some((uid, value)) = self.get_field_value(x + 1, y - 1, last_uid) {
                            last_uid = uid;
                            accumulator += value;
                        }
                        if let Some((uid, value)) = self.get_field_value(x - 1, y, last_uid) {
                            last_uid = uid;
                            accumulator += value;
                        }
                        if let Some((uid, value)) = self.get_field_value(x + 1, y, last_uid) {
                            last_uid = uid;
                            accumulator += value;
                        }
                        if let Some((uid, value)) = self.get_field_value(x - 1, y + 1, last_uid) {
                            last_uid = uid;
                            accumulator += value;
                        }
                        if let Some((uid, value)) = self.get_field_value(x, y + 1, last_uid) {
                            last_uid = uid;
                            accumulator += value;
                        }
                        if let Some((uid, value)) = self.get_field_value(x + 1, y + 1, last_uid) {
                            last_uid = uid;
                            accumulator += value;
                        }
                    }
                }
            }
        }

        accumulator
    }

    fn get_gear_ratios_sum(&self) -> u32 {
        let height = self.fields.len();
        let width = self.fields.get(0).unwrap().len();

        let mut last_uid = std::u32::MAX;
        let mut accumulator = 0;

        for x in 0..width {
            for y in 0..height {
                if let Some(field) = self.get_field(x, y) {
                    if *field == Field::Symbol{
                        let mut count = 0;
                        let mut gear_accumulator = 1;

                        if let Some((uid, value)) = self.get_field_value(x - 1, y - 1, last_uid) {
                            last_uid = uid;
                            count += 1;
                            gear_accumulator *= value;
                        }
                        if let Some((uid, value)) = self.get_field_value(x, y - 1, last_uid) {
                            last_uid = uid;
                            count += 1;
                            gear_accumulator *= value;
                        }
                        if let Some((uid, value)) = self.get_field_value(x + 1, y - 1, last_uid) {
                            last_uid = uid;
                            count += 1;
                            gear_accumulator *= value;
                        }
                        if let Some((uid, value)) = self.get_field_value(x - 1, y, last_uid) {
                            last_uid = uid;
                            count += 1;
                            gear_accumulator *= value;
                        }
                        if let Some((uid, value)) = self.get_field_value(x + 1, y, last_uid) {
                            last_uid = uid;
                            count += 1;
                            gear_accumulator *= value;
                        }
                        if let Some((uid, value)) = self.get_field_value(x - 1, y + 1, last_uid) {
                            last_uid = uid;
                            count += 1;
                            gear_accumulator *= value;
                        }
                        if let Some((uid, value)) = self.get_field_value(x, y + 1, last_uid) {
                            last_uid = uid;
                            count += 1;
                            gear_accumulator *= value;
                        }
                        if let Some((uid, value)) = self.get_field_value(x + 1, y + 1, last_uid) {
                            last_uid = uid;
                            count += 1;
                            gear_accumulator *= value;
                        }

                        if count == 2 {
                            accumulator += gear_accumulator;
                        }
                    }
                }
            }
        }

        accumulator
    }

    fn get_field(&self, x: usize, y: usize) -> Option<&Field> {
        if let Some(fields) = self.fields.get(y) {
            if let Some(field) = fields.get(x) {
                return Some(field)
            }
        }

        None
    }

    fn get_field_value(&self, x: usize, y: usize, last_uid: u32) -> Option<(u32, u32)> {
        if let Some(field) = self.get_field(x, y) {
            match field {
                Field::Number(uid, value) => {
                    if last_uid != *uid {
                        return Some((*uid, *value));
                    }
                },
                _ => return None,
            }
        }

        None
    }
}

fn main() {
    let text = include_str!("puzzle_input.txt");

    let engine: Engine = text.into();

    println!("Puzzle 0: {}\nPuzzle 1: {}", engine.get_sum(), engine.get_gear_ratios_sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_0() {
        let example_text =
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let engine: Engine = example_text.into();

        assert_eq!(engine.get_sum(), 4361);
    }

    #[test]
    fn example_1() {
        let example_text =
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let engine: Engine = example_text.into();

        assert_eq!(engine.get_gear_ratios_sum(), 467835);
    }

    #[test]
    fn puzzle_0() {
        let text = include_str!("puzzle_input.txt");

        let engine: Engine = text.into();

        assert_eq!(engine.get_sum(), 536202);
    }

    #[test]
    fn puzzle_1() {
        let text = include_str!("puzzle_input.txt");

        let engine: Engine = text.into();

        assert_eq!(engine.get_gear_ratios_sum(), 536202);
    }
}