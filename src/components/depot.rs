use dioxus::prelude::*;
use glam::Vec2;

use crate::components::{CardComponent, CardFrame, SkinTrait};

#[component]
pub fn DepotComponent<C: PartialEq + Clone + 'static, S: SkinTrait<C> + 'static>(
    position: Vec2,
    offset: Vec2,
    width: f32,
    cards: Vec<C>,
    skin: S,
) -> Element {
    let pos = |i: usize| position + offset * i as f32;
    rsx! {
        CardFrame {
            position,
            width,
        }
        for i in 0..cards.len() {
            CardComponent { 
                position: pos(i),
                width,
                card: cards[i].clone(),
                skin: skin.clone(),
            }
        }
    }
}