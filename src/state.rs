use std::iter;
/*
 * A game state is defined as any "break point" where the game
 * must stop to prompt the user.
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState { // <'names of players, 'player objects>
  Start,   //rankings are undetermined, deal equally and start with 3 of clubs
  Restart, //rankings were determined, President first
  Play,
  Pick, //(player, choices) player must pick 0..choices
  Exchange, //Higher rank conducting exchange (Scum, President)
}

impl GameState {
  pub fn new() -> Self {
    GameState::Start
  }

  pub fn next(self) -> Self {
    use GameState::*;
    match self {
      Start => Play,
      Restart => Play,
      Play => Pick,
      Pick => Exchange,
      Exchange => Restart,
    }
  }
}

impl iter::IntoIterator for GameState {
  type Item = GameState;
  type IntoIter = GameStateIterator;

  fn into_iter(self) -> Self::IntoIter {
    GameStateIterator(self)
  }
}

pub struct GameStateIterator(GameState);

impl iter::Iterator for GameStateIterator {
  type Item = GameState;

  fn next(&mut self) -> Option<Self::Item> {
    self.0 = self.0.next();
    Some(self.0)
  }
}
