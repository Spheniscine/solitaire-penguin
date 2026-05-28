use dioxus::prelude::*;

use crate::{components::rem, game::{GameState, GameVariant, ScreenState}};

#[component]
fn Choice(
    name: String,
    description: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div {
            style: "text-align: center; border: 0.5rem solid #00B163; border-radius: 2rem; padding: 2rem; margin-top: 7rem;",
            onclick,
            {name},
            br {},
            div {
                font_size: rem(3.5),
                {description},
            }
        }
    }
}

#[component]
pub fn NewGame(game_state: Signal<GameState>) -> Element {
    rsx! {
        div {
            font_size: rem(6.),
            text_align: "center",
            color: "#fff",
            margin: rem(4.),

            p {
                margin_top: rem(10.),

                "Welcome to Penguin Solitaire!"
                br {}
                "Please choose a variant:",
            }

            Choice {
                name: "Tuxedo",
                description: "Recommended for beginners. All cards start in the tableau.",
                onclick: move |_| game_state.write().new_game_with_variant(GameVariant::Tuxedo),
            }

            Choice {
                name: "Original",
                description: "The original game rules. The “beak” card determines which rank to start from, and ranks “wrap around”.",
                onclick: move |_| game_state.write().new_game_with_variant(GameVariant::Original),
            }

            if !game_state.read().new_player {
                button {
                    margin_top: rem(10.),
                    onclick: move |_| game_state.write().screen_state = ScreenState::Game,
                    "Cancel",
                }
            }
            
        }
        
    }
}