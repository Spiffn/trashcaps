use cards::{self, Hand, Rank, DealError};
use super::{Player, Config, ConfigError, Status};

struct Prev {
    player: usize,
    card: Rank,
    mult: usize,
}

//game state maintainer
pub struct State {
    players: Vec<Player>,
    hands: Vec<Hand>,
    current: usize,
    prev: Option<Prev>,
}
pub enum StateError {
    ZeroPlayers,
    TooManyPlayers,
}

//impl Prev
impl State {
    pub fn init(players: Vec<Player>) -> Self {
        let game = Game::new(players, 0).unwrap_or_else(|err| panic!("{}", err.to_string()));
        game
    }

    pub fn new(players: Vec<Player>) -> Result<Self, StateError> {
        let hands = card::deal(players_len()).map_err(|e| {
            match e {
                DealError::ZeroHands => StateError::ZeroPlayers,
                DealError::TooManyHands(h) => StateError::TooManyPlayers(h),
            }
        })?;
        Ok(Self {
            players,
            hands,
            current: 0,
            prev: None,
        })
    }

    pub fn restart(self) -> Config {
        self.players.sort_unstable_by_key(|p| p.ranking);
        GameConfig::from_restart(self.players)
    }

    pub fn player_has(&self, card: &Card) -> bool {
        self.hands[self.current].has(card)
    }

    pub fn status(&self) -> Status {
        let names = self.players.map(|p| p.name.clone()).collect();
        Status {
            players: names,
            hands: &self.hands[..],
            current: &self.players[self.current],
        }
    }

    pub fn tax(&mut self) {
        //moves best card from scum to president
        let pres = self.hands
            .first_mut()
            .expect("Unchecked No Players!");
        let scum = self.hands
            .last_mut()
            .expect("Unchecked No Players!");
        pres.add(scum.pop().expect("Unchecked Scum Hand Empty!"));
    }
}
