mod caps;
mod collection;
mod game;
mod ui;

use caps::card::Suit;
use caps::deck::Deck;
use game::Player;

fn main() {
  let mut players = [
    Player::new("Lawrence"),
    Player::new("Timothy"),
    Player::new("James"),
  ];
  let suits = [Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds];

  loop {
    game::deal(&mut players[..], Deck::from_range(1..14, &suits[..]));
    println!("Current Players: ");

    for player in players.iter() {
      println!("  {}", player);
    }

    if !ui::yesno("Continue?").expect("IO Error") {
      println!("Goodbye!");
      break;
    }
  }
}
