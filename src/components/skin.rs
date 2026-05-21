use dioxus::prelude::*;

use crate::{components::SkinTrait, game::{Card, Skin, SuitSkin}};


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