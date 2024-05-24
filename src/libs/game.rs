use crate::libs::helper::{read_n_numbers_range, read_n_play_numbers};


#[derive(Debug, Clone, Copy)]
pub struct Card {
    played: bool,
    number: u8,
}

impl Card {
    pub fn new(played: bool, number: u8) -> Card {
        Card {
            played,
            number,
        }
    }
}

#[derive(Debug)]
pub struct Game {
    cards: Vec<Card>,
    stacks: Vec<Vec<u8>>,
    hand: Vec<u8>,
    n_players: u8,
}

impl Game {
    pub fn new_game() -> Game {
        Game {
            cards: self::Game::init_playable_cards(),
            stacks: vec![vec![1], vec![1], vec![1], vec![1]],
            hand: vec![1],
            n_players: 1,
        }
    }

    fn init_playable_cards() -> Vec<Card> {
        let mut cards = vec![];
        for i in 1..101 {
            let card = Card::new(false, i);
            cards.push(card);
        }

        return cards;
    }
    
    pub fn set_unplayable_cards(&mut self, cards: Vec<u8>) {
        let cards_set: std::collections::HashSet<u8> = cards.into_iter().collect();

        for card in &mut self.cards {
            if cards_set.contains(&card.number) {
                card.played = true;
            }
        }
    }
    
    pub fn set_n_players(&mut self, n_players: u8) {
        self.n_players = n_players;
    }

    pub fn get_n_players(&self) -> u8 {
        return self.n_players;
    }

    pub fn set_hand(&mut self, hand: Vec<u8>) {
        self.set_unplayable_cards(hand.clone());
        self.hand = hand;
    }

    pub fn get_hand(&self) -> Vec<u8> {
        self.hand.clone()
    }

    pub fn get_played_exclude_hand(&self) -> Vec<u8> {
        let mut result = Vec::new();
        for card in self.cards.clone() {
            if card.played && !self.hand.contains(&card.number){
                result.push(card.number);
            }
        }

        result
    }

    pub fn remove_card_hand(&mut self, played_card: u8) {
        self.hand.retain(|card| *card != played_card);
    }

    pub fn add_to_stacks(&mut self, numbers: Vec<u8>, first: bool) {
        self.set_unplayable_cards(numbers.clone());
        if first {
            // Clear all stacks before adding new numbers
            self.stacks.iter_mut().for_each(|stack| stack.clear());

            // Iterate over stacks and numbers simultaneously
            self.stacks
                .iter_mut()
                .zip(numbers.into_iter())
                .for_each(|(stack, num)| {
                    stack.push(num);
                });
        } else {
            self.apped_to_stacks(numbers);
        }
    }

    fn apped_to_stacks(&mut self, mut numbers: Vec<u8>) {
        numbers.sort();
        println!("Pre: {:?}", self.stacks);

        for num in numbers {
            let mut min_idx = 0;
            let mut diff = i8::MAX;

            for (i, arr) in self.stacks.iter().enumerate() {
                let last_el = arr.last().unwrap_or(&0);
                if last_el > &num {
                    break;
                }

                let new_diff: i8 = num as i8 - *last_el as i8;

                if new_diff < diff {
                    diff = new_diff;
                    min_idx = i;
                }
            }

            if diff == i8::MAX {
                self.clean_stack(num);
            }

            self.stacks[min_idx].push(num);
        }
        println!("After: {:?}", self.stacks);
    }

    fn clean_stack(&mut self, end_num: u8) {
        println!("Which stack to clean?");
        let choice = read_n_numbers_range(1, 1, 4)[0];
        let idx = (choice - 1) as usize;
        self.stacks[idx] = vec![end_num];
    }
}
