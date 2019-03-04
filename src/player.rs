use std::fmt;

use crate::card::Card;
use crate::deck::Hand;
use crate::rank::Ranking;

pub struct Player {
  pub hand: Hand,
  pub rank: Ranking,
}

impl Player {
  pub fn new() -> Self {
    Self {
      hand: Hand::default(),
      rank: Ranking::default(),
    }
  }

  pub fn has(&self, card: Card) -> bool {
    self.hand.has(&card)
  }

  //gets card of this type if it exists
  pub fn play(&mut self, selection: &[Card]) -> Option<Hand> {
    self.hand.play(selection)
  }

  pub fn add_card(&mut self, card: Card) {
    self.hand.put(card);
  }

  pub fn add_hand(&mut self, hand: Hand) 
  {
    self.hand.add(hand);
  }
}
