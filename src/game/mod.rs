use std::string::ToString;
use cards::{self, Hand, Rank, DealErrors};

struct Player {
    name: String,
    ranking: SocialRank,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct SocialRank {
    President,
    VicePresident, //unused in this variant
    Citizen(usize), //used for seating later
    Scum,
}

struct Prev {
    player: usize,
    card: Rank,
    mult: usize,
}

//Handles pre-game config and post-game changes
pub struct GameConfig {
    players: Vec<Player>,
    game_count: usize,
}
pub enum GameConfigError {
    NoPlayers,
    TooManyPlayers(usize),
}

//Regular game loop
pub struct Game {
    count: usize, //number of games
    players: Vec<Player>,
    hands: Vec<Hand>,
    current: usize,
    prev: Option<Prev>, //last name
}

pub enum Input {
    Play(Vec<Card>),
    Pass,
    Tax(Card), //President-only action during exchange
    Status,
    StatusVerbose,
}

pub enum Event {
    Prompt, //that is, check for input from player
    End, //end the game or prompt a restart
}

//[IMPLS]
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
}
//SOCIALRANK
//PREV
impl GameConfig {
    fn from_restart(players: Vec<Player>, count: usize) -> Self {
        Self {
            players,
            game_count: count.saturating_add(1),
        }
    }

    pub fn new() -> Self {
        Self {
            players: Vec::new(),
            game_count: 0,
        }
    }

    pub fn add_player(&mut self, name: String) -> &mut Self {
        self.players.push(Player::new(name));
        self
    }

    pub fn start(self) -> Result<Game, GameConfigError> {
        if self.initial_game {
            Game::init(self.players)?
        } else {
            Game::reinit(self.players, self.game_count)?
        }
    }
}
impl ToString for GameConfigError {
    fn to_string(&self) -> String {
        match self {
            GameConfigError::NoPlayers => String::from("Not enough players!"),
            GameConfigError::TooManyPlayers(count) => format!("Ruleset cannot support {0} players! ({0} > 52)", count),
        }
    }
}

impl Game {
    fn init(players: Vec<Player>) -> Self {
        let game = Game::new(players, 0).unwrap_or_else(|err| panic!("{}", err.to_string()));
        println!("New Game Started!  It is {}'s turn", players.first().name);
        game.prompt();
        game
    }

    fn reinit(players: Vec<Player>, count: usize) -> Self {
        let game = Game::new(players, count).unwrap_or_else(|err| panic!("{}", err.to_string()));
        let pres: &Player = players.first();
        let scum: &Player = players.last();
        println!("President {}, please offer Scum {} one of your cards.",
                 pres.name, scum.name);
        game.prompt();
        game
    }

    fn new(players: Vec<Player>) -> Result<Self, GameConfigError> {
        let hands = card::deal(players_len()).map_err(|e| {
            match e {
                DealError::ZeroHands => GameConfigError::ZeroPlayers,
                DealError::TooManyHands(h) => GameConfigError::TooManyPlayers(h),
            }
        })?;
        Ok(Self {
            players,
            hands,
            current: 0,
            prev: None,
        })
    }

    pub fn play(&mut self, input: Input) -> Event {
        unimplemented!();
        match input {
            Input::Play(set) => {
                //check hand existence for player
                //remove cards
                //check empty hand and set socialrank
                //check endgame
                //otherwise next turn
            }
            Input::Pass => {
                //Check Prev for clear
                //Check endgame
                //otherwise next turn
            }
            Input::Tax(card) => {
                //Check player ranking
                //move card
                //repeat turn
            }
            Input::Status => {
                //print turn, rank, and multiple
            }
            Input::StatusVerbose => {
                //print game number
                //print all players with turn and rankings
                //print card rank, and multiple
            }
        }
    }

    pub fn restart(self) -> GameConfig {
        println!("Game Restarted!");
        self.players.sort_unstable_by_key(|p| p.ranking);
        GameConfig::from_restart(self.players)
    }

    fn prompt(&self) {
        print!("CAPITALISM {}>", self.players[self.current].name);
    }
}
