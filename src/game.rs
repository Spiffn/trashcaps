use crate::caps::card;
use crate::caps::deck::{Deck, Hand};
use crate::collection::CardCollection;
use std::fmt;
use std::mem;

#[derive(Debug)]
pub struct Player {
  pub name: String,
  pub hand: Deck,
}

impl fmt::Display for Player {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: {}", self.name, self.hand)
  }
}

impl Player {
  pub fn new(name: &str) -> Self {
    Self {
      name: String::from(name),
      hand: Deck::empty(),
    }
  }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
  President(i8),
  Citizen,
  Scum(i8),
}

pub enum Plays {
  Turn(),
  Completion(i64, u8), //(rank, number)
  Bomb(card::Suit),
}

#[derive(Debug)]
pub struct Game {
  players: &mut [Player],
  pile: Deck,
  status: Vec<Status>
}

impl Game {
  pub fn init(players: &mut [Player], deck: Deck) -> Result<Self> {
    let player_count = players.len();
    let status = Vec::with_capacity(player_count);
    if player_count >= 4 {
      status.push(Status::Scum(0));
      status.push(Status::Scum(1));
      (0..player_count - 4).for_each(|_| {
        hierarchy.push(Status::Citizen);
      });

      status.push(Status::President(0));
      status.push(Status::President(1));
    } else {
      status.push(Status::Scum(0));
      if player_count == 3 {
        status.push(Status::Citizen);
      }
      status.push(Status::President(0));
    }
    let mut res = Self {
      players,
      Deck::empty(),
      status,
    }

    res.deal(deck);
  }

  fn deal(&mut self, deck: Deck) {
    let num_players = self.players.len();
    let num_cards = deck.len();
    let chunks = num_cards / num_players;
    let leftovers = num_cards % num_players;

    for player in self.players.iter_mut() {
      player
        .hand
        .add(deck.deal(chunks).expect("Ran out of chunks!"));
    }

    for i in 0..leftovers {
      players[i % num_players]
        .hand
        .put(deck.draw().expect("Ran out of leftovers!"));
    }
  }
}
