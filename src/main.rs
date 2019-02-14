mod card;
mod deck;
mod event;
mod game;
mod ui;

use caps::game::{Game, GameStatOpt};
use regex::Regex;

fn print_events(mut events: Vec<String>) {
  events.drain(..).for_each(|evt| println!("{}", evt));
}

fn tokenize<'input>(
  play: &'input str,
) -> Result<(Option<&'input str>, Vec<&'input str>), ()> {
  // [/as <player name>] <card1>,<card2>,...
  let re = Regex::new(
    r"^(?:/as\s*(\S+)\s+)?((?:\d+[shcdSHCD♠♥♣♦*],\s*)*\d+[shcdSHCD♠♥♣♦*])$",
  )
  .expect("Invalid Tokenizer Regex!");
  let txt = play.trim();

  let captures = re.captures(txt).ok_or(())?;
  let players_match = captures.get(1).map(|m| m.as_str());
  let cards_match = captures.get(2).ok_or(())?;
  Ok((players_match, cards_match.as_str().split(',').collect()))
}

fn main() {
  const PLAYERS: [&str; 3] = ["Lawrence", "Timothy", "James"];

  let mut game = Game::init(&PLAYERS).unwrap_or_else(|err| panic!("{}", err));
  let mut turn = 0u32;
  loop {
    print_events(game.start().unwrap_or_else(|err| panic!("{}", err)));
    while !game.is_over() {
      println!("TURN {}", turn);
      println!("-----------");
      println!("{}", game.stat(GameStatOpt::Players));
      println!("{}", game.stat(GameStatOpt::GameState));
      let current_player =
        game.get_current_player().expect("No current player?");
      loop {
        //input loop for a single turn
        let play = ui::prompt("TRASHCAPS>>").expect("IO Error");
        let play_tok_res = tokenize(&play);
        if let Ok((player, cards)) = play_tok_res {
          let player_name = player.unwrap_or(current_player);
          let evt_opt = game.play(player_name, &cards[..]);
          if let Ok(events) = evt_opt {
            print_events(events);
            break;
          } else if let Err(mesg) = evt_opt {
            println!("{}", mesg);
          }
        } else if play_tok_res.is_err() {
          println!("Input could not be parsed.");
        }
      }
      turn += 1;
    }

    if !ui::yesno("Continue?").expect("IO Error") {
      println!("Goodbye!");
      break;
    }
  }
}
