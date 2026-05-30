use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::{BoardComponent, CardText, TUTORIAL_BEAK, TUTORIAL_DECK, rem}, game::{Board, ColorMode, GameState, GameVariant}};

#[component]
fn Emph(children: Element) -> Element {
    rsx! {
        strong {
            color: "#ff0",
            {children}
        }
    }
}

#[component]
pub fn Help(game_state: Signal<GameState>) -> Element {
    let st = game_state.read();
    let variant = GameVariant::Original; //st.variant;
    // let skin = st.skin;
    let mut skin = st.skin; skin.suits = crate::game::SuitSkin::Shapes;
    let board = Board::from_deal(TUTORIAL_DECK, variant);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; font-size: 4rem; color: #fff; padding: 4rem;",

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
                        skin,
                    }
                }
            }

            div {
                text_align: "center",
                p { 
                    margin_top: "1em",
                    "The ", Emph {"tableau"}, " consists of 7 columns.", 
                }
                if variant == GameVariant::Tuxedo {
                    p {
                        margin_top: "1em",
                        "A standard 52-card deck is shuffled and dealt into the layout shown – 7 rows each for 7 columns, then one extra card to the 1",
                        sup{"st"},", 4",sup{"th"}," and 7",sup{"th"}," columns."
                    }
                        
                } else if variant == GameVariant::Original {
                    p {
                        margin_top: "1em",
                        "A standard 52-card deck is shuffled and dealt. The first card dealt to the back of the first column is called the ",Emph{"beak"},". (",
                        CardText { card: TUTORIAL_BEAK, skin, color_mode: ColorMode::Light },
                        " in this tutorial)"
                    }
                    p {
                        margin_top: "1em",
                        "The rest of the cards are then dealt evenly to the columns, but the other three cards of the same rank as the beak will go to the foundations instead."
                    }
                }
                
            }
        }
        
    }
}