extern crate piston_window;
extern crate sdl2_window;
extern crate wave_timeline;

use sdl2_window::Sdl2Window;
use piston_window::*;
use wave_timeline::*;

fn main() {
    let window: PistonWindow<(), Sdl2Window> =
        WindowSettings::new("wave_timeline: test", [1024, 768])
        .exit_on_esc(true)
        .samples(8)
        .build()
        .unwrap();

    for e in window {
        e.draw_2d(|c, g| {
            clear([1.0; 4], g);

            wave_timeline::new_timeline::draw_timeline([0, 0, 500, 50], &c, g);
            wave_timeline::new_timeline::draw_timeline([100, 100, 600, 80], &c, g);
        });
    }
}
