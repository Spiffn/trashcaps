use std::convert;

use state::GameState;
use deck::Hand;

#[derive(Debug)]
pub enum Input<'game> {
  //confirm (re)start game
  Begin, 
  PlayCard(usize, Hand),
  //player selection
  Selection(usize, u64), 
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
  Start, //waiting on game start/restart confirmation
  PlayCard, //player needs to play a card
  SelectPile, //player needs to select a pile from 0..maximum
}

impl InputType {
  pub fn validate(&self, input: &Input) -> bool {
    use InputType;
    //validation of enum contents is left to Game instead.
    match self {
      Start => match input {
        Input::Begin => true,
        _ => false,
      },
      PlayCard => match input {
        Input::PlayCard(_,_) => true,
        _ => false,
      },
      SelectPile(max) => match input {
        Input::Selection(_) => true,
        _ => false,
      }
    }
  }
}

impl convert::From<GameState> for InputType {
  fn from(state: GameState) -> Self {
    match state {
      GameState::Start | GameState::Restart => InputType::Start,
      GameState::Play | GameState::Exchange => InputType::PlayCard,
      GameState::Pick => InputType::SelectPile,
    }
  }
}
