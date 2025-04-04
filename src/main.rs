mod card;
mod deck;

use crate::card::CardFactory;
use crate::card::Suit;
use crate::card::Rank;
use crate::card::Color;
use crate::card::Action;
use crate::deck::DeckFactory;

fn main() {
    println!("Hello, world!");

    let card_factory = CardFactory;

    let mut playing_card = card_factory.build_playing_card(Suit::Diamonds, Rank::Ace);
    let mut uno_card = card_factory.build_uno_card(Color::Blue, Action::Skip);
    println!("Playing Card: {}.", playing_card);
    println!("Uno Card: {}.", uno_card);

    let deck_factory = DeckFactory;
    let mut deck = deck_factory.build_playing_card_deck();
    for _ in 0..32 {
        let mut card = deck.deal();
        match card {
            Some(card) => println!("Card: {}", card),
            None => println!("No card to deal"),
        }
    }
    println!("Deck #1");
    println!("{}", deck);
    deck.reset();
    println!("Deck #2");
    println!("{}", deck);
    println!("Deck #2 Shuffled");
    deck.shuffle();
    println!("{}", deck);
}
