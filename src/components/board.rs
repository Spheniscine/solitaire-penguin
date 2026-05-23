use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::{CARD_HEIGHT_RATIO, CardComponent, CardFrame, SkinTrait, rem}, game::{Board, BoardPos, Card, DepotIndex, DepotRole, NUM_COLUMNS, NUM_DEPOTS, NUM_FOUNDATIONS, NUM_FREECELLS, Skin, Suit}};


#[component]
pub fn BoardComponent(
    position: Vec2,
    board: Board,
    skin: Skin,
) -> Element {
    let card_width = 12f32;
    let card_height = card_width * CARD_HEIGHT_RATIO;
    let spacer = 2f32;

    let center_x = |n: usize, i: usize| 
        50. - (card_width * n as f32 + spacer * (n-1) as f32) / 2. + (card_width + spacer) * i as f32;

    let start_y = 2f32;
    let pos_y = |i: usize| start_y + (card_height + spacer) * i as f32;

    let column_card_offset = Vec2::new(0., 6.5);

    let get_pos = |depot: usize, ord: usize| {
        let (role, index) = DepotIndex(depot).role_and_subindex();
        match role {
            DepotRole::Foundation => 
                Vec2::new(center_x(NUM_FOUNDATIONS, index), pos_y(0)),
            DepotRole::FreeCell => 
                Vec2::new(center_x(NUM_FREECELLS, index), pos_y(1)),
            DepotRole::Column => {
                Vec2::new(center_x(NUM_COLUMNS, index), pos_y(2)) + column_card_offset * ord as f32
            },
        }
    };

    let get_hint = |depot: usize| {
        match DepotIndex(depot).role() {
            DepotRole::Foundation => {
                skin.render_rank(&Card { rank: board.foundation_rank(), suit: Suit::Spades })
            },
            DepotRole::FreeCell => rsx!{
                span {
                    font_family: "'Noto Sans Symbols 2'",
                    position: "relative",
                    top: "0.12em",
                    "✽"
                }
            },
            DepotRole::Column => {
                skin.render_rank(&Card { rank: board.column_head_rank(), suit: Suit::Spades })
            },
        }
    };

    let selected_height = if let Some(BoardPos { depot_index, card_index }) = board.selected {
        let d = if DepotIndex(depot_index).role() == DepotRole::Column {
            board.depots[depot_index].len() - card_index - 1
        } else {
            0
        };

        card_height + column_card_offset.y * d as f32
    } else {0.};

    rsx! {
        div {
            position: "absolute",
            top: rem(position.y),
            left: rem(position.x),

            for depot in 0..NUM_DEPOTS {
                CardFrame { 
                    position: get_pos(depot, 0),
                    width: card_width,
                    hint: get_hint(depot),
                }

                for i in 0..board.depots[depot].len() {
                    if board.selected == Some(BoardPos { depot_index: depot, card_index: i }) {
                        div {
                            position: "absolute",
                            top: rem(get_pos(depot, i).y),
                            left: rem(get_pos(depot, i).x),
                            width: rem(card_width),
                            height: rem(selected_height),
                            background_color: "#ff0",
                            border_radius: rem(card_width * (1.5 / 12.)),
                            class: "selected-halo",
                        }
                    }

                    CardComponent {
                        position: get_pos(depot, i),
                        width: card_width,
                        card: board.depots[depot][i],
                        skin,
                    }
                }
            }
        }
    }
}