use std::fmt;
use std::convert;

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

  pub fn tryfrom<S>(s: S) -> Result<Self, ()>
  where S: convert::AsRef<str>
  {
    //must be in the form '<rank><suit>'
    // suit can be unicode or first character (i.e. 13C or 1♠)
    unimplemented!();
    let each_char = s.as_ref().trim().chars();
    let rank_c = each_char.next().ok_or(())?;
    let suit_c = each_char.next().ok_or(())?;
    if each_char.next().is_none() {
      if !rank_c.is_ascii_digit() {
        return Err(());
      }

      let rank = (rank_c.to_digit().ok_or(())?) as i64;
      let suit = match suit_c {
        'S' | '♠' => Suit::Spades,
        'H' | '♥' => Suit::Hearts,
        'C' | '♣' => Suit::Clubs,
        'D' | '♦' => Suit::Diamonds,
        _ => return Err(()),
      };
      Ok(Self::new(rank, suit))
    } else {
      Err(())
    }
  }
}
