use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, FromRepr};

use crate::game::Suit;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, EnumIter, strum_macros::Display, Default, FromRepr)]
#[repr(u8)]
pub enum RankSkin {
    #[default]
    Numbers,
    Traditional,
}

impl RankSkin {
    pub fn rank_text(self, rank: u8) -> String {
        match self {
            RankSkin::Numbers => rank.to_string(),
            RankSkin::Traditional => {
                match rank {
                    1 => String::from("A"),
                    11 => String::from("J"),
                    12 => String::from("Q"),
                    13 => String::from("K"),
                    _ => rank.to_string(),
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, EnumIter, strum_macros::Display, Default, FromRepr)]
#[repr(u8)]
pub enum SuitSkin {
    #[default]
    Animals,
    Shapes,
    Traditional,
}

impl SuitSkin {
    pub fn suit_symbol(self, suit: Suit) -> &'static str {
        match self {
            SuitSkin::Animals => {
                match suit {
                    Suit::Clubs => "🐰",
                    Suit::Diamonds => "🦁",
                    Suit::Hearts => "🦊",
                    Suit::Spades => "🐧",
                }
            },
            SuitSkin::Shapes => {
                match suit {
                    Suit::Clubs => "▲",
                    Suit::Diamonds => "⬥",
                    Suit::Hearts => "●",
                    Suit::Spades => "★",
                }
            }
            SuitSkin::Traditional => {
                match suit {
                    Suit::Clubs => "♣",
                    Suit::Diamonds => "♦︎",
                    Suit::Hearts => "♥",
                    Suit::Spades => "♠",
                }
            }
        }
    }

    pub fn font(self) -> &'static str {
        match self {
            SuitSkin::Animals => "'Noto Color Emoji', 'Apple Color Emoji'",
            SuitSkin::Shapes => "'Noto Sans Symbols 2'",
            SuitSkin::Traditional => "KaTeX_Main", // links to custom version of Katex/MLModern that has filled card suits
        }
    }
}

const COLOR_AMBER: &str = "#b70";
const COLOR_GREEN: &str = "#062";
const COLOR_RED: &str = "#f00";
const COLOR_BLUE: &str = "#00d";
const COLOR_BLACK: &str = "#000";

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, EnumIter, strum_macros::Display, Default, FromRepr)]
#[repr(u8)]
pub enum ColorSkin {
    #[default]
    #[strum(to_string = "Four colors")]
    FourColor,
    #[strum(to_string = "Two colors")]
    TwoColor,
}

impl ColorSkin {
    pub fn color(self, suit: Suit) -> &'static str {
        match self {
            ColorSkin::FourColor => {
                // Use Spectrum Bridge colors - better distinction between reddish/warm and blackish/cool colors for
                // solitaires that care about that
                match suit {
                    Suit::Clubs => COLOR_GREEN,
                    Suit::Diamonds => COLOR_AMBER,
                    Suit::Hearts => COLOR_RED,
                    Suit::Spades => COLOR_BLUE,
                }
            },
            ColorSkin::TwoColor => {
                match suit {
                    Suit::Clubs | Suit::Spades => COLOR_BLACK,
                    Suit::Diamonds | Suit::Hearts => COLOR_RED,
                }
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct Skin {
    pub ranks: RankSkin,
    pub suits: SuitSkin,
    pub colors: ColorSkin,
}