use std::{ops::Range, time::Duration};

use rand::{Rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};
use strum::IntoEnumIterator;

use crate::{components::LocalStorage, game::{Card, ColorSkin, DECK_SIZE, NUM_RANKS, NUM_SUITS, RANK_MAX, RANK_MIN, RANKS, RankSkin, SettingsState, Skin, Suit, SuitSkin}};

pub const NUM_FOUNDATIONS: usize = NUM_SUITS;
pub const NUM_FREECELLS: usize = 7;
pub const NUM_TABLEAU_DEPOTS: usize = 7;

pub const NUM_DEPOTS: usize = NUM_FOUNDATIONS + NUM_FREECELLS + NUM_TABLEAU_DEPOTS;

pub const FOUNDATION_OFFSET: usize = 0;
pub const FREECELL_OFFSET: usize = NUM_FOUNDATIONS;
pub const TABLEAU_OFFSET: usize = FREECELL_OFFSET + NUM_FREECELLS;

pub const FOUNDATIONS: Range<usize> = FOUNDATION_OFFSET .. FREECELL_OFFSET;
pub const FREECELLS: Range<usize> = FREECELL_OFFSET .. TABLEAU_OFFSET;
pub const TABLEAU_COLUMNS: Range<usize> = TABLEAU_OFFSET .. NUM_DEPOTS;

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

#[derive(Copy, Clone, Serialize_tuple, Deserialize_tuple, Debug, PartialEq, Eq)]
pub struct BoardPos {
    pub depot_index: usize,
    pub card_index: usize,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum AnimationAct {
    Move(Vec<Card>, BoardPos, BoardPos),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Board {
    pub depots: Vec<Vec<Card>>,
    pub beak: Card,
    pub selected: Option<BoardPos>,
    pub animation_acts: Vec<AnimationAct>,
}

pub fn rank_under_wrap(rank: u8) -> u8 {
    rank_under_with_variant(rank, GameVariant::Original)
}

pub fn rank_under_with_variant(rank: u8, variant: GameVariant) -> u8 {
    if variant == GameVariant::Original && rank == RANK_MIN { RANK_MAX } else { rank - 1 }
}

impl Board {
    pub fn empty() -> Self {
        Self {
            depots: vec![],
            beak: Card { rank: 1, suit: Suit::Spades },
            selected: None,
            animation_acts: vec![],
        }
    }

    pub fn from_deal(deal: &[Card], variant: GameVariant) -> Self {
        assert_eq!(deal.len(), DECK_SIZE);
        let mut depots = vec![vec![]; NUM_DEPOTS];
        let beak = if variant == GameVariant::Original {deal[0]} else {Card { rank: 1, suit: Suit::Spades }};

        match variant {
            GameVariant::Tuxedo => {
                let mut deal = deal;
                for _ in 0..7 {
                    for i in TABLEAU_COLUMNS {
                        let card = *deal.split_off_last().unwrap();
                        depots[i].push(card);
                    }
                }
                for i in [0, 3, 6] {
                    let card = *deal.split_off_last().unwrap();
                    depots[TABLEAU_OFFSET + i].push(card);
                }
            },
            GameVariant::Original => {
                let mut column_ite = std::iter::repeat(TABLEAU_COLUMNS).flatten();
                let mut foundation_ite = FOUNDATIONS;

                for &card in deal {
                    if card != beak && card.rank == beak.rank {
                        depots[foundation_ite.next().unwrap()].push(card);
                    } else {
                        depots[column_ite.next().unwrap()].push(card);
                    }
                }
            },
        }

        Board {
            depots, beak, selected: None, animation_acts: vec![],
        }
    }

    pub fn foundation_rank(&self) -> u8 {
        self.beak.rank
    }

    pub fn tableau_head_rank(&self) -> u8 {
        rank_under_wrap(self.foundation_rank())
    }

    pub fn do_move(&mut self, pos1: BoardPos, pos2: BoardPos) {
        self.selected = None;
        let cards = self.depots[pos1.depot_index].drain(pos1.card_index ..).collect();
        self.animation_acts.push(
            AnimationAct::Move(cards, pos1, pos2)
        );
    }

    pub fn advance_actions(&mut self) {
        for act in self.animation_acts.drain(..) {
            match act {
                AnimationAct::Move(cards, _pos1, pos2) => {
                    self.depots[pos2.depot_index].extend(cards);
                },
            }
        }
    }
}

pub type AnimationKey = u16;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ActionRecord {
    pos1: BoardPos, pos2: BoardPos, auto: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScreenState {
    Game, Settings, NewGame,
}

#[derive(Copy, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, strum_macros::Display)]
pub enum GameVariant {
    Tuxedo, 
    #[default] Original // default to be backward compatible with saves
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct GameState {
    pub board: Board,
    pub deal: Vec<Card>,
    #[serde(skip)]
    pub animation_key: AnimationKey, // used for syncing and to provide animator components with cycling keys
    pub history: Vec<ActionRecord>,
    pub already_won: bool,
    pub num_wins: i32,

    #[serde(default)]
    pub variant: GameVariant,
    pub screen_state: ScreenState,
    #[serde(skip)]
    pub new_player: bool,

    pub allow_undo: bool,
    // pub random_beak: bool, // Remove fixed-beak option (issue #1)
    pub auto_play: bool,
    pub skin: Skin,
}

impl GameState {
    pub fn new_deal(rng: &mut impl Rng, random_beak: bool) -> Vec<Card> {
        let mut deck = Vec::with_capacity(DECK_SIZE);
        for rank in RANKS {
            for suit in Suit::iter() {
                deck.push(Card { rank, suit });
            }
        }

        deck.shuffle(rng);
        if !random_beak {
            let beak = Card { rank: 1, suit: Suit::Spades };
            let i = deck.iter().position(|&card| card == beak).expect("1S not found in deck, should be full deck");
            deck.swap(0, i);
        }

        deck
    }
    pub fn init() -> Self {
        let random_beak = true;
        let deal = Self::new_deal(&mut rand::rng(), random_beak);
        let variant = GameVariant::Tuxedo;

        let skin = Skin { 
            ranks: RankSkin::Numbers, 
            suits: SuitSkin::Animals, 
            colors: ColorSkin::FourColor,
        };

        let res = Self {
            board: Board::empty(),
            deal,
            animation_key: 0,
            history: vec![],
            num_wins: 0,
            already_won: false,

            variant,
            screen_state: ScreenState::NewGame,
            new_player: true,

            allow_undo: true,
            auto_play: true,
            skin,
        };

        res
    }

    pub fn can_stack(&self, back: Card, front: Card) -> bool {
        back.suit == front.suit && front.rank == rank_under_with_variant(back.rank, self.variant)
    }

    pub fn can_sort(&self, back: Card, front: Card) -> bool {
        back.suit == front.suit && back.rank == rank_under_with_variant(front.rank, self.variant)
    }

    pub fn can_select(&mut self, pos: BoardPos) -> bool {
        let depot = pos.depot_index;
        let ord = pos.card_index;

        if ord >= self.board.depots[depot].len() {
            return false;
        }
        let slice = &self.board.depots[depot][ord..];

        match DepotIndex(depot).role() {
            DepotRole::Foundation => false,
            DepotRole::FreeCell => { slice.len() <= 1 },
            DepotRole::Tableau => {
                slice.windows(2).all(|w| self.can_stack(w[0], w[1]))
            },
        }
    }

    pub fn can_move(&mut self, pos1: BoardPos, pos2: BoardPos) -> bool {
        //let max_tableau_test: usize = 18;

        if pos1.depot_index == pos2.depot_index { return false; }
        let depot1 = &self.board.depots[pos1.depot_index];
        let depot2 = &self.board.depots[pos2.depot_index];
        let num_moved = depot1.len() - pos1.card_index;
        if pos2.card_index != depot2.len() { return false; }
       
        let card = depot1[pos1.card_index];
        match DepotIndex(pos2.depot_index).role() {
            DepotRole::Foundation => {
                num_moved == 1 && if let Some(&c) = depot2.last() {
                    self.can_sort(c, card)
                } else {
                    self.board.foundation_rank() == card.rank
                }
            },
            DepotRole::FreeCell => {
                depot2.is_empty() && num_moved == 1
            },
            DepotRole::Tableau => {
                if let Some(&c) = depot2.last() {
                    self.can_stack(c, card)
                } else {
                    self.board.tableau_head_rank() == card.rank
                }
            },
        }
    }

    pub fn check_auto_moves(&mut self) {
        if self.is_busy() { return; }
        if !self.auto_play { return; }

        for depot in FREECELL_OFFSET .. NUM_DEPOTS {
            if let Some(_) = self.board.depots[depot].last() {
                let src = BoardPos { depot_index: depot, card_index: self.board.depots[depot].len() - 1 };
                if self.try_sort(src, true) { return; }
            }
        }
    }

    fn try_sort(&mut self, src: BoardPos, auto: bool) -> bool {
        for dest in FOUNDATIONS {
            let dest = BoardPos { depot_index: dest, card_index: self.board.depots[dest].len()};
            if self.can_move(src, dest) {
                self.board.do_move(src, dest);
                self.history.push(ActionRecord { pos1: src, pos2: dest, auto });
                return true;
            }
        }
        false
    }

    pub fn is_busy(&self) -> bool {
        self.is_acting()
    }

    pub fn is_acting(&self) -> bool {
        !self.board.animation_acts.is_empty()
    }

    pub fn is_won(&self) -> bool {
        FOUNDATIONS.clone().all(|i| {
            self.board.depots[i].len() == NUM_RANKS
        })
    }

    pub fn restart(&mut self) {
        if self.history.is_empty() || !self.undo_possible() { return; }
        self.board = Board::from_deal(&self.deal, self.variant);
        self.history.clear();
        LocalStorage.save_game_state(&self);
    }

    pub fn new_game(&mut self) {
        self.screen_state = ScreenState::NewGame;
    }

    pub fn new_game_with_variant(&mut self, variant: GameVariant) {
        let deal = Self::new_deal(&mut rand::rng(), true);
        self.board = Board::from_deal(&deal, variant);
        self.variant = variant;
        self.deal = deal;
        self.history.clear();
        self.already_won = false;
        self.new_player = false;
        self.screen_state = ScreenState::Game;
        LocalStorage.save_game_state(&self);
    }

    pub fn onclick(&mut self, pos: BoardPos) {
        if self.is_busy() { return; }

        if let Some(src) = self.board.selected {
            if pos == src { 
                self.board.selected = None; 
                return;
            }
            if src.depot_index == pos.depot_index && self.can_select(pos) {
                self.board.selected = Some(pos);
                return;
            }

            let dest = BoardPos { depot_index: pos.depot_index, card_index: pos.card_index.wrapping_add(1) };
            if !self.can_move(src, dest) { return; }
            self.board.do_move(src, dest);
            self.history.push(ActionRecord { pos1: src, pos2: dest, auto: false });
        } else {
            if self.can_select(pos) {
                self.board.selected = Some(pos);
            }
        }
    }

    pub fn ondoubleclick(&mut self, pos: BoardPos) {
        if self.is_busy() { return; }
        let depot = pos.depot_index;
        if DepotIndex(depot).role() == DepotRole::Foundation { return; }
        for dest in [FOUNDATIONS, TABLEAU_COLUMNS, FREECELLS].into_iter().flatten() {
            let dest = BoardPos { depot_index: dest, card_index: self.board.depots[dest].len()};
            if self.can_move(pos, dest) {
                self.board.do_move(pos, dest);
                self.history.push(ActionRecord { pos1: pos, pos2: dest, auto: false });
                return;
            }
        }
    }

    pub fn advance_animations(&mut self, key: AnimationKey) {
        if key != self.animation_key { return; }
        self.animation_key = self.animation_key.wrapping_add(1);
        
        self.board.advance_actions();

        if self.is_won() {
            if !self.already_won {
                self.num_wins += 1;
                self.already_won = true;
            }
        } else {
            self.check_auto_moves();
        }

        if !self.is_busy() { LocalStorage.save_game_state(&self); }
    }

    pub fn undo_possible(&self) -> bool {
        self.allow_undo && !self.history.is_empty()
    }

    pub fn undo(&mut self) {
        if self.is_busy() || !self.undo_possible() { return; }
        while let Some(rec) = self.history.pop() {
            self.board.do_move(rec.pos2, rec.pos1);
            self.board.advance_actions(); // no animation, as repeated card moves on same card causes problems
            if !rec.auto { break; }
        }
        LocalStorage.save_game_state(&self);
    }

    pub fn new_settings_state(&self) -> SettingsState {
        SettingsState {
            allow_undo: self.allow_undo,
            auto_play: self.auto_play,
            skin: self.skin,
        }
    }

    pub fn apply_settings(&mut self, settings: &SettingsState){
        self.allow_undo = settings.allow_undo;
        self.auto_play = settings.auto_play;
        self.skin = settings.skin;
        LocalStorage.save_game_state(&self);
    }
}