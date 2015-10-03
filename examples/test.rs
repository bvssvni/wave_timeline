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

    let mut timeline = {
        let frames = 100;
        let bounds = [10, 600, 300, 50];
        Timeline::new_frames_bounds(frames, bounds)
    };

    let mut toggle = 0;
    for e in window {
        timeline.bounds[2] = e.size().width - 2 * 10;
        timeline.event(&e);
        e.draw_2d(|c, g| {
            clear([1.0; 4], g);

            timeline.draw_timeline(&c, g);
            timeline.draw_drag(&c, g);
        });
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                toggle += 1;
                match toggle % 2 {
                    0 => {
                        timeline.bounds[3] += 10;
                    }
                    1 => {
                        timeline.bounds[3] -= 10;
                    }
                    _ => {}
                }
            }
        }
    }
}
