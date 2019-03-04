use regex::Regex;
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

impl Suit {
  pub fn all() -> Vec<Self> {
    vec![Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds]
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

  pub fn is_rank(&self, rank: &i64) -> bool {
    self.rank == rank
  }

  pub fn is_suit(&self, suit: &Suit) -> bool {
    self.suit == suit
  }

  pub fn try_from(s: &str) -> Result<Self, ()> {
    //must be in the form '<rank><suit>'
    // suit can be unicode or first character (i.e. 13C or 1♠)
    let re = Regex::new(r"^\s*(\d+)([shcdSHCD♠♥♣♦])\s*$")
      .expect("Invalid Regex!");
    let captures = re.captures(s).ok_or(())?;
    let rank_match = captures.get(1).ok_or(())?;
    let suit_match = captures.get(2).ok_or(())?;

    let rank_str: &str = rank_match.as_str();
    let suit_str: &str = suit_match.as_str();

    let rank: i64 = rank_str.parse().map_err(|_| ())?;
    let suit: Suit = match suit_str {
      "s" | "S" | "♠" => Suit::Spades,
      "h" | "H" | "♥" => Suit::Hearts,
      "c" | "C" | "♣" => Suit::Clubs,
      "d" | "D" | "♦" => Suit::Diamonds,
      _ => return Err(()),
    };
    Ok(Self::new(rank, suit))
  }
}
