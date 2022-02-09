use kotc_game::game::Game;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Clone)]
pub struct Lobby {
    pub game: Rc<RefCell<Game>>,
    pub sessions: HashSet<usize>,
}

impl Lobby {
    pub fn new() -> Self {
        Lobby {
            game: Rc::new(RefCell::new(Game::new())),
            sessions: HashSet::new(),
        }
    }
}
