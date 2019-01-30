use std::convert::AsRef;

pub trait CardCollection<C>: AsRef<[C]> {
  fn put(&mut self, card: C); //add card to collection
  fn to_vec(self) -> Vec<C>;

  fn add(&mut self, other: impl CardCollection<C>) {
    for card in other.to_vec().drain(..) {
      self.put(card);
    }
  }
}
