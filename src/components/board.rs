use dioxus::prelude::*;
use glam::Vec2;

use crate::{components::{CARD_HEIGHT_RATIO, CardComponent, CardFrame, Movement, SkinTrait, rem}, game::{AnimationAct, Board, BoardPos, Card, DepotIndex, DepotRole, NUM_DEPOTS, NUM_FOUNDATIONS, NUM_FREECELLS, NUM_TABLEAU_DEPOTS, Skin, Suit}};


#[component]
pub fn BoardComponent(
    position: Vec2,
    board: Board,
    skin: Skin,
    onclick: EventHandler<BoardPos>,
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
            DepotRole::Tableau => {
                Vec2::new(center_x(NUM_TABLEAU_DEPOTS, index), pos_y(2)) + column_card_offset * ord as f32
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
            DepotRole::Tableau => {
                skin.render_rank(&Card { rank: board.column_head_rank(), suit: Suit::Spades })
            },
        }
    };

    let selected_height = if let Some(BoardPos { depot_index, card_index }) = board.selected {
        let d = if DepotIndex(depot_index).role() == DepotRole::Tableau {
            board.depots[depot_index].len() - card_index - 1
        } else {
            0
        };

        card_height + column_card_offset.y * d as f32
    } else {0.};

    let anim_iter = board.animation_acts.iter().flat_map(|act| {
        match act {
            AnimationAct::Move(cards, pos1, pos2) => {
                let mut pos1 = *pos1;
                let mut pos2 = *pos2;
                cards.iter().map(move |card| {
                    let p1 = get_pos(pos1.depot_index, pos1.card_index);
                    let p2 = get_pos(pos2.depot_index, pos2.card_index);
                    let res = rsx! {
                        Movement {
                            src_translate_vec: p1 - p2,
                            CardComponent {
                                position: p2,
                                width: card_width,
                                card: *card,
                                skin,
                                onclick: move |_| {},
                            }
                        }
                    };
                    pos1.card_index += 1;
                    pos2.card_index += 1;
                    res
                })
            },
        }
    });
    

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
                    onclick: move |_| {
                        onclick.call(BoardPos { depot_index: depot, card_index: !0 })
                    },
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
                        onclick: move |_| {
                            onclick.call(BoardPos { depot_index: depot, card_index: i })
                        },
                    }
                }
            }

            for anim in anim_iter {
                {anim},
            }
        }
    }
}