extern crate piston_window;
extern crate wave_timeline;

use piston_window::*;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("wave_timeline: test", [1024, 768])
        .exit_on_esc(true)
        .build()
        .unwrap();
    for e in window {
        e.draw_2d(|c, g| {
            clear([1.0; 4], g);
        });
    }
}
