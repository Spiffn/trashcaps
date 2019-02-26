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
pub trait CardCollection: iter::IntoIter<Item=Card> {
  fn put(&mut self, card: Card); //add card to collection
  fn merge(&mut self, other: impl CardCollection<Card>) {
    for card in other {
      self.put(card);
    }
  }
}

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

impl iter::IntoIter for Deck {
  type Item = Card;
  type IntoIter = std::vec::IntoIter<Card>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl CardCollection<Card> for Deck {
  fn put(&mut self, card: Card) {
    self.0.push(card);
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

  /* deal()
   * Tries its best to return a Vec of Hands
   * will return empty vecs if count too high
   */
  pub fn deal(self, count: usize) -> Vec<Hand> {
    unimplemented!();
    self.shuffle();

    let portion: usize = match count {
      0 | 1 => 1,
      large if large >= self.len() => self.len(),
      chunk => self.len() / chunk,
    };

    if portion == 1 {
      vec![self]
    } else {
      let mut res = self
        .0
        .into_iter()
        .enumerate()
        .fold(Vec::new(), |mut v, (i, c)| {
          if 0 == i % portion {
            v.push();
          }
        });
      (0..count - res.len()).for_each(|| {
        res.push(Hand::new());
      });
    }

    let end = if self.len();
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
pub struct Hand(Vec<Card>);

impl iter::IntoIter for Hand {
  type Item = Card;
  type IntoIter = std::vec::IntoIter<Card>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl CardCollection<Card> for Hand {
  fn put(&mut self, card: Card) {
    self.0.push(card);
    self.0.sort_unstable();
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

impl ops::Index<u64> for Hand {
  type Output: [Card];

  fn index(&self, rank: u64) -> &Self::Output {
  }
}

impl ops::Index<Suit> for Hand {
  type Output: [Card];

  fn index(&self, suit: Suit) -> &Self::Output {
  }
}

impl Hand {
  pub fn new() -> Self {
    Self(Vec::new())
  }

  pub fn has(&self, card: &Card) -> bool {
    self.pos(card).is_some()
  }

  /*
   * essentially an intersection between the presented card slice
   * and what is in the hand
   */
  pub fn play(&mut self, selection: &[Card]) -> Option<Hand> {
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

  fn take(&mut self, idx: usize) -> Option<Card> {
    self.0.remove(idx)
  }
}
