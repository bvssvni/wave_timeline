extern crate piston_window;
extern crate sdl2_window;
extern crate wave_timeline;

use sdl2_window::Sdl2Window;
use piston_window::*;

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
            use wave_timeline::timeline_split::test_draw_timeline as draw;

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
