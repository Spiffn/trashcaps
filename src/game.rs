use super::card::{Card, Suit};
use super::deck::{Deck, Hand, CardCollection};
use super::event::GameEvent;
use std::default::Default;
use std::convert;
use std::fmt;
use std::ops;

struct Player<'p> {
  pub name: &'p str,
  hand: Hand,
  pub rank: Ranking,
}

impl<'p> fmt::Display for Player<'p> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}({}): [{}]", self.name, self.hand, self.rank)
  }
}

impl<'p> Player<'p> {
  pub fn new(name: &'p str, hand: Hand) -> Self {
    Self {
      name,
      hand,
      rank: Ranking::default(),
    }
  }

  pub fn has(&self, card: Card) -> bool {
    self.hand.has(&card)
  }

  pub fn has_rank(&self, rank: i64) -> bool {
    Suit::all()
      .into_iter()
      .map(|s| Card::new(rank, s))
      .all(|c| self.hand.has(&c))
  }

  //gets card of this type if it exists
  pub fn play(&mut self, card: Card) -> Option<Card> {
    self.hand.play(card)
  }

  //gets all card of some rank
  pub fn play_by_rank(&mut self, rank: i64) -> Option<Deck> {
    let d: Deck = Suit::all()
      .into_iter()
      .filter_map(|s| self.play(Card::new(rank,s)))
      .collect();
    if 0 == h.len() {
      None
    } else {
      Some(d)
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
  Start, //waiting on game start/restart confirmation
  PlayCard, //player needs to play a card
  SelectPile(usize), //player needs to select a pile from 0..maximum
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameState<'n, 'p> { // <'names of players, 'player objects>
  Start,   //rankings are undetermined, deal equally and start with 3 of clubs
  Restart, //rankings were determined, President first
  Turn(&'n str),
  Pick(&'n str, usize), //(player, choices) player must pick 0..choices
  Offer(&'p Player<'n>, &'p Player<'n>), //lower rank offering (Scum, President)
  Exchange(&'p Player<'n>, &'p Player<'n>), //Higher rank conducting exchange (Scum, President)
}

impl<'p> fmt::Display for GameState<'p> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      GameState::Start => write!(f, "Game has yet to begin."),
      GameState::Restart => write!(f, "Game awaiting restart"),
      GameState::Turn(p) => write!(f, "It's {}'s turn.", p),
      GameState::Pick(p, _) => write!(f, "{} is picking.", p),
      GameState::Offer(giver, taker) => write!(f,
        "Waiting on {}({})'s offer to {}({}).",
        giver.name,
        giver.rank,
        taker.name,
        taker.rank
      ),
      GameState::Exchange(giver, taker) => write!(f,
        "Waiting on {}({})'s response to {}({})'s offer.",
        taker.name,
        taker.rank,
        giver.name,
        giver.rank
      ),
    }
  }
}

impl<'p> GameState<'p> {
  pub fn prompt(&self) -> InputType {
    use GameState;
    match self {
      Start | Restart => InputType::Start,
      Turn(_) | Offer(_) | Exchange(_) => InputType::PlayCard,
      Pick(_, max) => InputType::SelectPile(max),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Ranking {
  President,
  VicePresident,
  Citizen,
  ViceScum,
  Scum,
}

impl fmt::Display for Ranking {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Ranking::VicePresident => write!(f, "Vice President"),
      Ranking::ViceScum => write!(f, "Vice Scum"),
      _ => write!(f, "{:?}", self),
    }
  }
}

impl Default for Ranking {
  fn default() -> Self {
    Ranking::Citizen
  }
}

#[derive(Debug)]
pub struct Game<'p> {
  players: Vec<Player<'p>>,
  state: GameState,
  pile: Deck,
  selection: Hand,
  events: Vec<GameEvent<'p>>,
}

impl<'p> fmt::Display for Game<'p> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for p in self.players.iter() {
      writeln!(f, "{}", p)?;
    }
    writeln!(f, "{}", self.state)?;
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

  pub fn start(&mut self) -> Result<Vec<String>, String> {
    match self.state {
      GameState::Start => {
        self.events.push(GameEvent::Start);
        let first_player_i = self.initial_deal();
        self.state = GameState::Turn(first_player_i);
      }
      GameState::Restart => {
        self.events.push(GameEvent::StartPick);
        let pres_i = self
          .ranking
          .iter()
          .position(|&rank| rank == Ranking::President)
          .expect("Anarchy! No Presidents!");
        self.state = GameState::Pick(pres_i);
      }
      _ => return Err(String::from("Can't call start() at this time")),
    }
    Ok(self.flush_events())
  }

  pub fn play(
    &mut self,
    player_idx: usize,
    hand_str: &[Card],
  ) -> Result<Vec<GameEvent>, String> {
    //validate player
    //- exists
    //validate cards
    //- are valid cards
    //- are of the player's hand
    //validate play
    //- turn order

    //execute play->events
    //otherwise->errorstr
    unimplemented!();
  }

  pub fn select(
    &mut self,
    player_idx: usize,
    selection: usize) -> Result<Vec<GameEvent>, String> {

  }

  pub fn init(names: &'p str) -> Result<Self, String> {
    let mut players = Vec::new();
    let num_players = names.len();
    if num_players < 2 {
      return Err(String::from("There is no lone Capitalism."));
    }

    let mut ranking = Vec::with_capacity(num_players);
    let mut hands = Vec::with_capacity(num_players);

    (0..num_players).for_each(|_| {
      ranking.push(Ranking::Citizen);
      hands.push(Hand::new());
    });

    Ok(Self {
      players ,
      state: GameState::Start,
      pile: Deck::from_range(1..14, &Suit::all()[..]),
      selection: Hand::new(),
      events: Vec::new(),
    })
  }

  fn initial_deal(&mut self) -> usize {
    let num_players = self.players.len();
    let num_cards = self.pile.len();
    let chunks = num_cards / num_players;
    let leftovers = num_cards % num_players;

    //add deck to hands
    let hands_iter = self.hands.iter_mut();
    for h in hands_iter {
      h.add(self.pile.deal(chunks).expect("Ran out of chunks!"));
    }

    //add leftovers
    (0..leftovers).for_each(|i| {
      self.hands[i].put(self.pile.draw().expect("Ran out of leftovers!"));
    });

    //find first player
    let start_card = Card::new(3, Suit::Clubs);
    self
      .hands
      .iter()
      .position(|h| h.has(&start_card))
      .expect("No 3 of Clubs!")
  }

  fn validate_cards(&self, player_i: usize, hand_str: &[&str])
  -> Result<Deck, String> {
    hand_str
      .iter()
      .map(|card_str| {
        let err_msg = format!("Invalid Card {}!", card_str);
        if card_str.ends_with("*") {
          //wildcard
          let rank: i64 = card_str
            .get(..card_str.len()-1)
            .ok_or(err_msg)?
            .parse()
            .map_err(err_msg)?;
          let suits = [
            Suit::Spades,
            Suit::Hearts,
            Suit::Diamonds,
            Suit::Clubs];
          Ok(suits
            .iter()
            .filter_map(|s| {
              let c = Card::new(rank, s);
              if self.hands.has(&c) {
                Some(c)
              } else {
                None
              }
            })
            .collect()::<Deck>())
        } else {
          let card = Card::tryfrom(card_str).ok_or(err_msg)?;
          if self.hands[player_i].has(&card) {
            let mut res = Deck::new();
            res.put(card);
            Ok(res)
          } else {
            Err(format!("{} doesn't have a {}",
              self.players[player_i].as_ref(), card_str))
          }
        }
      })
      .collect()
  }
}
