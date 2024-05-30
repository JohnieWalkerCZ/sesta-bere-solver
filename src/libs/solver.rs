use crate::libs::{game::Game, helper::print_debug};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct LastCard {
    number: u8,
    ending: bool,
}

impl LastCard {
    pub fn new(number: u8, ending: bool) -> LastCard {
        LastCard {
            number,
            ending,
        }
    }
}


#[derive(Debug)]
pub struct GameSolver {}

impl GameSolver {
    pub fn new() -> GameSolver {
        GameSolver{}
    }

    pub fn suggest_move(&self, game: &Game) {
        let stacks = game.get_stacks();
        let mut end_cards = Vec::new();
        for stack in stacks {
            let last_el = stack.last().unwrap();
            let mut ending = false;
            if stack.len() == 5 {
                ending = true;
            }
            let end_card = LastCard::new(*last_el, ending);
            end_cards.push(end_card);
        }
        end_cards.sort();
        println!("{:?}", end_cards);
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
                let dist = game.get_unplayed_cards_range(end_num, *num).len() as u8;
                print_debug(format!("Distance between {end_num} and {num} is {dist}"));
                if dist < min_dist {
                    min_dist = dist
                }
            }

            if min_dist < best_dist {
                best_dist = min_dist;
                best_idx = i;
            }
        }

        println!("{best_idx}, {best_dist}");
    }
}
