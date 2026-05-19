use dioxus::prelude::*;
use glam::Vec2;

use crate::components::rem;

pub trait SkinTrait<C>: PartialEq {
    fn get_color(&self, card: &C) -> String;
    fn render_rank(&self, card: &C) -> Element;
    fn render_suit(&self, card: &C) -> Element;
}

#[component]
pub fn CardComponent<C: PartialEq + 'static, S: SkinTrait<C> + 'static>(
    position: Vec2,
    width: f32,
    card: C,
    skin: S,
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

            {skin.render_rank(&card)},
            {skin.render_suit(&card)},
            {skin.render_suit(&card)},
            {skin.render_rank(&card)},
        }
    }
}