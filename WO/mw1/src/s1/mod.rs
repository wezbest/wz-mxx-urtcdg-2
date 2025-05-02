/*
Sectin 1 Work here
*/

#![allow(unused)]

use std::os::unix::thread;

// Rand Package
use rand::{
    prelude::*,
    rng,
    seq::{SliceChooseIter, SliceRandom},
};

use crate::utils::{header, pswg};
use yansi::Paint;

pub fn s1main() {
    test_func1();
}

// Test function
fn test_func1() {
    s1_fn();
}

/////////////// Section 1 Work starts here ///////////////

// Struct with vector
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

// Inherent Implemenation
impl Deck {
    fn new() -> Self {
        // list of suits
        let suits = ["SS", "HH", "AA", "DD"];
        // List of values
        let values = ["Ace_1", "Two_2", "Three_3"];

        // Empty vector of cards
        let mut cards = vec![];

        // Example of reassignment
        // cards = vec![];

        // Double nested for loops - to generate combinations
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards }
    }

    // Shuffle function
    fn shuffle(&mut self) {
        // Create a new random number generator
        let mut rngz = rng();

        println!("Random die roll: {}", rng().random_range(1..=600));
        self.cards.shuffle(&mut rngz);
    }

    // Function to deal out random cards
    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn s1_fn() {
    // Printing the header text
    pswg("Section 1 - Structs and Vectors".to_string());

    let mut deck = Deck::new();

    deck.shuffle();
    // TODO: Add Error handling - If u deal more than 12 cards error handling
    let cards_dealt = deck.deal(2);
    println!("Cards: {:#?}", cards_dealt);
    println!("Total number of cards: {}", cards_dealt.len().yellow());

    println!("Main Deck: {:#?}", deck.green());
    println!("Total number of cards: {}", deck.cards.len().red());
}
