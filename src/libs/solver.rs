use crate::libs::{game::Game, helper::print_info};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct LastCard {
    number: u8,
    ending: bool,
    interval_start: u8,
    interval_end: u8,
}

impl LastCard {
    pub fn new(number: u8, ending: bool, interval_start: u8, interval_end: u8) -> LastCard {
        LastCard {
            number,
            ending,
            interval_start,
            interval_end,
        }
    }
}

#[derive(Debug)]
pub struct GameSolver {}

impl GameSolver {
    pub fn new() -> GameSolver {
        GameSolver {}
    }

    pub fn suggest_move(&self, game: &Game) {
        let end_cards: Vec<LastCard> = self.load_end_cards(game);
        let end_cards = self.generate_intervals(end_cards);

        let mut hand = game.get_hand();
        hand.sort();
        let mut best_idx = 0;
        let mut best_dist = u8::MAX;
        for (i, num) in hand.iter().enumerate() {
            let mut min_dist = u8::MAX;
            for end_card in end_cards.clone() {
                let end_num = end_card.number;
                if end_num > *num {
                    break;
                }
                let mut dist = 0;
                if end_card.ending {
                    if end_card.interval_end < *num {
                        continue;
                    }
                    dist = game.get_unplayed_cards_range(*num, end_card.interval_end).len() as u8;
                } else {

                    dist = game.get_unplayed_cards_range(end_num, *num).len() as u8;
                }
                if dist < min_dist {
                    min_dist = dist
                }
            }

            if min_dist < best_dist {
                best_dist = min_dist;
                best_idx = i;
            }
        }
        
        let best_play = hand[best_idx];
        print_info(format!("You shoud play {best_play} with the separation {best_dist}"));
    }

    fn load_end_cards(&self, game: &Game) -> Vec<LastCard> {
        let stacks = game.get_stacks();
        let mut end_cards = Vec::new();
        for stack in stacks {
            let last_el = stack.last().unwrap();
            let mut ending = false;
            if stack.len() == 5 {
                ending = true;
            }
            let end_card = LastCard::new(*last_el, ending, 0, 0);
            end_cards.push(end_card);
        }
        end_cards.sort_by_key(|x| x.number);
        
        end_cards
    }

    fn generate_intervals(&self, end_cards: Vec<LastCard>) -> Vec<LastCard> {
        let mut last_cards = Vec::new();
        for (i, card) in end_cards.iter().enumerate() {
            let interval_start = card.number;
            let mut interval_end = 100;
            if i+1 != end_cards.len() {
                let next_card = end_cards[i+1].clone();
                interval_end = next_card.number;
            }
            let new_card = LastCard::new(card.number, card.ending, interval_start, interval_end);
            last_cards.push(new_card);
        }

        last_cards
    }

}
