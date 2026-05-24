use async_std::stream::StreamExt;
use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::{BoardComponent, rem}, game::{ANIMATION_DURATION, AnimationKey, GameState, Skin}};

mod game;
mod components;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const KATEX_SUITS: Asset = asset!("/assets/KaTeX_Suits.woff2");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link {
            rel: "preconnect",
            href: "https://fonts.googleapis.com",
        }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "anonymous",
        }
        document::Link {
            href: "https://fonts.googleapis.com/css2?family=Noto+Color+Emoji&family=Noto+Sans+Symbols+2&family=Noto+Sans:ital,wght@0,100..900;1,100..900&display=swap",
            rel: "stylesheet",
        }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Style {
            r#"
            @font-face {{
                font-family: KaTeX_Main;
                font-style: normal;
                font-weight: 700;
                src: url({KATEX_SUITS}) format("woff2");
            }}
            "#,
        }
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {

    let skin = Skin { 
        ranks: game::RankSkin::Numbers, 
        suits: game::SuitSkin::Animals, 
        colors: game::ColorSkin::FourColor,
    };

    let mut state = use_signal(|| {
        GameState::init()
    });

    let st = state.read();
    let clean = !st.is_busy();

    let animate_timer = use_coroutine(move |mut rx: UnboundedReceiver<AnimationKey>| async move {
        while let Some(key) = rx.next().await {
            async_std::task::sleep(ANIMATION_DURATION).await;
            state.write().advance_animations(key);
        }
    });

    if st.is_acting() {
        animate_timer.send(st.animation_key);
    }

    rsx! {
        div {
            id: "hero",
            class: "select-none",

            div {
                position: "absolute",
                border: "{rem(0.5)} solid #00B163",
                border_radius: rem(1.),
                padding: rem(1.),
                top: rem(2.),
                left: rem(2.),
                font_size: rem(4.),
                width: rem(24.),
                color: "#fff",
                text_align: "center",
                "New Game"
            }

            div {
                position: "absolute",
                border: "{rem(0.5)} solid #00B163",
                border_radius: rem(1.),
                padding: rem(1.),
                top: rem(2.),
                right: rem(2.),
                font_size: rem(4.),
                width: rem(24.),
                color: "#fff",
                text_align: "center",
                "Settings"
            }

            div {
                position: "absolute",
                border: "{rem(0.5)} solid #00B163",
                border_radius: rem(1.),
                padding: rem(1.),
                top: rem(2.),
                right: rem(30.),
                font_size: rem(4.),
                width: rem(24.),
                color: "#fff",
                text_align: "center",
                "Restart"
            }

            div {
                position: "absolute",
                border: "{rem(0.5)} solid #00B163",
                border_radius: rem(1.),
                padding: rem(1.),
                top: rem(11.),
                right: rem(2.),
                font_size: rem(4.),
                width: rem(24.),
                color: "#fff",
                text_align: "center",
                "Help"
            }

            div {
                position: "absolute",
                border: "{rem(0.5)} solid #00B163",
                border_radius: rem(1.),
                padding: rem(1.),
                top: rem(11.),
                right: rem(30.),
                font_size: rem(4.),
                width: rem(24.),
                color: "#fff",
                text_align: "center",
                onclick: move |_| if clean {state.write().undo()},
                "Undo"
            }

            BoardComponent { 
                position: Vec2 { x: 0., y: 20. },
                board: state.read().board.clone(),
                skin,
                onclick: move |pos| if clean {state.write().onclick(pos);},
                animation_key: st.animation_key,
            }
        }
    }
}
