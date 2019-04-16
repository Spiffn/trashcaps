pub mod cards;
mod state;

use std::string::ToString;
use std::fmt::{self, Display};
use state::{State, StateError};

struct Player {
    name: String,
    ranking: SocialRank,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SocialRank {
    President,
    VicePresident, //unused in this variant
    Citizen(usize), //used for seating later
    Scum,
}

//Handles pre-game config and post-game changes
pub struct Config {
    players: Vec<Player>,
    initial_game: bool,
}
pub enum ConfigError {
    ZeroPlayers,
    TooManyPlayers(usize),
}

pub enum Input {
    Play(Vec<Card>),
    Pass,
    Tax(Card),
}

pub enum Game {
    Play(State), //regular card play or pass
    Tax(State), //only used when president taxes scum
    End(Config), //used to indicate endgame
}
pub enum GameError {
    InvalidPlay(String),
}

pub struct GameStatus {
    players: &[Player],
    hands: &[Hand],
    current_player: &Player,
}

pub struct ConfigStatus {
    players: &[Player],
}

pub enum Status {
    Game(GameStatus),
    Config(ConfigStatus),
    players: Vec<String>,
    hands: &[Hand],
    current_player: &Player,
}

impl Player {
    fn new(name: String) -> Self {
        Player::with_rank(name, SocialRank::Scum)
    }

    fn with_rank(name: String, ranking: SocialRank) -> Self {
        Self {
            name,
            ranking,
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_ranking(&self) -> &SocialRank {
        &self.ranking
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format!("{} [{}]", self.name, self.ranking);
    }
}

//impl SocialRank

impl Display for SocialRank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Config {
    fn from_restart(players: Vec<Player>, count: usize) -> Self {
        Self {
            players,
            initial_game: false,
        }
    }

    pub fn new() -> Self {
        Self {
            players: Vec::new(),
            initial_game: true,
        }
    }

    pub fn add_player(&mut self, name: &str) -> &mut Self {
        self.players.push(Player::new(String::from(name)));
        self
    }

    pub fn start(self) -> Result<Game, ConfigError> {
        let state = State::init(self.players).map_err(|e| {
            match e {
                StateError::ZeroPlayers => ConfigError::ZeroPlayers,
                StateError::TooManyHands(h) => ConfigError::TooManyPlayers(h),
            }
        })?;
        if self.initial_game {
            Game::Play(state)
        } else {
            Game::Tax(state)
        }
    }

    pub fn status(&self) -> Status {
        Status {
            players:
        }
    }
}
impl ToString for ConfigError {
    fn to_string(&self) -> String {
        match self {
            ConfigError::NoPlayers => String::from("Not enough players!"),
            ConfigError::TooManyPlayers(count) => format!("Ruleset cannot support {0} players! ({0} > 52)", count),
        }
    }
}

//impl Input

impl Game {
    pub fn play(&mut self, cards: Vec<Card>) -> Result<Game, GameError> {
        unimplemented!();
    }

    pub fn pass(&mut self) -> Game {
        unimplemented!();
    }

    pub fn tax(&mut self, card: Card) -> Result<Game, GameError> {
        unimplemented!();
        //check appropriate tax
    }

    pub fn restart(self) -> Config {
        unimplemented!();
    }

    pub fn status(&self) -> Status {
    }
}
