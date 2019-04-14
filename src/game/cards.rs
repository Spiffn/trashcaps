use std::str::FromStr;
use std::convert::From;
use std::fmt::Display;

//[STRUCT|ENUMS]

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    Two, //or bomb, a unique name is not specified in pagat.com
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

#[derive(Debug, Default)]
pub struct Hand {
    is_ordered: bool,
    cards: Vec<Card>,
}

pub enum DealErrors {
    ZeroHands,
    TooManyHands(usize),
}

//[IMPLS]

//SUIT
//RANK
impl FromStr for Rank {
    type String;
    fn from_str(value: &str) -> Result<Self, Self::Error> {
        match value {
            "1"  => Ok(Rank::Ace),
            "2"  => Ok(Rank::Two),
            "3"  => Ok(Rank::Three),
            "4"  => Ok(Rank::Four),
            "5"  => Ok(Rank::Five),
            "6"  => Ok(Rank::Six),
            "7"  => Ok(Rank::Seven),
            "8"  => Ok(Rank::Eight),
            "9"  => Ok(Rank::Nine),
            "10" => Ok(Rank::Ten),
            "11" => Ok(Rank::Jack),
            "12" => Ok(Rank::Queen),
            "13" => Ok(Rank::King),
            _    => Err(format!("Invalid token {}", value)),
        }
    }
}

//HAND
impl Hand {
    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
        self.is_ordered = false;
    }

    pub fn has(&self, card: &Card) -> bool {
        if !self.is_ordered {
            self.order();
        }
        self.cards.binary_search()
    }

    pub fn has_all(&self, cards: &[Card]) -> bool {
        cards.iter().all(self.has)
    }

    fn order(&mut self) {
        self.cards.sort_unstable();
        self.is_ordered = true;
    }
}

impl Display for Hand {
    use std::fmt;
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!("{:?}", self.cards)
    }
}

impl From<Vec<Card>> for Hand {
    fn from(cards: Vec<Card>) -> Self {
        Self(cards)
    }
}

//[FUNCTIONS] 
pub fn deal(hands: usize) -> Result<Vec<Hand>, DealErrors> {
    const SUIT_NUM: usize = 4;
    const RANK_NUM: usize = 13;
    const DECK_SIZE: usize = 52;
    const SUITS: [Suit; SUIT_NUM] = [Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds];
    const RANKS: [Rank: RANK_NUM] = [
                Rank::Two,
                Rank::Ace,
                Rank::King,
                Rank::Queen,
                Rank::Jack,
                Rank::Ten,
                Rank::Nine,
                Rank::Eight,
                Rank::Seven,
                Rank::Six,
                Rank::Five,
                Rank::Four,
                Rank::Three,
    ];

    if hands == 0 {
        return Err(DealErrors::ZeroHands);
    }

    let hand_size = DECK_SIZE / hands;
    if hand_size == 0 {
        return Err(DealErrors::TooManyHands(hands));
    }
    let mut deck = Vec::with_capacity(DECK_SIZE);
    //populate deck
    (0..SUIT_NUM).for_each(|s_i| {
        (0..RANK_NUM).for_each(|r_i| {
            deck.push(Card {suit: suits[s_i], rank: ranks[r_i] });
        });
    });

    {
        use rand::thread_rng;
        use rand::seq::SliceRandom;

        let mut rng = thread_rng();
        deck.shuffle(&mut rng);
    }

    let mut res = Vec::with_capacity(hands);
    (0..hands)
        .for_each(|_| {
            res.push(self.0.split_off(hand_size).into());
        });

    self.0
        .drain(..)
        .zip((0..hands).cycle())
        .for_each(|(card, hand_i)| {
            res[hand_i].add(card);
        });
    Ok(res)
}
