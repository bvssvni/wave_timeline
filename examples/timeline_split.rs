extern crate piston_window;
extern crate sdl2_window;
extern crate wave_timeline;
extern crate rect;

use sdl2_window::Sdl2Window;
use piston_window::*;
use wave_timeline::timeline_split::*;
use rect::*;

fn main() {
    let window: PistonWindow<(), Sdl2Window> =
        WindowSettings::new("wave_timeline: test", [1024, 768])
        .exit_on_esc(true)
        .samples(8)
        .build()
        .unwrap();

    let mut width: f64 = 100.0;
    let mut height: f64 = 100.0;
    for e in window {
        if let Some(pos) = e.mouse_cursor_args() {
            let pos: [f64; 2] = pos;
            width = pos[0];
            height = pos[1];
        }
        e.draw_2d(|c, g| {
            use test_draw_timeline as draw;

            clear([1.0; 4], g);

            draw([0, 0, 500, 50], &c, g);
            draw([100, 100, 600, 80], &c, g);

            draw([0, 200, 100, 80], &c, g);
            draw([0, 300, 190, 80], &c, g);
            draw([0, 400, 200, 80], &c, g);
            draw([0, 500, 210, height as u32], &c, g);
            draw([0, 600, width as u32, 80], &c, g);
        });
    }
}

/////////////////////// DRAWING ////////////////////////////

const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

fn draw_box<G: Graphics>(color: [f32; 4], rect: [f64; 4], c: &Context, g: &mut G) {
    Rectangle::new_border(color, 0.5)
        .draw(rect, &c.draw_state, c.transform, g);
}

/// Test drawing for timeline split.
fn test_draw_timeline<G: Graphics>(rect: [u32; 4], c: &Context, g: &mut G) {
    let top = 20.0;
    let top_factor = 0.3;
    if let Some(layout) =
        (TimelineSplitInput {
            rect: Rect::from_u32(rect),
            margin: 4.0,
            left: 40.0,
            right: 40.0,
            top: top,
        }).call() {

        draw_box(BLUE, layout.left, c, g);
        draw_box(BLUE, layout.middle, c, g);
        draw_box(BLUE, layout.right, c, g);

        let (_, left) = layout.left.split_top(top, top_factor);
        draw_box(RED, left, c, g);
        let (_, middle) = layout.middle.split_top(top, top_factor);
        draw_box(RED, middle, c, g);
        let (_, right) = layout.right.split_top(top, top_factor);
        draw_box(RED, right, c, g);
    }
}
