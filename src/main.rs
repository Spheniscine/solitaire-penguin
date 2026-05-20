use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::{CARD_HEIGHT_RATIO, DepotComponent, rem}, game::{Card, Skin}};

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
    let card_width = 12f32;
    let card_height = card_width * CARD_HEIGHT_RATIO;
    let spacer = 2f32;

    let center_x = |n: usize, i: usize| 
        50. - (card_width * n as f32 + spacer * (n-1) as f32) / 2. + (card_width + spacer) * i as f32;

    let skin = Skin { 
        ranks: game::RankSkin::Numbers, 
        suits: game::SuitSkin::Animals, 
        colors: game::ColorSkin::FourColor,
    };

    let test_single = vec![Card { rank: 10, suit: game::Suit::Clubs }];

    let mut test_depot = vec![];
    for i in 1..=6 {
        test_depot.push(Card { rank: i, suit: game::Suit::Spades });
    }

    for i in (2..=13).rev() {
        test_depot.push(Card { rank: i, suit: game::Suit::Diamonds });
    }

    let start_y = 2f32;
    let pos_y = |i: usize| start_y + (card_height + spacer) * i as f32;

    rsx! {
        div {
            id: "hero",

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
                top: rem(11.5),
                left: rem(2.),
                font_size: rem(4.),
                width: rem(24.),
                color: "#fff",
                text_align: "center",
                "Test"
            }

            div {
                position: "absolute",
                top: rem(20.),
                left: 0,

                for i in 0..4 {
                    DepotComponent { 
                        position: Vec2::new(
                            center_x(4, i),
                            pos_y(0)
                        ),
                        offset: Vec2::new(0., 0.),
                        width: card_width,
                        cards: test_single.clone(),
                        skin,
                    }
                }

                for i in 0..7 {
                    DepotComponent { 
                        position: Vec2::new(
                            center_x(7, i),
                            pos_y(1)
                        ),
                        offset: Vec2::new(0., 0.),
                        width: card_width,
                        cards: test_single.clone(),
                        skin,
                    }
                }

                for i in 0..7 {
                    DepotComponent { 
                        position: Vec2::new(
                            center_x(7, i),
                            pos_y(2)
                        ),
                        offset: Vec2::new(0., 6.5),
                        width: card_width,
                        cards: test_depot.clone(),
                        skin,
                    }
                }
            }
        }
    }
}
