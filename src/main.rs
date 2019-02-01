mod caps;
mod collection;
mod ui;

use caps::game::Game;

fn main() {
  const PLAYERS: [&str; 3] = ["Lawrence", "Timothy", "James"];

  let game = Game::init(&PLAYERS).expect("Error initializing Game!");
  let mut turn = 0u32;
  loop {
    println!("TURN {}", turn);
    println!("=======");
    print!("{}", game);
    if !ui::yesno("Continue?").expect("IO Error") {
      println!("Goodbye!");
      break;
    }

    turn += 1;
  }
}
