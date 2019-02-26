use super::card::{Card, Suit};
use super::deck::{Deck, Hand, CardCollection};
use super::event::{GameEvent, self};
use std::default::Default;
use std::convert;
use std::fmt;
use std::ops;

#[derive(Debug)]
pub struct Game<'p> {
  players: Vec<Player<'p>>,
  state: GameState,
  pile: Deck,
  selection: Hand,
}

impl<'p> fmt::Display for Game<'p> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for p in self.players.iter() {
      writeln!(f, "{}", p)?;
    }
    match self.state {
      GameState::Start => write!(f, "Game has yet to begin.")?,
      GameState::Restart => write!(f, "Game awaiting restart")?,
      GameState::Turn(pidx) => write!(f,
        "It's {}'s turn.", 
        self.players[pidx].name)?,
      GameState::Pick(pidx, _) => write!(f,
        "{} is picking.",
        self.players[pidx].name)?,
      GameState::Offer(g_i, t_i) => {
        let giver = self.players[g_i];
        let taker = self.players[t_i];
        write!(f,
          "Waiting on {}({})'s offer to {}({}).",
          giver.name,
          giver.rank,
          taker.name,
          taker.rank
        )?;
      },
      GameState::Exchange(g_i, t_i) => {
        let giver = self.players[g_i];
        let taker = self.players[t_i];
        write!(f,
          "Waiting on {}({})'s response to {}({})'s offer.",
          taker.name,
          taker.rank,
          giver.name,
          giver.rank
        )?;
      },
    }
    writeln!(f, "{}", self.state.prompt())
  }
}

impl<'p> Game<'p> {
  pub fn prompt(&self) -> InputType {
    self.state.prompt()
  }

  pub fn is_over(&self) -> bool {
    self.state == GameState::Restart
  }

  pub fn get_hand(&self, name: &'p str) -> Option<&Hand> {
    self
      .player
      .iter()
      .find(|p| p.name == name)
      .map(|p| &p.hand)
  }

  pub fn play(&mut self, input: Input) -> Result<Vec<GameEvent>, String> {
    let errmsg = String::from("Invalid Input");
    let mut evt = Vec::new();
    match input {
      Input::Begin => {
        match self.state {
          GameState::Start => {
            evt.push(GameEvent::Start);
            self.initial_deal();
            //find first player
            let start_card = Card::new(3, Suit::Clubs);
            let first_player_i = self
              .players
              .iter()
              .position(|p| p.has(&start_card))
              .expect("No 3 of Clubs!");
            self.state = GameState::Turn(first_player_i);
            Ok(evt)
          },
          GameState::Restart => {
            evt.push(GameEvent::StartPick);
            let pres_i = self
              .players
              .iter()
              .position(|p| p.rank == Ranking::President)
              .expect("Anarchy! No Presidents!");
            self.state = GameState::Pick(pres_i);
            Ok(evt)
          },
          _ => Err(errmsg)
        }
      },
      Input::PlayCard(p_opt, cards) => {
        let p_idx_opt: Option<usize> = if p_opt.is_some() {
          Some(self
            .players
            .iter()
            .position(|p| p.name == name)
            .ok_or(format!("No such player {}!", name))?)
        } else {
          None
        };
        match self.state {
          GameState::Turn(idx) =>,
          GameState::Offer(g_i, t_i) =>,
          GameState::Exchange(g_i, t_i) =>,
          _ => Err(errmsg)
        }
      },
      Input::Selection(p_opt, sel) => {
        match self.state {
          GameState::Pick(p_i, choices) => {
          },
          _ => Err(errmsg)
        }
      }
    }
  }

  pub fn new(names: &'p str) -> Result<Self, String> {
    let mut players = Vec::new();
    let num_players = names.len();
    if num_players < 2 {
      return Err(String::from("There is no lone Capitalism."));
    }

    names
      .iter()
      .for_each(|nm| {
        players.push(Player::new(nm));
      });

    Ok(Self {
      players,
      state: GameState::Start,
      pile: Deck::from_range(1..14, &Suit::all()[..]),
      selection: Hand::new(),
    })
  }

  fn initial_deal(&mut self) {
    let num_players = self.players.len();
    let num_cards = self.pile.len();
    let chunks = num_cards / num_players;

    //add deck to hands
    self
      .players
      .iter_mut()
      .zip(pile.deal_iter())
      .for_each(|p, pile| {
        p.add_pile(
          self
            .pile
            .deal(chunks)
            .expect("Ran out of chunks!"));
      });

    //add leftovers
    self
      .pile
      .zip(self.players.iter_mut())
      .for_each(|(c, p)| { p.put(c); });
  }

  fn pick(&mut self, current: usize, player: usize, selection: usize) -> bool {
  }

  fn turn(&mut self, current: usize, player: usize, ) -> bool {
  }

  fn offer(&mut self) -> bool {
  }

  fn exchange(&mut self) -> bool {
  }
}
