use std::collections::HashSet;
use kotc_game::game::Game;

pub struct Lobby {
    pub game: Game,
    pub sessions: HashSet<usize>
}

impl Lobby {
    pub fn new() -> Self {
        Lobby {
            game: Game::new(),
            sessions: HashSet::new(),
        }
    }
}
