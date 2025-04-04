use std::fmt;
use std::collections::VecDeque;
use strum::IntoEnumIterator;
use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::card::CardFactory;
use crate::card::Card;
use crate::card::Suit;
use crate::card::Rank;

pub fn fifty_two_playing_card_deck(cards: &mut VecDeque<Box<dyn Card>>) {
    let factory = CardFactory;

    for suit in Suit::iter() {
        for rank in Rank::iter() {
            let card = factory.build_playing_card(suit, rank);
            cards.push_back(card);
        }
    }
}

pub trait Deck: fmt::Display {
    fn shuffle(&mut self);
    fn deal(&mut self) -> Option<Box<dyn Card>>;
    fn reset(&mut self);
}

pub struct PlayingCardDeck {
    pub cards: VecDeque<Box<dyn Card>>,
}

impl PlayingCardDeck {
    fn new() -> Self {
        let mut cards = VecDeque::new();
        fifty_two_playing_card_deck(&mut cards);
        Self {
            cards
        }
    }
}

impl Deck for PlayingCardDeck {
    fn shuffle(&mut self) {
        self.cards.make_contiguous().shuffle(&mut thread_rng());
    }

    fn deal(&mut self) -> Option<Box<dyn Card>> {
        self.cards.pop_back()
    }

    fn reset(&mut self) {
        self.cards.clear();
        fifty_two_playing_card_deck(&mut self.cards);
    }
}

impl fmt::Display for PlayingCardDeck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for card in &self.cards {
            write!(f, "{}\n", card)?;
        }
        Ok(())
    }
}

pub struct DeckFactory;

impl DeckFactory {
    pub fn build_playing_card_deck(&self) -> Box<dyn Deck> {
        Box::new(PlayingCardDeck::new())
    }
}