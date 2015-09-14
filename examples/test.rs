extern crate piston_window;
extern crate wave_timeline;

use piston_window::*;
use wave_timeline::*;

fn main() {
    let window: PistonWindow =
        WindowSettings::new("wave_timeline: test", [1024, 768])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let timeline = Timeline::new_frames_bounds(10, [10, 10, 300, 30]);

    for e in window {
        e.draw_2d(|c, g| {
            clear([1.0; 4], g);

            timeline.draw_timeline(&c, g);
            timeline.draw_drag(&c, g);
        });
    }
}
