use rand::seq::SliceRandom;
use std::convert::AsRef;
use std::fmt;
use std::ops;
use std::iter;
use std::default::Default;
use std::convert;

use super::card::{Card, Suit};

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

impl iter::IntoIterator for Deck {
  type Item = Card;
  type IntoIter = std::vec::IntoIter<Card>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl iter::FromIterator<Card> for Deck {
  fn from_iter<I: iter::IntoIterator<Item=Card>>(iter: I) -> Self {
    iter.fold(Deck::new(), |mut d, c| d.put(c))
  }
}

impl Deck {
  pub fn new() -> Self {
    Self(Vec::new())
  }

  //given rank range and collection of suits, return a Deck
  //of combinatorial cards of all possible ranks and suits
  pub fn from_range(ranks: ops::Range<i64>, suits: &[Suit]) -> Self {
    let mut deck = Self::empty();
    for i in ranks {
      for s in suits {
        deck.put(Card::new(i, *s));
      }
    }
    deck
  }

  pub fn top(&self) -> Option<&Card> {
    self.0.last()
  }

  pub fn draw(&mut self) -> Option<Card> {
    self.0.pop()
  }

  pub fn put(&mut self, card: Card) {
    self.0.push(card);
  }

  /* deal()
   * Tries its best to return a Vec of Hands
   * Returns None if count is too high
   */
  pub fn deal(self, count: usize) -> Result<Vec<Hand>, ()> {
    self.shuffle();

    let portion: usize = match count {
      large if large > self.len() => {
        return Err(());
      },
      0 | 1 => 1,
      chunk => self.len() / chunk,
    };

    if portion == 1 {
      Ok(self.0)
    } else {
      let mut res: Vec<Hand> = Vec::with_capacity(count);
      (0..count).for_each(|| {
        res.push(self.0.split_off(portion).drain(..));
      });

      if self.len() > 0 {
        self.0.drain(..)
          .zip(res.iter_mut())
          .for_each(|(c, h)| {
            h.put(c);
          });
      }
      Ok(res)
    }
  }

  pub fn shuffle(&mut self) {
    let mut rng = rand::thread_rng();
    self.0.shuffle(&mut rng); //shuffle impl by rand crate
  }
}

//A strictly ordered set of Cards for easy searching.
//Use Deck if order doesn't matter.
pub struct Hand(Vec<Card>);

impl convert::AsRef<[Card]> for Hand {
  fn as_ref(&self) -> &[Card] {
    &self.0[..]
  }
}

impl iter::IntoIterator for Hand {
  type Item = Card;
  type IntoIter = std::vec::IntoIter<Card>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
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

  pub fn put(&mut self, card: Card) {
    self.0.push(card);
  }

  pub fn add(&mut self, hand: Hand) {
    hand.drain(..).for_each(|c| self.put(c));
  }

  pub fn has(&self, card: &Card) -> bool {
    self.pos(card).is_some()
  }

  pub fn get_suits(&self, suit: &Suit) -> Option<&[Card]> {
    let start_i = self
      .0
      .iter()
      .position(|c| c.is_suit(suit))?;
    let end_i = self.len() - self
      .0
      .iter()
      .rev()
      .position(|c| c.is_suit(suit))?;
    Some(&self.0[start_i..=end_i])
  }

  /*
   * essentially an intersection between the presented card slice
   * and what is in the hand
   */
  pub fn play(&mut self, selection: &[Card]) -> Option<Hand> {
    unimplemented!();
    let played: Hand = selection
      .iter()
      .filter_map(|c| {
        self.pos(c).map(self.take)
      })
      .collect();
    if played.is_empty() {
      None
    } else {
      Some(played)
    }
  }

  pub fn cards(&self) -> std::slice::Iter<Card> {
    self.0.iter()
  }

  pub fn is_empty(&self) -> bool {
    0 == self.0.len()
  }

  fn pos(&self, card: &Card) -> Option<usize> {
    self.0.binary_search(card).ok()
  }
}
