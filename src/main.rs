use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::{CARD_HEIGHT_RATIO, CardComponent, DepotComponent}, game::{Card, Skin}};

mod game;
mod components;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
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
                    offset: Vec2::new(0., 6.),
                    width: card_width,
                    cards: test_depot.clone(),
                    skin,
                }
            }
            
            // div {
            //     style: "place-items: center; --tx: -30rem; --ty: 10rem;",
            //     position: "absolute",
            //     top: "3rem",
            //     left: "44rem",
            //     background_color: "#fff",
            //     width: "11rem",
            //     height: "12rem",
            //     border: "0.25rem solid #000",
            //     border_radius: "1.5rem",
            //     display: "grid",
            //     grid_template_columns: "50% 50%",
            //     grid_template_rows: "50% 50%",
            //     font_size: "5rem",
            //     text_align: "center",
            //     padding: "0.5rem",
            //     animation: "0.2s movement",
            //     color: colors[3],

            //     div {
            //         font_family: "KaTeX_Main",
            //         "A"
            //     },
            //     div {
            //         font_family: "KaTeX_Main",
            //         line_height: "1",
            //         "{suits[3]}",
            //     },
            //     div {
            //         font_family: "KaTeX_Main",
            //         line_height: "1",
            //         "{suits[3]}",
            //     },
            //     div {
            //         font_family: "KaTeX_Main",
            //         "A"
            //     },
            // }
        }
    }
}
