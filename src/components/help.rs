use dioxus::prelude::*;
use glam::Vec2;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::{KATEX_SUITS, components::{BoardComponent, CardText, TUTORIAL_BEAK, TUTORIAL_DECK, rem, skin::KATEX_MAIN}, game::{Board, Card, ColorMode, GameState, GameVariant, RANK_MAX, RANK_MIN, RankSkin, Skin, Suit}};

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
fn MinRank(skin: Skin) -> Element {
    match skin.ranks {
        RankSkin::Numbers => rsx! {
            span {
                font_family: KATEX_MAIN,
                font_size: "1.2em",
                "{RANK_MIN}"
            }
        },
        RankSkin::Traditional => rsx! {
            "Ace"
        }
    }
}

#[component]
fn MaxRank(skin: Skin) -> Element {
    match skin.ranks {
        RankSkin::Numbers => rsx! {
            span {
                font_family: KATEX_MAIN,
                font_size: "1.2em",
                "{RANK_MAX}"
            }
        },
        RankSkin::Traditional => rsx! {
            "King"
        }
    }
    
}

#[component]
pub fn Help(game_state: Signal<GameState>) -> Element {
    let st = game_state.read();

    let variant = st.variant;
    let skin = st.skin;

    let stack_example = || {
        let mut ite = (2..=5).rev().map(|rank| {
            rsx! {
                CardText {
                    card: Card { rank, suit: Suit::Spades },
                    skin,
                    color_mode: ColorMode::Light,
                }
            }
        });
        let last = ite.next().unwrap();
        rsx! {
            {ite.next().unwrap()},
            for x in ite { "–", {x} },
            " can be placed on the ", {last}
        }
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; font-size: 3.5rem; color: #fff; padding: 4rem;",
            class: "help",

            div {
                text_align: "left",

                p {
                    margin_top: "0",
                    "The layout consists of three rows:"
                    ul {
                        li { "The ", Emph {"foundation"}, " with 4 stacks, one for each suit"}
                        li { "The ", Emph {"flipper"}, " with 7 free cells"}
                        li { "The ", Emph {"tableau"}, " with 7 columns"}
                    }
                }

                if variant == GameVariant::Tuxedo {
                    p {
                        "A standard 52-card deck is shuffled and dealt into 7 rows each for each tableau column, then one extra card to the ",
                        "1",sup{"st"},", 4",sup{"th"}," and 7",sup{"th"}," columns."
                    }
                        
                } else if variant == GameVariant::Original {
                    p {
                        "A standard 52-card deck is shuffled and dealt. The first card dealt to the back of the first column is called the ",Emph{"beak"},". The rest of the cards are then dealt evenly to the columns, but the other three cards of the same rank as the beak will go to the foundations instead."
                    }
                }
                
                p {
                    "Cards in the ", Emph {"tableau"}, " are stacked by descending ranks of the same suit. Such stacks of any size can be moved as a unit."
                    if variant == GameVariant::Tuxedo {
                        " (e.g. ",{stack_example()},")"
                    } else if variant == GameVariant::Original {
                        " (In the ",Emph {"Original"}," variant, ranks ",Emph {"“wrap around”"}," so that ",
                        MaxRank { skin }, "s are one rank below the ",MinRank { skin },")"
                    }
                }

                p {
                    Emph {"NOTE:"}, " To move cards, click to select a card or stack, then click the destination. ", Emph{"“Drag and drop” is not required."}
                }

                p {
                    "An empty column in the tableau may only be filled by a card or stack headed by a ",
                    if variant == GameVariant::Tuxedo {
                        MaxRank { skin }
                    } else if variant == GameVariant::Original {
                        "card one rank below the beak"
                    },
                    "."
                }

                p {
                    "The ", Emph {"flipper"}, " has 7 ", Emph {"free cells"}, ", each of which may store one card of any kind."
                }

                p {
                    "The ", Emph {"foundations"}, " are built by suit in ascending order ",
                    if variant == GameVariant::Tuxedo {
                        "from ",MinRank { skin }," to ",MaxRank { skin }
                    } else if variant == GameVariant::Original {
                        "from the beak’s rank up, wrapping around if needed"
                    },
                    "."
                }

                p {
                    "You win the game when all cards have been moved to the foundations."
                }

                p {
                    "Shortcut note: Double-clicking on a card will automatically try to move it to a valid position, in the priority order of: foundations, tableau, flipper."
                }

                div {
                    position: "absolute",
                    bottom: rem(2.),
                    width: "92rem",
                    display: "flex",
                    justify_content: "center",
                    div {
                        width: rem(30.),
                        position: "relative",
                        class: "game-button",
                        "Back to game"
                    }
                }
                
            }
        }
        
    }
}