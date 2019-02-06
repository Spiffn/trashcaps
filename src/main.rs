mod caps;
mod collection;
mod ui;

use caps::game::Game;

fn print_events(events: Vec<String>) {
  events
    .drain(..)
    .for_each(|evt| println!("{}", evt));
}

fn main() {
  const PLAYERS: [&str; 3] = ["Lawrence", "Timothy", "James"];

  let game = Game::init(&PLAYERS).unwrap_or_else(|err| panic!("{}", err));
  let mut turn = 0u32;
  loop {
    print_events(game.start());

    println!("TURN {}", turn);
    println!("=======");



    if !ui::yesno("Continue?").expect("IO Error") {
      println!("Goodbye!");
      break;
    }
    turn += 1;
  }
}
