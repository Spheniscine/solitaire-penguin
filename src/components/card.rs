use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::rem, game::ANIMATION_DURATION};

pub trait SkinTrait<C>: PartialEq + Clone {
    fn get_color(&self, card: &C) -> String;
    fn render_rank(&self, card: &C) -> Element;
    fn render_suit(&self, card: &C) -> Element;
}

pub const CARD_HEIGHT_RATIO: f32 = 13. / 12.;

#[component]
pub fn CardComponent<C: PartialEq + Clone + 'static, S: SkinTrait<C> + 'static>(
    position: Vec2,
    width: f32,
    card: C,
    skin: S,
    onclick: EventHandler<MouseEvent>,
    animate_from: Option<Vec2>,
) -> Element {
    let pt = width / 12.;
    let pt = |x: f32| {
        rem(x * pt)
    };

    let animation = if let Some(animate_from) = animate_from {
        let diff = animate_from - position;
        format!("--translateX: {}; --translateY: {}; animation: {}s movement;", rem(diff.x), rem(diff.y), ANIMATION_DURATION.as_secs_f32())
    } else {
        String::new()
    };

    rsx! {
        div {
            style: "{animation} place-items: center; position: absolute; top: {rem(position.y)}; left: {rem(position.x)};
            background-color: #fff; width: {pt(11.)}; height: {pt(12.)}; border: {pt(0.25)} solid #000;
            border-radius: {pt(1.5)}; display: grid; grid-template-columns: 50% 50%; grid-template-rows: 50% 50%;
            font-size: {pt(5.)}; text-align: center; padding: {pt(0.5)}; color: {skin.get_color(&card)};",
            onclick,

            div { display: "flex", align_items: "center", {skin.render_rank(&card)}},
            div { display: "flex", align_items: "center", {skin.render_suit(&card)}},
            div { display: "flex", align_items: "center", {skin.render_suit(&card)}},
            div { display: "flex", align_items: "center", {skin.render_rank(&card)}},
        }
    }
}

#[component]
pub fn CardFrame(
    position: Vec2,
    width: f32,
    hint: Option<Element>,
    #[props(default = "#aaa".to_string())] color: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let pt = width / 12.;
    let pt = |x: f32| {
        rem(x * pt)
    };
    rsx! {
        div {
            display: "flex",
            align_items: "center",
            justify_content: "center",
            position: "absolute",
            top: rem(position.y),
            left: rem(position.x),
            margin: pt(0.25), // frame must be slightly smaller than card to prevent peeking out in some platforms
            width: pt(10.),
            height: pt(11.),
            border: "{pt(0.5)} solid {color}",
            text_align: "center",
            color,
            border_radius: pt(1.5),
            font_size: pt(5.),
            padding: pt(0.5),
            onclick,

            if let Some(hint) = hint {
                div {
                    {hint},
                }
            }
        }
    }
}