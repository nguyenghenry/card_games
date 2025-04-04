use std::fmt;
use strum_macros::EnumIter;

// Looks like I need Clone and Copy because Suit is in the outer loop when I try to make the deck?
#[derive(Clone, Copy, EnumIter)]
pub enum Suit {
    Diamonds,
    Hearts,
    Spades,
    Clubs,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suit = match self {
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Clubs => "Clubs",
            Suit::Spades => "Spades",
        };
        write!(f, "{}", suit)
    }
}

#[derive(EnumIter)]
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

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank = match self {
            Rank::Ace => "Ace",
            Rank::Two => "Two",
            Rank::Three => "Three",
            Rank::Four => "Four",
            Rank::Five => "Five",
            Rank::Six => "Six",
            Rank::Seven => "Seven",
            Rank::Eight => "Eight",
            Rank::Nine => "Nine",
            Rank::Ten => "Ten",
            Rank::Jack => "Jack",
            Rank::Queen => "Queen",
            Rank::King => "King",
        };
        write!(f, "{}", rank)
    }
}

#[derive(EnumIter)]
pub enum Exposure {
    FaceUp,
    FaceDown,
}

impl fmt::Display for Exposure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let exposure = match self {
            Exposure::FaceDown => "Face-Down",
            Exposure::FaceUp => "Face-Up",
        };
        write!(f, "{}", exposure)
    }
}

#[derive(EnumIter)]
pub enum Color {
    Blue,
    Green,
    Red,
    Yellow,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color = match self {
            Color::Blue => "Blue",
            Color::Red => "Red",
            Color::Green => "Green",
            Color::Yellow => "Yellow",
        };
        write!(f, "{}", color)
    }
}

#[derive(EnumIter)]
pub enum Action {
    Skip,
    Reverse,
    DrawTwo,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let action = match self {
            Action::Skip => "Skip",
            Action::Reverse => "Reverse",
            Action::DrawTwo => "DrawTwo"
        };
        write!(f, "{}", action)
    }
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