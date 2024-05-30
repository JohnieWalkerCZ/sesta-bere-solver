use crate::libs::game::Game;
use crate::libs::game::Card;

struct GameSolver {}

impl GameSolver {
    pub fn new() -> GameSolver {
        GameSolver{}
    }

    pub fn solve(game: Game) -> Card {
        // TODO: Implement solving logic
        return Card::new(false, 222);
    }
}
