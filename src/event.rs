#[derive(Debug)]
pub enum GameEvent {
  Start,
  Invalid,
  Play, //regular play
  Skip,
  Complete, //that is, completion
  Bomb,
  Finish, //player's play ends them with no cards
  RoundFinish,               //all players have finished
  StartPick,
  Pick, //player has picked a deck
  StartExchange,
  Offer, //lower-status -> higher-status
  Exchange, //higher-status -> lower-status
}
