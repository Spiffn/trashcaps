use std::convert::AsRef;

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
