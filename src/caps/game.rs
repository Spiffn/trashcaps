use super::card::{Card, Suit};
use super::deck::{Deck, Hand};
use super::repl::{Repl, self};
use crate::impl::collection::CardCollection;
use crate::impl::game;
use std::convert;
use std::fmt;
use std::opt;

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
pub enum GameStatRes {
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

impl<'players, S> Iterator for Game<'players, S> {
  fn next(&mut self) -> Option<Self::Item> {
    unimplemented!();
  }
}

impl<'players, S> game::Game for Game<'players, S> {
  type Turn: Repl;
  type StatOpt: GameStatOpt;
  type StatOut: 
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
    player_str: &str,
    hand_str: &[&str],
  ) -> Result<Vec<String>, String> {
    //validate player
    //- exists
    //validate cards
    //- are valid cards
    //- are of the player's hand
    //validate play
    //- turn order

    //execute play->events
    //otherwise->errorstr
    let player_i = self
      .players
      .iter()
      .position(|p| p.as_ref() == player_str)
      .ok_or(format!("Invalid Player {}!", player_str))?;
    let played: Deck = self.validate_cards()?;
    if played.is_empty() {
      return Err(String::from("No matchable cards!"));
    }
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

  fn validate_cards(&self, player_i: usize, hand_str: &[&str])
  -> Result<Deck, String> {
    hand_str
      .iter()
      .map(|card_str| {
        let err_msg = format!("Invalid Card {}!", card_str);
        if card_str.ends_with("*") {
          //wildcard
          let rank: i64 = card_str
            .get(..card_str.len()-1)
            .ok_or(err_msg)?
            .parse()
            .map_err(err_msg)?;
          let suits = [
            Suit::Spades,
            Suit::Hearts,
            Suit::Diamonds,
            Suit::Clubs];
          Ok(suits
            .iter()
            .filter_map(|s| {
              let c = Card::new(rank, s);
              if self.hands.has(&c) {
                Some(c)
              } else {
                None
              }
            })
            .collect()::<Deck>())
        } else {
          let card = Card::tryfrom(card_str).ok_or(err_msg)?;
          if self.hands[player_i].has(&card) {
            let mut res = Deck::new();
            res.put(card);
            Ok(res)
          } else {
            Err(format!("{} doesn't have a {}",
              self.players[player_i].as_ref(), card_str))
          }
        }
      })
      .collect()
  }

  fn exec(&mut self, player_i: usize, cards: Deck) {
  }
}
