use std::fmt;
use strum_macros::EnumIter;
use strum_macros::Display;

// Looks like I need Clone and Copy because Suit is in the outer loop when I try to make the deck?
#[derive(Clone, Copy, EnumIter, Display)]
pub enum Suit {
    Diamonds,
    Hearts,
    Spades,
    Clubs,
}

#[derive(Clone, Copy, EnumIter, Display)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Clone, Copy, EnumIter, Display)]
pub enum Exposure {
    FaceUp,
    FaceDown,
}

#[derive(Clone, Copy, EnumIter, Display)]
pub enum Color {
    Blue,
    Green,
    Red,
    Yellow,
}

#[derive(Clone, Copy, EnumIter, Display)]
pub enum Action {
    Skip,
    Reverse,
    DrawTwo,
}

pub trait Card: fmt::Display {
    fn reveal(&mut self);
    fn hide(&mut self);
}

pub struct PlayingCard {
    pub suit: Suit,
    pub rank: Rank,
    pub exposure: Exposure,
}

// Technically probably won't even need this PlayingCard constructor with the factory implemented?
impl PlayingCard {
    fn new(suit: Suit, rank: Rank) -> Self {
        Self {
            suit,
            rank,
            exposure: Exposure::FaceDown,
        }
    }
}

impl Card for PlayingCard {
    fn reveal(&mut self) {
        self.exposure = Exposure::FaceUp;
    }

    fn hide(&mut self) {
        self.exposure = Exposure::FaceDown;
    }
}

impl fmt::Display for PlayingCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {} ({})", self.rank, self.suit, self.exposure)
    }

}

// Maybe not a completely legit Uno Card, but for "demonstration of factory" purposes, this will work for now.
pub struct UnoCard {
    color: Color,
    action: Action,
    exposure: Exposure,
}

// Technically probably won't even need this UnoCard constructor with the factory implemented?
impl UnoCard {
    fn new(color: Color, action: Action) -> Self {
        Self {
            color,
            action,
            exposure: Exposure::FaceDown,
        }
    }
}

impl Card for UnoCard {
    fn reveal(&mut self) {
        self.exposure = Exposure::FaceUp;
    }

    fn hide(&mut self) {
        self.exposure = Exposure::FaceDown;
    }
}

impl fmt::Display for UnoCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} ({})", self.color, self.action, self.exposure)
    }

}

pub struct CardFactory;

impl CardFactory {
    // pub fn build_playing_card(suit: Suit, rank: Rank) -> PlayingCard {
    //     PlayingCard::new(suit, rank)
    // }

    pub fn build_playing_card(&self, suit: Suit, rank: Rank) -> Box<dyn Card> {
        Box::new(PlayingCard {
            suit,
            rank,
            exposure: Exposure::FaceDown,
        })
    }

    // pub fn build_uno_card(color: Color, action: Action) -> UnoCard {
    //     UnoCard::new(color, action)
    // }

    pub fn build_uno_card(&self, color: Color, action: Action) -> Box<dyn Card> {
        Box::new(UnoCard {
            color,
            action,
            exposure: Exposure::FaceDown,
        })
    }
}