use rand::seq::SliceRandom;
use std::convert::{AsMut, AsRef};
use std::fmt;
use std::ops;

use super::card::{Card, Suit};
use crate::collection::CardCollection;

#[derive(Debug)]
pub struct Deck(Vec<Card>);

impl fmt::Display for Deck {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let cnt = self.as_ref().len();
    write!(f, "[")?;
    let enumiter = self
      .as_ref()
      .iter()
      .enumerate()
      .map(|idxelem| (idxelem.0, idxelem.1.to_string()));
    for (i, cardstr) in enumiter {
      if i < cnt - 1 {
        write!(f, "{}, ", cardstr)?;
      } else {
        write!(f, "{}", cardstr)?;
      }
    }
    write!(f, "]")?;
    Ok(())
  }
}

impl AsRef<[Card]> for Deck {
  fn as_ref(&self) -> &[Card] {
    self.0.as_ref()
  }
}

impl AsMut<[Card]> for Deck {
  fn as_mut(&mut self) -> &mut [Card] {
    self.0.as_mut()
  }
}

impl CardCollection<Card> for Deck {
  fn put(&mut self, card: Card) {
    self.0.push(card);
  }
  fn to_vec(self) -> Vec<Card> {
    self.0
  }
}

impl Deck {
  pub fn empty() -> Self {
    Self(Vec::new())
  }
  //given rank range and collection of suits, return a shuffled Deck
  //of combinatorial cards of all possible ranks and suits
  pub fn from_range(ranks: ops::Range<i64>, suits: &[Suit]) -> Self {
    let mut deck = Self::empty();
    for i in ranks {
      for s in suits {
        deck.put(Card::new(i, *s));
      }
    }
    deck.shuffle();
    deck
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn draw(&mut self) -> Option<Card> {
    self.0.pop()
  }

  //if possible, moves [count] cards from Stack
  //if not possible, returns None
  pub fn deal(&mut self, count: usize) -> Option<Self> {
    let end = self.len();
    if end < count {
      None
    } else {
      Some(Self(self.0.split_off(end - count)))
    }
  }

  pub fn shuffle(&mut self) {
    let mut rng = rand::thread_rng();
    self.0.shuffle(&mut rng); //shuffle impl by rand crate
  }
}

#[derive(Debug)]
pub struct Hand(Vec<Card>);

impl AsRef<[Card]> for Hand {
  fn as_ref(&self) -> &[Card] {
    self.0.as_ref()
  }
}

impl AsMut<[Card]> for Hand {
  fn as_mut(&mut self) -> &mut [Card] {
    self.0.as_mut()
  }
}

impl CardCollection<Card> for Hand {
  fn put(&mut self, card: Card) {
    self.0.push(card);
    self.0.sort_unstable();
  }

  fn to_vec(self) -> Vec<Card> {
    self.0
  }
}

impl fmt::Display for Hand {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let cnt = self.as_ref().len();
    write!(f, "[")?;
    let enumiter = self
      .as_ref()
      .iter()
      .enumerate()
      .map(|idxelem| (idxelem.0, idxelem.1.to_string()));
    for (i, cardstr) in enumiter {
      if i < cnt - 1 {
        write!(f, "{}, ", cardstr)?;
      } else {
        write!(f, "{}", cardstr)?;
      }
    }
    write!(f, "]")?;
    Ok(())
  }
}

impl Hand {
  pub fn play(&mut self, rank: i64, suit: Suit) -> Option<Card> {
    let card = Card::new(rank, suit);
    let res = self.0.binary_search(&card);
    res.ok().map(|idx| self.0.remove(idx))
  }
}
