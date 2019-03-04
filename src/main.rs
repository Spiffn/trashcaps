mod card;
mod deck;
mod event;
mod rank;
mod state;
mod player;
mod ui;

use regex::Regex;

use player::Player;
use state::GameState;
use card::{Suit, Card};
use deck::{Deck, Hand};

fn tokenize(
  names: &str,
  players: &[Player],
  play: &str,
) -> Result<(Option<usize>, Hand), ()> {
  // [/as <player name>] <card1>,<card2>,...
  let re = Regex::new(
    r"^(?:/as\s*(\S+)\s+)?((?:\d+[shcdSHCD♠♥♣♦*],\s*)*\d+[shcdSHCD♠♥♣♦*])$",
  )
  .expect("Invalid Tokenizer Regex!");

  let captures = re.captures(play.trim()).ok_or(())?;
  let player_token: &str = captures.get(1).ok_or(())?.as_str();

  let players_opt: Option<usize> = names
    .iter()
    .position(|name| name == player_token);

  let cards_match = captures.get(2).ok_or(())?;
  let card_selection = cards_match
    .as_str()
    .split(',')
    .map(|c_str| c_str.trim())
    .fold(Hand::new(), |mut h, c_str| {
      if let Ok(c) = Card::try_from(c_str) {
        h.put(c);
      } else {
        let rank: i64 = Regex::new(r"^(\d+)\*$")
          .expect("Invalid * regex!")
          .captures(c_str)
          .expect("Impossible card format")
          .get(1)
          .expect("Impossible card format")
          .as_str()
          .parse();

        /* Create all possible cards of this rank */
        Suit::all()
          .iter()
          .map(|s| Card::new(rank, s))
          .for_each(|c| {
            h.put(c);
          });
      }
      h
    });
  Ok((players_opt, card_selection))
}

fn main() {
  const PLAYERNUM: usize = 3;
  const PLAYERNAMES: [&str; PLAYERNUM] = ["Lawrence", "Timothy", "James"];

  let mut players: Vec<Player> = (0..PLAYERNUM)
    .fold(Vec::with_capacity(PLAYERNUM), |mut p| {
      p.push(Player::new())
    });
  let mut state = GameState::new();
  let mut turn: usize = 0;

  loop {
    if !ui::yesno("Continue?").expect("IO Error") {
      println!("Goodbye!");
      break;
    }
  }
}
