use super::card::{Card, Suit};
use super::deck::{Deck, Hand};
use crate::collection::CardCollection;
use std::convert;
use std::fmt;

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
enum GameEvent<'evt> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameState {
  Start,   //rankings are undetermined, deal equally and start with 3 of clubs
  Restart, //rankings were determined, President first
  Turn(usize),
  Pick(usize),
  Offer(usize, usize), //lower rank offering (Scum, President)
  Exchange(usize, usize), //Higher rank conducting exchange (Scum, President)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Ranking {
  President,
  VicePresident,
  Citizen,
  ViceScum,
  Scum,
}

impl fmt::Display for Ranking {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Ranking::VicePresident => write!(f, "Vice President"),
      Ranking::ViceScum => write!(f, "Vice Scum"),
      _ => write!(f, "{:?}", self),
    }
  }
}

#[derive(Debug)]
pub enum GameStatOpt {
  Players,
  GameState,
}

#[derive(Debug)]
pub struct Game<'players, S: convert::AsRef<str>> {
  players: &'players [S],
  hands: Vec<Hand>,
  ranking: Vec<Ranking>,
  state: GameState,
  pile: Deck,
  events: Vec<GameEvent<'players>>,
}

impl<'players, S> Game<'players, S>
where
  S: convert::AsRef<str>,
{
  pub fn is_over(&self) -> bool {
    self.state == GameState::Restart
  }

  pub fn get_current_player(&self) -> Option<&'players str> {
    match self.state {
      GameState::Turn(idx)
      | GameState::Pick(idx)
      | GameState::Offer(idx, _)
      | GameState::Exchange(_, idx) => Some(self.players[idx].as_ref()),
      _ => None,
    }
  }

  pub fn start(&mut self) -> Result<Vec<String>, String> {
    match self.state {
      GameState::Start => {
        self.events.push(GameEvent::Start);
        let first_player_i = self.initial_deal();
        self.state = GameState::Turn(first_player_i);
      }
      GameState::Restart => {
        self.events.push(GameEvent::StartPick);
        let pres_i = self
          .ranking
          .iter()
          .position(|&rank| rank == Ranking::President)
          .expect("Anarchy! No Presidents!");
        self.state = GameState::Pick(pres_i);
      }
      _ => return Err(String::from("Can't call start() at this time")),
    }
    Ok(self.flush_events())
  }

  //get current game state as String
  pub fn stat(&self, opt: GameStatOpt) -> String {
    match opt {
      GameStatOpt::Players => self.stat_players(),
      GameStatOpt::GameState => self.stat_gamestate(),
    }
  }

  pub fn play(
    &mut self,
    player: &str,
    cards: &[&str],
  ) -> Result<Vec<String>, String> {
    unimplemented!();
  }

  pub fn init(players: &'players [S]) -> Result<Self, String>
  where
    S: convert::AsRef<str>,
  {
    let num_players = players.len();
    if num_players < 2 {
      return Err(String::from("There is no lone Capitalism."));
    }

    let mut ranking = Vec::with_capacity(num_players);
    let mut hands = Vec::with_capacity(num_players);

    (0..num_players).for_each(|_| {
      ranking.push(Ranking::Citizen);
      hands.push(Hand::new());
    });

    let suits = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];

    Ok(Self {
      players,
      hands,
      ranking,
      state: GameState::Start,
      pile: Deck::from_range(1..14, &suits),
      events: Vec::new(),
    })
  }

  fn stat_gamestate(&self) -> String {
    match self.state {
      GameState::Start => String::from("Game has yet to begin."),
      GameState::Restart => String::from("Game awaiting restart"),
      GameState::Turn(idx) => {
        format!("It's {}'s Turn.", self.players[idx].as_ref())
      }
      GameState::Pick(idx) => format!(
        "{} {} is Picking.",
        self.ranking[idx],
        self.players[idx].as_ref()
      ),
      GameState::Offer(giver_i, taker_i) => format!(
        "Waiting on {}'s offer between {} and {}.",
        self.ranking[giver_i],
        self.players[giver_i].as_ref(),
        self.players[taker_i].as_ref()
      ),
      GameState::Exchange(giver_i, taker_i) => format!(
        "Waiting on {}'s exchange between {} and {}.",
        self.ranking[taker_i],
        self.players[giver_i].as_ref(),
        self.players[taker_i].as_ref()
      ),
    }
  }

  fn stat_players(&self) -> String {
    self
      .players
      .iter()
      .zip(self.hands.iter())
      .zip(self.ranking.iter())
      .map(|((name, hand), rank)| {
        format!("  {} ({}): {}", name.as_ref(), rank, hand)
      })
      .fold(String::new(), |s, player| format!("{}\n{}", s, player))
  }

  fn flush_events(&mut self) -> Vec<String> {
    self
      .events
      .drain(..)
      .map(|evt| format!("{}", evt))
      .collect()
  }

  fn initial_deal(&mut self) -> usize {
    let num_players = self.players.len();
    let num_cards = self.pile.len();
    let chunks = num_cards / num_players;
    let leftovers = num_cards % num_players;

    //add deck to hands
    let hands_iter = self.hands.iter_mut();
    for h in hands_iter {
      h.add(self.pile.deal(chunks).expect("Ran out of chunks!"));
    }

    //add leftovers
    (0..leftovers).for_each(|i| {
      self.hands[i].put(self.pile.draw().expect("Ran out of leftovers!"));
    });

    //find first player
    let start_card = Card::new(3, Suit::Clubs);
    self
      .hands
      .iter()
      .position(|h| h.has(&start_card))
      .expect("No 3 of Clubs!")
  }
}
