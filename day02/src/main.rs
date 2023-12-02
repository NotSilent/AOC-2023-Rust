#[derive(Clone)]
struct Bag {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

impl Bag {
    fn new(red: i32, green: i32, blue: i32) -> Self {
        Bag {
            red,
            green,
            blue,
        }
    }

    fn get_power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

struct GameRound {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

impl GameRound {
    fn new(red: i32, green: i32, blue: i32) -> Self {
        GameRound {
            red,
            green,
            blue,
        }
    }
}

struct Game {
    rounds: Vec<GameRound>,
}

impl Game {
    fn new(rounds: Vec<GameRound>) -> Self {
        Game {
            rounds,
        }
    }

    fn get_smallest_required_bag(&self) -> Bag {
        let mut bag = Bag::new(0, 0, 0);

        for round in &self.rounds {
            bag.red = std::cmp::max::<i32>(bag.red, round.red);
            bag.blue = std::cmp::max::<i32>(bag.blue, round.blue);
            bag.green = std::cmp::max::<i32>(bag.green, round.green);
        }

        bag
    }

    fn check_if_possible(&self, bag: &Bag) -> bool {
        for round in &self.rounds
        {
            if bag.red - round.red < 0 {
                return false;
            }
            if bag.green - round.green < 0 {
                return false;
            }
            if bag.blue - round.blue < 0 {
                return false;
            }
        }

        true
    }
}

fn parse_input(input: &str) -> Vec<Game> {
    input.lines().map(|line| {
        let mut game_rounds= vec![];
        let mut split = line.split(':');
        if let Some(_game) = split.next() {
            if let Some(rounds) = split.next() {
                for round in rounds.split(';') {
                    let mut game_round = GameRound::new(0, 0, 0);
                    for inner_elem in round.split(',') {
                        let mut elem = inner_elem.trim().split(' ');
                        if let Some(count) = elem.next() {
                            if let Some(colour) = elem.next() {
                                if let Ok(count) = count.parse::<i32>() {
                                    match colour {
                                        "red" => game_round.red = count,
                                        "green" => game_round.green = count,
                                        "blue" => game_round.blue = count,
                                        _ => panic!("Unsupported colour!"),
                                    }
                                }
                            }
                        }
                    }
                    game_rounds.push(game_round);
                }
            }
        }

        Game::new(game_rounds)
    }).collect()
}

fn sum_of_ids(bag: &Bag, games: &[Game]) -> i32 {
    games.iter().enumerate()
        .filter(|(_, game)| game.check_if_possible(bag))
        .fold(0, |result, (element, _)| result + element + 1) as i32
}

fn sum_of_powers(games: &[Game]) -> i32 {
    games.iter()
        .map(|game| game.get_smallest_required_bag())
        .fold(0, |sum, bag| sum + bag.get_power())
}

fn main() {
    let input = include_str!("puzzle_input.txt");
    let games = parse_input(input);

    let ids = sum_of_ids(&Bag::new(12, 13, 14), &games);
    let powers = sum_of_powers(&games);

    println!("Puzzle 0: {ids}\nPuzzle 1: {powers}");
}


#[cfg(test)]
mod tests {
    use super::*;

    fn create_example_0() -> Vec<Game> {
        let game_1 = Game::new(vec!(
            GameRound::new(4, 0, 3),
            GameRound::new(1, 2, 6),
            GameRound::new(0, 2, 0),
        ));

        let game_2 = Game::new(vec!(
            GameRound::new(0, 2, 1),
            GameRound::new(1, 3, 4),
            GameRound::new(0, 1, 1),
        ));

        let game_3 = Game::new(vec!(
            GameRound::new(20, 8, 6),
            GameRound::new(4, 13, 5),
            GameRound::new(1, 5, 0),
        ));

        let game_4 = Game::new(vec!(
            GameRound::new(3, 1, 6),
            GameRound::new(6, 3, 0),
            GameRound::new(14, 3, 15),
        ));

        let game_5 = Game::new(vec!(
            GameRound::new(6, 3, 1),
            GameRound::new(1, 2, 2),
        ));

        vec!(game_1, game_2, game_3, game_4, game_5)
    }

    #[test]
    fn example_0() {
        let bag = Bag::new(12, 13, 14);
        let example_0 = create_example_0();

        if example_0.len() != 5 {
            assert!(false, "Example 0 count != 5");
        }

        assert!(example_0[0].check_if_possible(&bag));
        assert!(example_0[1].check_if_possible(&bag));
        assert!(!example_0[2].check_if_possible(&bag));
        assert!(!example_0[3].check_if_possible(&bag));
        assert!(example_0[4].check_if_possible(&bag));
    }

    #[test]
    fn example_0_result() {
        let bag = Bag::new(12, 13, 14);
        let example_0 = create_example_0();

        let value = sum_of_ids(&bag, &example_0);

        assert_eq!(value, 8);
    }

    #[test]
    fn puzzle_0() {
        let input = include_str!("puzzle_input.txt");
        let games = parse_input(input);

        let value = sum_of_ids(&Bag::new(12, 13, 14), &games);

        assert_eq!(value, 2512);
    }
    #[test]
    fn puzzle_1() {
        let input = include_str!("puzzle_input.txt");
        let games = parse_input(input);

        let value = sum_of_powers(&games);

        assert_eq!(value, 67335);
    }
}