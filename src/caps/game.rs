use super::card::{Card, Suit};
use super::deck::{Deck, Hand};
use crate::collection::CardCollection;
use std::convert;
use std::fmt;

#[derive(Debug)]
struct Event {
  pub player: &str,
}

#[derive(Debug)]
struct CardEvent {
  pub player: &str,
  pub cards: Deck,
}

#[derive(Debug)]
struct StatusEvent {
  pub player: &str,
  pub rank: Ranking,
}

#[derive(Debug)]
struct ExchangeEvent {
  pub giver: &str,
  pub receiver: &str,
}

#[derive(Debug)]
enum GameEvent {
  Start,
  Invalid(Event),
  Play(CardEvent), //regular play
  Skip(Event),
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
      Start => write!(f, "Game time started!"),
      Invalid(evt) => write!(f, "{}'s play is invalid!", evt.player),
      Play(card_evt) => {
        write!(f, "{} plays {}", card_evt.player, card_evt.cards)
      },
      Complete(card_evt) => {
        write!(f, "{} completes with {}", card_evt.player, card_evt.cards)
      },
      Bomb(evt) => {
        write!(f, "{} bombs", evt.player)
      },
      Finish(stat_evt) {
        write!(f, "{} has cleared their hand as {}",
          stat_evt.player,
          stat_evt.rank)
      },
      RoundFinish => write!(f, "Round Finished!"), //all players have finished
      StartPick => write!(f, "Pick Stage Start!"),
      Pick(card_evt) => {
        write!(f, "{} has picked deck {}", card_evt.player, card_evt.cards)
      },
      StartOffer => write!(f, "Offer Stage Start!")
      Offer(x_evt) {
        write!(f, "{} has offered cards to {}",
          card_evt.giver, 
          card_evt.receiver)
      },
      Exchange(x_evt) {
        write!(f, "{} has accepted the exchange with {}",
          card_evt.receiver,
          card_evt.giver)
      },
    }
  }
}

#[derive(Debug)]
enum GameState {
  Start, //rankings are undetermined, deal equally and start with 3 of clubs
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
  events: Vec<GameEvent>,
}

impl<'players, S> Game<'players, S>
where
  S: convert::AsRef<str>,
{ 
  pub fn start(&mut self) -> Result<Vec<String>, String> {
    match self.state {
      GameState::InitialStart => {
        self.events.push(GameEvent::Start);
        let first_player_i = self.initial_deal();
        self.state = GameState::Turn(first_player_i);
      },
      GameState::Restart => {
        self.events.push(GameEvent::StartPick);
        let pres_i = self
          .ranking
          .iter()
          .position(|rank| rank == Ranking::President)
          .expect("Anarchy! No Presidents!");
        self.state = GameState::Pick(pres_i);
      },
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

    (0..num_players).for_each(|_| {
      ranking.push(Ranking::Citizen);
      hands.push(Hand::new());
    });

    let suits = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];

    Ok(Self {
      players,
      hands,
      ranking,
      state: GameState::InitialStart,
      pile: Deck::from_range(1..14,&suits),
      events: Vec::new(),
    })
  }

  fn stat_gamestate(&self) -> String {
    match self.state {
      GameState::Turn(idx) => {
        format!(f, "It's {}'s Turn.", self.players[idx])
      }
      GameState::Pick(idx) => {
        format!("{} {} is Picking.", self.ranking[idx], self.players[idx])
      }
      GameState::Offer(giver_i, taker_i) => {
        format!(
          "Waiting on {}'s offer between {} and {}.",
          self.ranking[giver_i],
          self.players[giver_i],
          self.players[taker_i]
        )
      }
      GameState::Exchange(giver_i, taker_i) => {
        format!(
          "Waiting on {}'s exchange between {} and {}.",
          self.ranking[taker_i],
          self.players[giver_i],
          self.players[taker_i]
        )
      }
    }
  }

  fn stat_players(&self) -> String {
    self
      .players
      .iter()
      .zip(self.hands.iter())
      .zip(self.ranking.iter())
      .map(|((name, hand), rank)| format!("  {} ({}): {}", name, rank, hand))
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
    self.hands.iter_mut().for_each(|h| {
      h.add(self.pile.deal(chunks).expect("Ran out of chunks!"));
    });

    //add leftovers
    (0..leftovers).for_each(|i| {
      self.hands[i].put(self.pile.draw().expect("Ran out of leftovers!"));
    });

    //find first player
    let start_card = Card::new(3, Suit::Clubs);
    self.hands.iter().position(|h| h.has(&start_card)).expect("No 3 of Clubs!");
  }
}
