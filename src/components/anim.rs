use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::rem, game::{ANIMATION_DURATION, AnimationKey}};

#[component]
pub fn Movement(
    animation_key: AnimationKey,
    src_translate_vec: Vec2,
    children: Element,
) -> Element {
    rsx! {
        div {
            style: "--translateX: {rem(src_translate_vec.x)}; --translateY: {rem(src_translate_vec.y)}; 
            animation: {ANIMATION_DURATION.as_secs_f32()}s movement{animation_key & 1};",
            {children},
        }
    }
}