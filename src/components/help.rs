use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::{BoardComponent, TUTORIAL_DECK, rem}, game::{Board, GameState}};

#[component]
pub fn Help(game_state: Signal<GameState>) -> Element {
    let st = game_state.read();
    let board = Board::from_deal(TUTORIAL_DECK, st.variant);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center;",

            div {
                overflow: "hidden",
                border: "0.5rem solid #fff",
                position: "relative",
                width: rem(90.),
                height: rem(85.),
                
                div {
                    position: "absolute",
                    width: rem(90. / 0.9),
                    top: 0,
                    left: 0,
                    transform: "scale(90%)",
                    transform_origin: "top left",
                    BoardComponent { 
                        position: Vec2::ZERO,
                        board,
                        skin: st.skin,
                    }
                }

                
                
            }
        }
        
    }
}