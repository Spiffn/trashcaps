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
