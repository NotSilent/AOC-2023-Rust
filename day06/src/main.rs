#[derive(Default)]
struct Race {
    time: u64,
    distance: u64,
}

impl From<&str> for Race {
    fn from(value: &str) -> Self {
        fn extract_number(value: &str) -> u64 {
            let mut split = value.split(':');

            if let Some(_text) = split.next() {
                if let Some(numbers) = split.next() {
                    let number = numbers
                        .split_whitespace()
                        .collect::<String>();

                    return number
                        .parse::<u64>()
                        .unwrap();
                }
            }

            0
        }

        let mut lines = value.lines();

        if let Some(times_text) = lines.next() {
            if let Some(distances_text) = lines.next() {
                let time = extract_number(times_text);
                let distance = extract_number(distances_text);

                return Race {
                    time,
                    distance,
                };
            }
        }

        Race::default()
    }
}

impl Race {
    fn get_margin_of_error(&self) -> u64 {
        (0..self.time)
            .filter(|time_charging|{
                let time_traveling = self.time - time_charging;

                let distance_traveled = time_charging * time_traveling;

                distance_traveled > self.distance
            })
            .count() as u64
    }
}

struct Document {
    races: Vec<Race>,
}

impl From<&str> for Document {
    fn from(value: &str) -> Self {
        fn extract_numbers(value: &str) -> Vec<u64> {
            let mut split = value.split(':');

            if let Some(_text) = split.next() {
                if let Some(numbers) = split.next() {
                    return numbers
                        .split_whitespace()
                        .map(|number| number.parse::<u64>().unwrap())
                        .collect();
                }
            }

            vec![]
        }

        let mut lines = value.lines();

        if let Some(times_text) = lines.next() {
            if let Some(distances_text) = lines.next() {
                let times = extract_numbers(times_text);
                let distances = extract_numbers(distances_text);

                let races = times.iter().zip(distances.iter())
                    .map(|(time, distance)| Race {
                        time: *time,
                        distance: *distance,
                    }).collect();

                return Document {
                    races,
                };
            }
        }

        Document {
            races: vec![],
        }
    }
}

impl Document {

    fn get_total_margin_of_error(&self) -> u64 {
        self.races.iter().map(|race| race.get_margin_of_error()).product()
    }
}

fn main() {
    let text = include_str!("puzzle_input.txt");

    let document: Document = text.into();
    let race: Race = text.into();

    println!("Puzzle 0: {}\nPuzzle 1: {}", document.get_total_margin_of_error(), race.get_margin_of_error(), )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let text = "Time:      7  15   30
Distance:  9  40  200";

        let document: Document = text.into();

        assert_eq!(document.get_total_margin_of_error(), 288);
    }

    #[test]
    fn puzzle_0() {
        let text = include_str!("puzzle_input.txt");

        let document: Document = text.into();

        assert_eq!(document.get_total_margin_of_error(), 6209190);
    }

    #[test]
    fn puzzle_1() {
        let text = include_str!("puzzle_input.txt");

        let race: Race = text.into();

        assert_eq!(race.get_margin_of_error(), 28545089);
    }
}