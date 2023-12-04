struct Card {
    winning_numbers: Vec<u32>,
    owned_numbers: Vec<u32>,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let mut winning_numbers = vec![];
        let mut owned_numbers = vec![];
        let mut split = value.split(':');
        if let Some(_card) = split.next() {
            if let Some(numbers) = split.next() {
                let mut numbers_split = numbers.split('|');
                if let Some(winnings_text) = numbers_split.next() {
                    if let Some(owned_text) = numbers_split.next() {
                        let winning_numbers_split = winnings_text.split_whitespace();
                        for winning_number_text in winning_numbers_split {
                            if let Ok(number) = winning_number_text.parse::<u32>() {
                                winning_numbers.push(number);
                            }
                        }

                        let owned_numbers_split = owned_text.split_whitespace();
                        for owned_number_text in owned_numbers_split {
                            owned_numbers.push(owned_number_text.parse::<u32>().unwrap());
                        }
                    }
                }
            }
        }

        Card {
            winning_numbers,
            owned_numbers,
        }
    }
}

impl Card {
    fn get_points(&self) -> u32 {
        self.owned_numbers
            .iter()
            .filter(|owned_number| self.winning_numbers.contains(owned_number))
            .fold(1, |result, _| result * 2 ) / 2
    }

    fn get_matching_cards(&self) -> usize {
        self.owned_numbers
            .iter()
            .filter(|owned_number| self.winning_numbers.contains(owned_number))
            .count()
    }
}

struct ScratchCard {
    cards: Vec<Card>,
}

impl From<&str> for ScratchCard {
    fn from(value: &str) -> Self {
        let cards: Vec<Card> = value.lines().map(|value| value.into()).collect();

        ScratchCard {
            cards,
        }
    }
}

impl ScratchCard {
    fn get_total_points(&self) -> u32 {
        self.cards.iter().map(|card| card.get_points()).sum()
    }

    fn get_won_cards(&self) -> u32 {
        let cards_count = self.cards.len();
        let mut copies = vec![1; cards_count];

        for index in 0..cards_count {
            let card_copies = copies[index];
            let card = &self.cards[index];

            let points = card.get_matching_cards();

            for offset in 0..points {
                if let Some(copy) = copies.get_mut(index + offset + 1) {
                    *copy += card_copies;
                }
            }
        }

        copies.iter().sum()
    }
}

fn main() {
    let text = include_str!("puzzle_input.txt");

    let scratch_card: ScratchCard = text.into();

    println!("Puzzle 0: {}\nPuzzle 1: {}", scratch_card.get_total_points(), scratch_card.get_won_cards())
}

#[cfg(test)]
mod tests {
    use crate::ScratchCard;

    #[test]
    fn example_0() {
        let example_text =
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let scratch_card: ScratchCard = example_text.into();

        let total_points = scratch_card.get_total_points();

        assert_eq!(total_points, 13);
    }

    #[test]
    fn example_1() {
        let example_text =
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let scratch_card: ScratchCard = example_text.into();

        let won_cards = scratch_card.get_won_cards();

        assert_eq!(won_cards, 30);
    }

    #[test]
    fn puzzle_0() {
        let text = include_str!("puzzle_input.txt");

        let scratch_card: ScratchCard = text.into();

        let total_points = scratch_card.get_total_points();

        assert_eq!(total_points, 25651);
    }

    #[test]
    fn puzzle_1() {
        let text = include_str!("puzzle_input.txt");

        let scratch_card: ScratchCard = text.into();

        let total_points = scratch_card.get_won_cards();

        assert_eq!(total_points, 19499881);
    }
}