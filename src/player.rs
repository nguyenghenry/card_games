use std::fmt;
use std::collections::VecDeque;
use crate::card::Card;

pub trait Player: fmt::Display {
    fn add_card_to_hand(&mut self, card: Box<dyn Card>);
    fn reveal_hand(&mut self);
}

pub struct HoldEmPlayer {
    pub points: i32,
    pub hand: VecDeque<Box<dyn Card>>,
}

impl HoldEmPlayer {
    fn new() -> Self {
        let mut hand = VecDeque::new();
        Self {
            points: 0,
            hand,
        }
    }
}

impl Player for HoldEmPlayer {
    fn add_card_to_hand(&mut self, card: Box<dyn Card>) {
        self.hand.push_back(card);
    }

    fn reveal_hand(&mut self) {
        // For loop through hand and flipping all the cards face up?
    }
}

impl fmt::Display for HoldEmPlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Player's current hand is:\n")?;
        for card in &self.hand {
            write!(f, "{}\n", card)?;
        }
        Ok(())
    }
}

pub struct PlayerFactory;

impl PlayerFactory {
    pub fn build_holdem_player(&self) -> Box<dyn Player> {
        Box::new(HoldEmPlayer::new())
    }
}