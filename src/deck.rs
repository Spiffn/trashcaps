use rand::seq::SliceRandom;
use std::convert::AsRef;
use std::fmt;
use std::ops;
use std::iter;
use std::default::Default;

use super::card::{Card, Suit};

//this trait defines CardCollection as compatible with all other CardCollections
//assuming that the card C is the same.
//to_vec is just there so an auto impl drain() is possible.
pub trait CardCollection<C>: AsRef<[C]> {
  fn put(&mut self, card: C); //add card to collection
  fn to_vec(self) -> Vec<C>;

  fn add(&mut self, other: impl CardCollection<C>) {
    for card in other.to_vec().drain(..) {
      self.put(card);
    }
  }
}

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

/* REQUIRED */
impl AsRef<[Card]> for Deck {
  fn as_ref(&self) -> &[Card] {
    self.0.as_ref()
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

/* OTHER TRAITS */
impl iter::Iterator for Deck {
  type Item = Card;
  fn next(&mut self) -> Option<Self::Item> {
    self.draw()
  }
}

impl iter::ExactSizeIterator for Deck {
  fn len(&self) -> usize {
    self.0.len()
  }

  fn is_empty(&self) -> bool {
    0 == self.0.len()
  }
}

//merges all CardCollection objects into a Deck
impl iter::FromIterator<CardCollection<Card>> for Deck {
  fn from_iter<I: IntoIterator<Item=CardCollection<Card>>>(iter: I) -> Self {
    iter.fold(Deck::new(), |mut d, other| d.add(other))
  }
}

impl iter::FromIterator<Card> for Deck {
  fn from_iter<I: IntoIterator<Item=Card>>(iter: I) -> Self {
    iter.fold(Deck::new(), |mut d, c| d.put(c))
  }
}

impl Deck {
  pub fn new() -> Self {
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

  pub fn top(&self) -> Option<&Card> {
    self.0.last()
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

//A strictly ordered set of Cards for easy searching.
//Use Deck if order doesn't matter.
#[derive(Debug)]
pub struct Hand(Vec<Card>);

impl AsRef<[Card]> for Hand {
  fn as_ref(&self) -> &[Card] {
    self.0.as_ref()
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
    let cnt = self.0.len();
    write!(f, "[")?;
    let enumiter = self
      .0
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

impl iter::FromIterator<Card> for Hand {
  fn from_iter<I: IntoIterator<Item=Card>>(iter: I) -> Self {
    iter.fold(Hand::new(), |mut d, c| d.put(c))
  }
}

impl Default for Hand {
  fn default() -> Self {
    Hand::new()
  }
}

impl Hand {
  pub fn new() -> Self {
    Self(Vec::new())
  }

  pub fn has(&self, card: &Card) -> bool {
    self.pos(card).is_some()
  }

  pub fn play(&mut self, card: &Card) -> Option<Card> {
    let res = self.pos(card);
    res.map(|idx| self.0.remove(idx))
  }

  fn pos(&self, card: &Card) -> Option<usize> {
    self.0.binary_search(card).ok()
  }
}
