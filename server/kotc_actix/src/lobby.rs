use kotc_game::game::Game;
use std::collections::HashSet;

pub struct Lobby {
    pub game: Game,
    pub sessions: HashSet<usize>,
}

impl Lobby {
    pub fn new() -> Self {
        Lobby {
            game: Game::new(),
            sessions: HashSet::new(),
        }
    }
}
