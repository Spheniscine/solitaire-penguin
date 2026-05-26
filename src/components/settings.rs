use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::game::{ColorSkin, GameState, RankSkin, SuitSkin};

#[component]
pub fn Settings(game_state: Signal<GameState>) -> Element {
    let mut gs = game_state;
    let mut ok = move || {
        let _ = gs.write();
        // game_state.write().apply_settings(&state.read());
        // game_state.write().show_settings = false;
    };
    let mut cancel = move || {
        let _ = gs.write();
        // game_state.write().show_settings = false;
    };

    let mut onmounted = async move |e: Event<MountedData>| {
        e.set_focus(true).await;
    };
    let mut onkeydown = move |e: Event<KeyboardData>| {
        let key = e.key();
        match key {
            Key::Enter => {
                ok();
            }
            Key::Escape => {
                cancel();
            }
            _ => {}
        }
    };

    rsx! {
        div {
            id: "settingsDialog",
            tabindex: -1,
            onmounted: onmounted,
            onkeydown: onkeydown,

            p {
                "Auto-play to foundations: "
                input {
                    r#type: "checkbox",
                }
            }

            p {
                "Card style: "
            }

            div {
                margin_left: "5rem",
                p {
                    "Ranks: "
                    select {
                        for x in RankSkin::iter() {
                            option {
                                value: x as usize,
                                "{x}"
                            }
                        }
                    }
                }
                p {
                    "Suits: "
                    select {
                        for x in SuitSkin::iter() {
                            option {
                                value: x as usize,
                                "{x}"
                            }
                        }
                    }
                }
                p {
                    "Color scheme: "
                    select {
                        for x in ColorSkin::iter() {
                            option {
                                value: x as usize,
                                "{x}"
                            }
                        }
                    }
                }
            }

            p {
                line_height: 1,
                span {
                    "Random beak: "
                    input {
                        r#type: "checkbox",
                    }
                }
                br {}
                span {
                    style: "font-size: 3.5rem; line-height: 2rem;",
                    "(does not apply until a new game is started)"
                }
            }

            p {
                button {
                    r#type: "button",
                    onclick: move |_| ok(),
                    "OK"
                }
                " ",
                button {
                    r#type: "button",
                    onclick: move |_| cancel(),
                    "Cancel"
                }
            }

            p {
                class: "copyright",
                "© OnlineMathLearning.com"
            }
        }
    }
}