use super::card::{Card, Suit};
use super::deck::{Deck, Hand};
use crate::collection::CardCollection;
use std::convert;
use std::fmt;

#[derive(Debug)]
struct Event {
  pub name: &str,
}

#[derive(Debug)]
struct CardEvent {
  pub name: &str,
  pub cards: &[Card],
}

#[derive(Debug)]
struct StatusEvent {
  pub name: &str,
  pub rank: Ranking,
}

#[derive(Debug)]
struct ExchangeEvent {
  pub offer: &str,
  pub receiver: &str,
}

#[derive(Debug)]
enum GameEvent {
  Start,
  Invalid(Event),
  Play(CardEvent), //regular play
  Complete(CardEvent), //that is, completion
  Bomb(Event),
  Finish(StatusEvent), //player's play ends them with no cards
  RoundFinish, //all players have finished
  StartPick,
  Pick(CardEvent), //player has picked a deck
  StartExchange(ExchangeEvent),
  Offer(Event), //lower-status -> higher-status
  Receive(Event), //higher status ACK lower-status offer
}

impl fmt::Display for GameEvent {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Invalid(player) => writeln!("{}'s play is invalid!", player),
      Play(CardEvent) => writeln!(""), //regular play
      Complete(CardEvent), //that is, completion
      Bomb(Event),
      Finish(StatusEvent), //player's play ends them with no cards
      RoundFinish, //all players have finished
      StartPick,
      Pick(CardEvent), //player has picked a deck
      StartOffer
      Offer(ExchangeEvent), //lower-status -> higher-status
      Exchange(ExchangeEvent), //higher status ACK lower-status offer
    }
  }
}

#[derive(Debug)]
enum GameState {
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
    write!(f, "{:?}", self)
  }
}

#[derive(Debug)]
pub enum GameStatOpt {
  Player(usize),
  GameState,
}

#[derive(Debug)]
pub struct Game<'players, S: convert::AsRef<str>> {
  players: &'players [S],
  hands: Vec<Hand>,
  ranking: Vec<Ranking>,
  state: GameState,
  pile: Deck,
}

impl<'players, S> fmt::Display for Game<'players, S>
where
  S: convert::AsRef<str> + fmt::Display,
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let player_states = self
      .players
      .iter()
      .zip(self.hands.iter())
      .zip(self.ranking.iter());
    for ((name, hand), rank) in player_states {
      writeln!(f, "  {} ({}): {}", name, rank, hand)?;
    }

    match self.state {
      GameState::Turn(idx) => {
        writeln!(f, "It's {}'s Turn.", self.players[idx])?;
      }
      GameState::Pick(idx) => {
        writeln!(f, "{} {} is Picking.", self.ranking[idx], self.players[idx])?;
      }
      GameState::Offer(giver_i, taker_i) => {
        writeln!(
          f,
          "Waiting on {}'s offer between {} and {}.",
          self.ranking[giver_i],
          self.players[giver_i],
          self.players[taker_i]
        )?;
      }
      GameState::Exchange(giver_i, taker_i) => {
        writeln!(
          f,
          "Waiting on {}'s exchange between {} and {}.",
          self.ranking[taker_i],
          self.players[giver_i],
          self.players[taker_i]
        )?;
      }
    }
    Ok(())
  }
}

impl<'players, S> Game<'players, S>
where
  S: convert::AsRef<str>,
{ 
  pub fn restart(&mut self) {
  }

  //get current game state as String
  pub fn stat(&self) -> String {
  }

  pub fn play(&mut self, player: &str, cards: &[&str]) -> Vec<String> {
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
    //initialize
    (0..num_players).for_each(|_| {
      ranking.push(Ranking::Citizen);
      hands.push(Hand::new());
    });
    let mut deck = Deck::from_range(
      1..14,
      &[Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs],
    );

    let num_cards = deck.len();
    let chunks = num_cards / num_players;
    let leftovers = num_cards % num_players;

    //add deck to hands
    hands.iter_mut().for_each(|h| {
      h.add(deck.deal(chunks).expect("Ran out of chunks!"));
    });

    //add leftovers
    (0..leftovers).for_each(|i| {
      hands[i].put(deck.draw().expect("Ran out of leftovers!"));
    });

    //find first player
    let start_card = Card::new(3, Suit::Clubs);
    let starting = hands.iter().position(|h| h.has(&start_card));

    Ok(Self {
      players,
      hands,
      ranking,
      state: GameState::Turn(starting.expect("3 of Clubs wasn't generated!")),
      pile: Deck::empty(),
    })
  }

  fn current_player(&self) -> String {
    let idx = match self.state {
      GameState::Turn(idx) => idx,
      GameState::Pick(idx) => idx,
      GameState::Offer(giver_i, _) => giver_i,
      GameState::Exchange(_, taker_i) => taker_i,
    };
    String::from(self.players[idx].as_ref())
  }
}
