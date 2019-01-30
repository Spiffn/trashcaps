use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
  Spades,
  Hearts,
  Clubs,
  Diamonds,
}

impl fmt::Display for Suit {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let name = match self {
      Suit::Spades => "♠",
      Suit::Hearts => "♥",
      Suit::Clubs => "♣",
      Suit::Diamonds => "♦",
    };
    write!(f, "{}", name)
  }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
  suit: Suit,
  rank: i64,
}

impl fmt::Display for Card {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}{}", self.rank, self.suit)
  }
}

impl Card {
  pub fn new(rank: i64, suit: Suit) -> Self {
    Self { rank, suit }
  }
}
