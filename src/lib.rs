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
        let left_to_frame: f64 = 5.0;
        let top_to_frame: f64 = 15.0;
        let frame_width: f64 = 10.0;
        let frame_height: f64 = 10.0;
        let frame_offset_x: f64 = 5.0;
        let slide_offset: f64 = 0.0;

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

        // Draw a sequence of squares for each frame.
        {
            use graphics::Rectangle;

            let rect = Rectangle::new_border([0.0, 0.0, 1.0, 1.0], 0.5);
            for i in 0..self.frames {
                let bounds = [
                    self.bounds[0] as f64 + left_to_frame
                        + i as f64 * (frame_width + frame_offset_x)
                        - slide_offset,
                    self.bounds[1] as f64 + top_to_frame,
                    frame_width,
                    frame_height
                ];
                rect.draw(bounds, &c.draw_state, c.transform, g);
            }
        }
    }

    pub fn draw_drag<G: Graphics>(&self, c: &Context, g: &mut G) {

    }
}
