use rand::{Rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::game::{Card, DECK_SIZE, NUM_SUITS, RANK_MAX, RANK_MIN, RANKS, Suit};

pub const NUM_FOUNDATIONS: usize = NUM_SUITS;
pub const NUM_FREECELLS: usize = 7;
pub const NUM_COLUMNS: usize = 7;

pub const NUM_DEPOTS: usize = NUM_FOUNDATIONS + NUM_FREECELLS + NUM_COLUMNS;

pub const FOUNDATION_OFFSET: usize = 0;
pub const FREECELL_OFFSET: usize = NUM_FOUNDATIONS;
pub const COLUMN_OFFSET: usize = FREECELL_OFFSET + NUM_FREECELLS;

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum DepotRole {
    Foundation, FreeCell, Column
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct DepotIndex(pub usize);
impl DepotIndex {
    pub fn role_and_subindex(&self) -> (DepotRole, usize) {
        if self.0 < NUM_FOUNDATIONS {
            (DepotRole::Foundation, self.0 - FOUNDATION_OFFSET)
        } else if self.0 < FREECELL_OFFSET + NUM_FREECELLS {
            (DepotRole::FreeCell, self.0 - FREECELL_OFFSET)
        } else {
            (DepotRole::Column, self.0 - COLUMN_OFFSET)
        }
    }

    pub fn role(&self) -> DepotRole {
        self.role_and_subindex().0
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Board {
    pub depots: Vec<Vec<Card>>,
    pub beak: Card,
}

impl Board {
    pub fn from_deal(mut deal: Vec<Card>, random_beak: bool) -> Self {
        assert_eq!(deal.len(), DECK_SIZE);
        let mut depots = vec![vec![]; NUM_DEPOTS];
        let mut column_ite = std::iter::repeat(0..NUM_COLUMNS).flatten();
        let mut foundation_ite = 0..NUM_FOUNDATIONS;

        if !random_beak {
            let beak = Card { rank: 1, suit: Suit::Spades };
            let i = deal.iter().position(|&card| card == beak).expect("1S not found in deck, should be full deck");
            deal.swap(0, i);
        }

        let beak = deal[0];
        for &card in &deal {
            if card != beak && card.rank == beak.rank {
                depots[foundation_ite.next().unwrap() + FOUNDATION_OFFSET].push(card);
            } else {
                depots[column_ite.next().unwrap() + COLUMN_OFFSET].push(card);
            }
        }

        Board {
            depots, beak,
        }
    }

    pub fn foundation_rank(&self) -> u8 {
        self.beak.rank
    }

    pub fn column_head_rank(&self) -> u8 {
        if self.foundation_rank() == RANK_MIN { RANK_MAX } else {self.foundation_rank() - 1}
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct GameState {
    pub board: Board,
    pub deal: Vec<Card>,
    pub num_wins: i32,
    pub random_beak: bool,
}

impl GameState {
    pub fn new_deal(rng: &mut impl Rng) -> Vec<Card> {
        let mut deck = Vec::with_capacity(DECK_SIZE);
        for rank in RANKS {
            for suit in Suit::iter() {
                deck.push(Card { rank, suit });
            }
        }

        deck.shuffle(rng);
        deck
    }
    pub fn init() -> Self {
        let deal = Self::new_deal(&mut rand::rng());
        let random_beak = false;

        Self {
            board: Board::from_deal(deal.clone(), random_beak),
            deal,
            num_wins: 0,
            random_beak,
        }
    }
}