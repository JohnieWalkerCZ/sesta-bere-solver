use crate::libs::game::Game;
use crate::libs::helper::print_error;
use crate::libs::helper::read_n_play_numbers;
use std::{
    collections::HashSet,
    io::{self, BufRead},
};

#[derive(Debug)]
pub struct GameManager {
    game: Game,
}

impl GameManager {
    pub fn new_game() -> GameManager {
        GameManager {
            game: Game::new_game(),
        }
    }

    pub fn load_start_info(&mut self) {
        let n_players = GameManager::read_n_players();
        let starting_stack_numbers = GameManager::read_starting_numbers();
        let hand_numbers = GameManager::read_hand_numbers(&starting_stack_numbers);
        self.game.add_to_stacks(starting_stack_numbers, true);
        self.game.set_hand(hand_numbers);
        self.game.set_n_players(n_players);
    }

    pub fn start_game_loop(&mut self) {
        loop {
            let played_cards: Vec<u8> = self.read_played_cards();
            self.game.add_to_stacks(played_cards, false);
            if self.game.get_hand().len() == 0 {
                return;
            }
        }
    }

    fn read_played_cards(&mut self) -> Vec<u8> {
        let mut played_cards: Vec<u8> = Vec::new();
        loop {
            println!("Enter the played cards: ");
            played_cards = read_n_play_numbers(self.game.get_n_players());
            if GameManager::n_common_els(&self.game.get_hand(), &played_cards) == 1
                && GameManager::n_common_els(&self.game.get_played_exclude_hand(), &played_cards)
                    == 0
            {
                break;
            } else {
                print_error("Invalid played cards".to_string());
            }
        }
        self.game.set_unplayable_cards(played_cards.clone());
        self.remove_card_after_round(played_cards.clone());

        return played_cards;
    }

    fn remove_card_after_round(&mut self, played_cards: Vec<u8>) {
        let s1: HashSet<_> = self.game.get_hand().iter().cloned().collect();
        let s2: HashSet<_> = played_cards.iter().cloned().collect();
        let intersection: Vec<u8> = s1.intersection(&s2).cloned().collect();
        let played_hand_card = intersection[0];
        self.game.remove_card_hand(played_hand_card);
    }

    fn n_common_els(vec1: &Vec<u8>, vec2: &Vec<u8>) -> u8 {
        let mut n_common = 0;
        for el in vec1 {
            if vec2.contains(el) {
                n_common += 1;
            }
        }

        n_common
    }

    fn read_n_players() -> u8 {
        println!("Enter number of players: ");
        let n_players = read_n_play_numbers(1)[0];
        return n_players;
    }

    fn read_hand_numbers(stack_numbers: &Vec<u8>) -> Vec<u8> {
        println!("Enter number of cards you drew: ");
        let n_cards = read_n_play_numbers(1)[0];
        loop {
            println!("Enter your card numbers: ");
            let hand_cards = read_n_play_numbers(n_cards);
            if GameManager::have_common_el(&stack_numbers, &hand_cards) {
                print_error("Can't have a card on stack and in hand".to_string());
                continue;
            }
            if hand_cards.len() == n_cards as usize {
                return hand_cards;
            }
        }
    }

    fn have_common_el(vec1: &Vec<u8>, vec2: &Vec<u8>) -> bool {
        for el in vec1 {
            if vec2.contains(&el) {
                return true;
            }
        }
        return false;
    }

    fn read_starting_numbers() -> Vec<u8> {
        loop {
            println!("Enter 4 starting stack numbers: ");
            let starting_stacks = read_n_play_numbers(4);
            if starting_stacks.len() == 4 {
                return starting_stacks;
            }
        }
    }

}
