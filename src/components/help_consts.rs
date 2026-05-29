#![allow(dead_code)]

use crate::game::{Card, GameState};
use rand_core::SeedableRng;

type CheapRng = rand_xorshift::XorShiftRng;

fn tutorial_deck() -> Vec<Card> {
    let mut rng = CheapRng::seed_from_u64(0);
    GameState::new_deal(&mut rng, false)
}

use crate::game::Suit::*;
pub const TUTORIAL_DECK: &[Card] = &[Card { rank: 3, suit: Diamonds }, Card { rank: 10, suit: Diamonds }, Card { rank: 13, suit: Hearts }, Card { rank: 4, suit: Spades }, Card { rank: 12, suit: Hearts }, Card { rank: 6, suit: Diamonds }, Card { rank: 3, suit: Hearts }, Card { rank: 9, suit: Diamonds }, Card { rank: 6, suit: Spades }, Card { rank: 3, suit: Spades }, Card { rank: 8, suit: Hearts }, Card { rank: 7, suit: Spades }, Card { rank: 1, suit: Diamonds }, Card { rank: 1, suit: Hearts }, Card { rank: 9, suit: Spades }, Card { rank: 7, suit: Hearts }, Card { rank: 7, suit: Diamonds }, Card { rank: 10, suit: Clubs }, Card { rank: 5, suit: Diamonds }, Card { rank: 2, suit: Hearts }, Card { rank: 4, suit: Diamonds }, Card { rank: 13, suit: Diamonds }, Card { rank: 2, suit: Spades }, Card { rank: 8, suit: Diamonds }, Card { rank: 7, suit: Clubs }, Card { rank: 13, suit: Clubs }, Card { rank: 9, suit: Clubs }, Card { rank: 11, suit: Hearts }, Card { rank: 4, suit: Clubs }, Card { rank: 2, suit: Clubs }, Card { rank: 6, suit: Hearts }, Card { rank: 12, suit: Spades }, Card { rank: 10, suit: Spades }, Card { rank: 6, suit: Clubs }, Card { rank: 10, suit: Hearts }, Card { rank: 8, suit: Clubs }, Card { rank: 12, suit: Clubs }, Card { rank: 11, suit: Clubs }, Card { rank: 13, suit: Spades }, Card { rank: 8, suit: Spades }, Card { rank: 11, suit: Diamonds }, Card { rank: 12, suit: Diamonds }, Card { rank: 9, suit: Hearts }, Card { rank: 5, suit: Clubs }, Card { rank: 2, suit: Diamonds }, Card { rank: 5, suit: Spades }, Card { rank: 3, suit: Clubs }, Card { rank: 4, suit: Hearts }, Card { rank: 5, suit: Hearts }, Card { rank: 1, suit: Clubs }, Card { rank: 11, suit: Spades }, Card { rank: 1, suit: Spades }];
