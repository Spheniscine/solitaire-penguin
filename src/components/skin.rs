use dioxus::prelude::*;

use crate::{components::SkinTrait, game::{Card, Skin, Suit, SuitSkin}};


impl SkinTrait<Card> for Skin {
    fn get_color(&self, card: &Card) -> String {
        self.colors.color(card.suit).to_string()
    }

    fn render_rank(&self, card: &Card) -> Element {
        rsx! {
            div {
                font_family: "KaTeX_Main",
                {self.ranks.rank_text(card.rank)}
            }
        }
    }

    fn render_suit(&self, card: &Card) -> Element {
        if self.suits == SuitSkin::Animals {
            let asset = match card.suit {
                Suit::Clubs => asset!("assets/emoji/emoji_u1f430.svg"),
                Suit::Diamonds => asset!("assets/emoji/emoji_u1f981.svg"),
                Suit::Hearts => asset!("assets/emoji/emoji_u1f98a.svg"),
                Suit::Spades => asset!("assets/emoji/emoji_u1f427.svg"),
            };
            rsx! {
                img {
                    style: "height: 1.15em;",
                    src: asset,
                    draggable: false,
                }
            }
        } else {
            rsx! {
                span {
                    font_family: self.suits.font(),
                    position: if self.suits == SuitSkin::Shapes {"relative"},
                    top: if self.suits == SuitSkin::Shapes {"0.12em"},
                    {self.suits.suit_symbol(card.suit)}
                }
            }
        }
    }
}