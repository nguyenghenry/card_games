use std::fmt;
use std::collections::VecDeque;
use crate::card::Card;
use crate::deck::Deck;
use crate::deck::DeckFactory;
use crate::player::Player;
use crate::PlayerFactory;

pub struct HoldEm {
    pub community_pile: VecDeque<Box<dyn Card>>,
    pub players_pile: VecDeque<Box<dyn Player>>,
    pub deck: Box <dyn Deck>,
}

impl HoldEm {
    pub fn new() -> Self {
        let mut deck_factory = DeckFactory;
        let mut deck = deck_factory.build_playing_card_deck();
        let mut community_pile = VecDeque::new();
        let mut players_pile = VecDeque::new();
        Self {
            community_pile,
            players_pile,
            deck,
        }
    }

    // Could separate the adding players to the game and dealing to players so we don't make a new player everytime we deal.
    pub fn add_players_to_game(&mut self, num_players: i32) {
        let player_factory = PlayerFactory;
        for _ in 0..num_players {
            let player = player_factory.build_holdem_player();
            self.players_pile.push_back(player);
        }
    }

    pub fn deal_to_players(&mut self) {
        // Make sure previous hands are cleared.
        self.deck.shuffle();
        for mut player in &mut self.players_pile {
            for _ in 0..2 {
                let mut card = self.deck.deal();
                match card {
                    Some(card) => {
                        player.add_card_to_hand(card);
                    },
                    None => println!("No card to deal"),
                }
            }
        }
    }

    pub fn deal_to_flop(&mut self) {
        for _ in 0..3 {
            let mut card = self.deck.deal();
            match card {
                Some(card) => {
                    self.community_pile.push_back(card);
                },
                None => println!("No card to deal"),
            }
        }
    }

    pub fn deal_to_turn(&mut self) {
        let mut card = self.deck.deal();
        match card {
            Some(card) => {
                self.community_pile.push_back(card);
            },
            None => println!("No card to deal"),
        }
    }

    pub fn deal_to_river(&mut self) {
        let mut card = self.deck.deal();
        match card {
            Some(card) => {
                self.community_pile.push_back(card);
            },
            None => println!("No card to deal"),
        }
    }
}


impl fmt::Display for HoldEm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut player_number = 1;
        for player in &self.players_pile {
            write!(f, "Player Number {}\n{}\n", player_number, player)?;
            player_number += 1;
        }
        write!(f, "Rest of the Deck\n{}", self.deck)?;
        Ok(())
    }
}
