use dioxus::prelude::*;
use glam::Vec2;

use crate::components::rem;

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
    #[props(default)]
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let pt = width / 12.;
    let pt = |x: f32| {
        rem(x * pt)
    };

    rsx! {
        div {
            style: "place-items: center;",
            position: "absolute",
            top: rem(position.y),
            left: rem(position.x),
            background_color: "#fff",
            width: pt(11.),
            height: pt(12.),
            border: "{pt(0.25)} solid #000",
            border_radius: pt(1.5),
            display: "grid",
            grid_template_columns: "50% 50%",
            grid_template_rows: "50% 50%",
            font_size: pt(5.),
            text_align: "center",
            padding: pt(0.5),
            color: skin.get_color(&card),

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