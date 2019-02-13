use std::fmt;

pub trait GameEvent {
  fn is_over(&self) -> bool;
}

pub trait Repl {
  type Event: GameEvent + fmt::Display;
  type Input: fmt::Display;

  fn events(&self) -> &[Event];
  fn input(&self) -> Input;
}

pub trait Game: Iterator<Item=Self::Turn> {
  type Turn: Repl;
  type StatOpt;
  type StatOut: fmt::Display;

  fn stat(&self, opt: StatOpt) -> StatOut;
}
