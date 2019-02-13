use super::card::Deck;
use super::game::Ranking;
use crate::impl::game;
use std::fmt;

pub struct Repl {
  events: Vec<GameEvent>,
  next_input: GameInput,
}

impl game::Repl for Repl {
  type Event = GameEvent;
  type Input = GameInput;

  fn events(&self) -> &[Event] {
    &self.events[..]
  }

  fn input(&self) -> Input {
    self.next_input
  }
}

impl Repl {
  pub fn new(events: Vec<GameEvent>, next_input: GameInput) -> Self {
    Self { events, next_input }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameInput {
  PlayCard, //play card
  SelectPile, //Select piles
}

impl fmt::Display for GameInput {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      GameInput::PlayCard => write!(f, "Play a Card"),
      GameInput::SelectPile => write!(f, "Select a Pile"),
    }
  }
}

#[derive(Debug)]
struct Event<'evt> {
  pub player: &'evt str,
}

#[derive(Debug)]
struct CardEvent<'cevt> {
  pub player: &'cevt str,
  pub cards: Deck,
}

#[derive(Debug)]
struct StatusEvent<'sevt> {
  pub player: &'sevt str,
  pub rank: Ranking,
}

#[derive(Debug)]
struct ExchangeEvent<'xevt> {
  pub giver: &'xevt str,
  pub receiver: &'xevt str,
}

#[derive(Debug)]
pub enum GameEvent<'evt> {
  Start,
  Invalid(Event<'evt>),
  Play(CardEvent<'evt>), //regular play
  Skip(Event<'evt>),
  Complete(CardEvent<'evt>), //that is, completion
  Bomb(Event<'evt>),
  Finish(StatusEvent<'evt>), //player's play ends them with no cards
  RoundFinish,               //all players have finished
  StartPick,
  Pick(CardEvent<'evt>), //player has picked a deck
  StartExchange,
  Offer(ExchangeEvent<'evt>), //lower-status -> higher-status
  Exchange(ExchangeEvent<'evt>),
}

impl<'evt> fmt::Display for GameEvent<'evt> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      GameEvent::Start => write!(f, "Game time started!"),
      GameEvent::Invalid(evt) => write!(f, "{}'s play is invalid!", evt.player),
      GameEvent::Play(card_evt) => {
        write!(f, "{} plays {}", card_evt.player, card_evt.cards)
      }
      GameEvent::Skip(evt) => write!(f, "{} Skipped!", evt.player),
      GameEvent::Complete(card_evt) => {
        write!(f, "{} completes with {}", card_evt.player, card_evt.cards)
      }
      GameEvent::Bomb(evt) => write!(f, "{} bombs", evt.player),
      GameEvent::Finish(stat_evt) => write!(
        f,
        "{} has cleared their hand as {}",
        stat_evt.player, stat_evt.rank
      ),
      GameEvent::RoundFinish => write!(f, "Round Finished!"),
      GameEvent::StartPick => write!(f, "Pick Stage Start!"),
      GameEvent::Pick(card_evt) => {
        write!(f, "{} has picked deck {}", card_evt.player, card_evt.cards)
      }
      GameEvent::StartExchange => write!(f, "Exchange stage Start!"),
      GameEvent::Offer(x_evt) => {
        write!(f, "{} has offered cards to {}", x_evt.giver, x_evt.receiver)
      }
      GameEvent::Exchange(x_evt) => write!(
        f,
        "{} has accepted the exchange with {}",
        x_evt.receiver, x_evt.giver
      ),
    }
  }
}

impl<'evt> game::GameEvent for GameEvent<'evt> {
  fn is_over(&self) -> bool {
    match self {
      GameEvent::RoundFinish => true,
      _ => false
    }
  }
}
