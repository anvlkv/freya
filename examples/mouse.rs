#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    rsx!(
        rect {
            color: "white",
            height: "100%",
            width: "100%",
            Area {

            }
            Area {

            }
        }
    )
}

#[allow(non_snake_case)]
fn Area(cx: Scope) -> Element {
    let cursor_pos_over = use_state(cx, || (0f64, 0f64));
    let cursor_pos_click = use_state(cx, || (0f64, 0f64));

    let cursor_moved = |e: MouseEvent| {
        cursor_pos_over.with_mut(|cursor_pos| {
            let pos = e.get_screen_coordinates();
            cursor_pos.0 = pos.x;
            cursor_pos.1 = pos.y;
        })
    };

    let cursor_clicked = |e: MouseEvent| {
        cursor_pos_click.with_mut(|cursor_pos| {
            let pos = e.get_screen_coordinates();
            cursor_pos.0 = pos.x;
            cursor_pos.1 = pos.y;
        })
    };

    rsx!(
        rect {
            height: "50%",
            width: "100%",
            background: "blue",
            padding: "5",
            onmouseover: cursor_moved,
            onclick: cursor_clicked,
            label {
                "Mouse is at [x: {cursor_pos_over.0}, y: {cursor_pos_over.1}] ",
            },
            label {
                "Mouse clicked at [x: {cursor_pos_click.0}, y: {cursor_pos_click.1}]"
            }
        }
    )
}
