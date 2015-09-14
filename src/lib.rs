extern crate graphics;

use graphics::{ Context, Graphics };

pub struct Timeline {
    pub frames: u32,
    pub current_frame: u32,
    pub bounds: [u32; 4],
}

impl Timeline {
    pub fn new_frames_bounds(frames: u32, bounds: [u32; 4]) -> Timeline {
        Timeline {
            frames: frames,
            current_frame: 0,
            bounds: bounds,
        }
    }

    pub fn draw_timeline<G: Graphics>(&self, c: &Context, g: &mut G) {
        // Draw bounds to see where the control is.
        {
            use graphics::Rectangle;

            let bounds = [
                    self.bounds[0] as f64,
                    self.bounds[1] as f64,
                    self.bounds[2] as f64,
                    self.bounds[3] as f64
                ];
            Rectangle::new_border([0.0, 0.0, 1.0, 1.0], 0.5)
                .draw(bounds, &c.draw_state, c.transform, g);
        }
        
    }

    pub fn draw_drag<G: Graphics>(&self, c: &Context, g: &mut G) {

    }
}
