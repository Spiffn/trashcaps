use std::fmt;

use card::Card;
use deck::Hand;

#[derive(Debug)]
struct Player<'p> {
  pub hand: Hand,
  pub rank: Ranking,
}

impl<'p> fmt::Display for Player<'p> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}({}): [{}]", self.name, self.hand, self.rank)
  }
}

impl<'p> Player<'p> {
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
  pub fn play(&mut self, idx: usize) -> Option<Card> {
    self.hand.play(idx)
  }

  pub fn add_card(&mut self, card: Card) {
    self.hand.put(card);
  }

  pub fn add_pile<P>(&mut self, pile: P) 
    where P: CardCollection<Card>
  {
    self.hand.add(hand);
  }
}
