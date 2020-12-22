use shared::input::read_input_as_vec;
use shared::puzzle_trait::PuzzleTrait;

struct CardDeck {
    deck: std::collections::VecDeque<u32>
}

impl CardDeck {
    pub fn new() -> Self {
        Self { deck: std::collections::VecDeque::new() }
    }
}

pub struct Day22 {
    player_one: CardDeck,
    player_two: CardDeck
}

impl Day22 {
    pub fn new() -> Day22 {
        Day22 { player_one: CardDeck::new(), player_two: CardDeck::new() }
    }
}

impl PuzzleTrait for Day22 {
    fn print_info(&self) {
        println!("DAY 22 - CRAB COMBAT");
    }

    fn gather_input(&mut self) {
        let input = read_input_as_vec("./input/day_22.txt");

        let mut is_player_one = true;
        for line in &input {
            if line.len() == 0 {
                // Found the end of player one and the start of player two
                is_player_one = false;
                continue;
            }

            let parsed = line.parse::<u32>();
            if parsed.is_err() {
                // Either "Player 1:" or "Player 2:", irrelevant for the problem at hand
                continue;
            }

            // Construct the card decks
            let card = parsed.unwrap();
            if is_player_one {
                self.player_one.deck.push_back(card);
            } else {
                self.player_two.deck.push_back(card);
            }
        }
    }

    // Part one: what is the winning player's score?
    fn solve_part_one(&self) {
        let mut player_one_deck = self.player_one.deck.clone();
        let mut player_two_deck = self.player_two.deck.clone();

        loop {
            // Play until one of the players runs out of cards
            if player_one_deck.is_empty() || player_two_deck.is_empty() {
                break;
            }

            // Draw cards
            let card_p1 = player_one_deck.pop_front().unwrap();
            let card_p2 = player_two_deck.pop_front().unwrap();

            // The card with the highest value is always inserted before the card from the opponent
            // This ensures that the players always put their winning card in the deck first
            let player_one_wins = card_p1 > card_p2;
            if player_one_wins {
                // Move cards to the bottom of player one's deck
                player_one_deck.push_back(card_p1);
                player_one_deck.push_back(card_p2);
            } else {
                // Player two wins, move the cards to their deck instead
                player_two_deck.push_back(card_p2);
                player_two_deck.push_back(card_p1);
            }
        }

        let winning_deck: Vec<u32>;
        let player_one_wins = !player_one_deck.is_empty();

        // Determine the winner and save the winner's deck
        if player_one_wins {
            winning_deck = player_one_deck.into_iter().collect::<Vec<_>>();
        } else {
            winning_deck = player_two_deck.into_iter().collect::<Vec<_>>();
        }

        // Calculate the winning score
        let mut winning_score = 0;

        // Iterate in reverse to make the card at the bottom of the deck have the lowest multiplier
        for (index, score) in winning_deck.iter().rev().enumerate() {
            let multiplier = index as u32 + 1;
            winning_score += multiplier * score;
        }

        println!("Answer part one: {} is the winning player's score", winning_score);
    }

    // Part two: ___
    fn solve_part_two(&self) {
        println!("Answer part two: {}", -1);
    }
}
