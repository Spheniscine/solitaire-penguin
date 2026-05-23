use std::time::Duration;

use rand::{Rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::game::{Card, DECK_SIZE, NUM_SUITS, RANK_MAX, RANK_MIN, RANKS, Suit};

pub const NUM_FOUNDATIONS: usize = NUM_SUITS;
pub const NUM_FREECELLS: usize = 7;
pub const NUM_TABLEAU_DEPOTS: usize = 7;

pub const NUM_DEPOTS: usize = NUM_FOUNDATIONS + NUM_FREECELLS + NUM_TABLEAU_DEPOTS;

pub const FOUNDATION_OFFSET: usize = 0;
pub const FREECELL_OFFSET: usize = NUM_FOUNDATIONS;
pub const TABLEAU_OFFSET: usize = FREECELL_OFFSET + NUM_FREECELLS;

pub const ANIMATION_DURATION: Duration = Duration::from_millis(200);

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum DepotRole {
    Foundation, FreeCell, Tableau
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
            (DepotRole::Tableau, self.0 - TABLEAU_OFFSET)
        }
    }

    pub fn role(&self) -> DepotRole {
        self.role_and_subindex().0
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct BoardPos {
    pub depot_index: usize,
    pub card_index: usize,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Action {
    Move(Vec<Card>, BoardPos, BoardPos),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Board {
    pub depots: Vec<Vec<Card>>,
    pub beak: Card,
    pub selected: Option<BoardPos>,
    pub actions: Vec<Action>,
}

impl Board {
    pub fn from_deal(deal: &Vec<Card>) -> Self {
        assert_eq!(deal.len(), DECK_SIZE);
        let mut depots = vec![vec![]; NUM_DEPOTS];
        let mut column_ite = std::iter::repeat(0..NUM_TABLEAU_DEPOTS).flatten();
        let mut foundation_ite = 0..NUM_FOUNDATIONS;

        let beak = deal[0];
        for &card in deal {
            if card != beak && card.rank == beak.rank {
                depots[foundation_ite.next().unwrap() + FOUNDATION_OFFSET].push(card);
            } else {
                depots[column_ite.next().unwrap() + TABLEAU_OFFSET].push(card);
            }
        }

        Board {
            depots, beak, selected: None, actions: vec![],
        }
    }

    pub fn foundation_rank(&self) -> u8 {
        self.beak.rank
    }

    pub fn column_head_rank(&self) -> u8 {
        if self.foundation_rank() == RANK_MIN { RANK_MAX } else {self.foundation_rank() - 1}
    }

    pub fn do_move(&mut self, pos1: BoardPos, pos2: BoardPos) {
        self.selected = None;
        let cards = self.depots[pos1.depot_index].drain(pos1.card_index ..).collect();
        self.actions.push(
            Action::Move(cards, pos1, pos2)
        );
    }

    pub fn finalize_actions(&mut self) {
        for act in self.actions.drain(..) {
            match act {
                Action::Move(cards, pos1, pos2) => {
                    self.depots[pos2.depot_index].extend(cards);
                },
            }
        }
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
        let mut deal = Self::new_deal(&mut rand::rng());
        let random_beak = false;

        if !random_beak {
            let beak = Card { rank: 1, suit: Suit::Spades };
            let i = deal.iter().position(|&card| card == beak).expect("1S not found in deck, should be full deck");
            deal.swap(0, i);
        }

        Self {
            board: Board::from_deal(&deal),
            deal,
            num_wins: 0,
            random_beak,
        }
    }

    pub fn can_select(&mut self, pos: BoardPos) -> bool {
        let depot = pos.depot_index;
        let ord = pos.card_index;

        // todo: rules
        ord < self.board.depots[depot].len()
    }

    pub fn can_move(&mut self, pos1: BoardPos, pos2: BoardPos) -> bool {
        let max_tableau_test: usize = 18;

        if pos1.depot_index == pos2.depot_index { return false; }
        let num_moved = self.board.depots[pos1.depot_index].len() - pos1.card_index;
        if pos2.card_index != self.board.depots[pos2.depot_index].len() { return false; }
        // todo: rules
        match DepotIndex(pos2.depot_index).role() {
            DepotRole::Foundation => {
                num_moved == 1
            },
            DepotRole::FreeCell => {
                num_moved == 1
            },
            DepotRole::Tableau => {
                num_moved + self.board.depots[pos2.depot_index].len() <= max_tableau_test
            },
        }
    }

    pub fn is_busy(&self) -> bool {
        self.is_acting()
    }

    pub fn is_acting(&self) -> bool {
        !self.board.actions.is_empty()
    }

    pub fn onclick(&mut self, pos: BoardPos) {
        if self.is_busy() { return; }
        let depot = pos.depot_index;
        let ord = pos.card_index;

        if let Some(src) = self.board.selected {
            if pos == src { self.board.selected = None; }

            let dest = BoardPos { depot_index: pos.depot_index, card_index: pos.card_index.wrapping_add(1) };
            if !self.can_move(src, dest) { return; }
            self.board.do_move(src, dest);
        } else {
            if ord < self.board.depots[depot].len() {
                self.board.selected = Some(pos);
            }
        }
    }

    pub fn finalize_actions(&mut self) {
        self.board.finalize_actions();
    }
}