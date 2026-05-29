use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::{BoardComponent, TUTORIAL_DECK}, game::{Board, GameState}};

#[component]
pub fn Help(game_state: Signal<GameState>) -> Element {
    let st = game_state.read();
    let board = Board::from_deal(TUTORIAL_DECK, st.variant);

    rsx! {
        BoardComponent { 
            position: Vec2::ZERO,
            board,
            skin: st.skin,
        }
    }
}