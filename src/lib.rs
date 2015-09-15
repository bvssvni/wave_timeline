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
        let left_to_frame: f64 = 20.0;
        let right_to_frame: f64 = 20.0;
        let top_to_frame: f64 = 15.0;
        let frame_width: f64 = 10.0;
        let frame_height: f64 = 10.0;
        let frame_offset_x: f64 = 5.0;
        let slide_offset: f64 = 0.0;
        let left_to_goto_beginning: f64 = 15.0;
        let right_to_goto_end: f64 = 15.0;
        let width_for_frames = self.bounds[2] as f64
            - left_to_frame - right_to_frame;
        let max_visible_frames: u32 =
            if width_for_frames < frame_width {
                0
            } else {
                1
                + (
                    (width_for_frames - frame_width)
                    / (frame_width + frame_offset_x)
                ) as u32
            };

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

        // Draw navigate to beginning button.
        {
            fn draw_triangle<G: Graphics>(
                offset: graphics::math::Vec2d,
                scale_up: graphics::math::Vec2d,
                line: &Line,
                c: &Context,
                g: &mut G
            ) {
                use graphics::math::*;

                let triangle = [
                    [0.0, 0.0],
                    [1.0, 0.5],
                    [0.0, 1.0]
                ];
                let transform_triangle = multiply(
                    translate(offset),
                    scale(scale_up[0], scale_up[1])
                );
                let p = [
                    transform_pos(transform_triangle, triangle[0]),
                    transform_pos(transform_triangle, triangle[1]),
                    transform_pos(transform_triangle, triangle[2])
                ];
                for i in 0..3 {
                    let j = (i + 1) % 3;
                    line.draw([p[i][0], p[i][1], p[j][0], p[j][1]], &c.draw_state, c.transform, g);
                }
            }

            fn draw_double_arrow<G: Graphics>(
                offset: graphics::math::Vec2d,
                scale_up: graphics::math::Vec2d,
                line: &Line,
                c: &Context,
                g: &mut G
            ) {
                draw_triangle(offset, scale_up, line, c, g);
                let offset = [offset[0] + scale_up[0], offset[1]];
                draw_triangle(offset, scale_up, line, c, g);
            }

            fn draw_goto_end<G: Graphics>(
                at_end: bool,
                offset: graphics::math::Vec2d,
                scale_up: graphics::math::Vec2d,
                line: &Line,
                c: &Context,
                g: &mut G
            ) {
                if !at_end {
                    draw_triangle(offset, scale_up, line, c, g);
                }

                line.draw([
                    offset[0] + scale_up[0], offset[1],
                    offset[0] + scale_up[0], offset[1] + scale_up[1]
                ], &c.draw_state, c.transform, g);
            }

            use graphics::Line;
            use graphics::math::*;

            let line = Line::new([0.0, 0.0, 1.0, 1.0], 0.5);

            let at_beginning: bool = false;
            draw_goto_end(
                at_beginning,
                [
                    self.bounds[0] as f64 + left_to_goto_beginning,
                    self.bounds[1] as f64 + top_to_frame
                ],
                [
                    -frame_width * 0.5,
                    frame_height
                ],
                &line,
                c, g
            );

            let at_end: bool = true;
            draw_goto_end(
                at_end,
                [
                    self.bounds[0] as f64 + self.bounds[2] as f64
                        - right_to_goto_end,
                    self.bounds[1] as f64 + top_to_frame
                ],
                [
                    frame_width * 0.5,
                    frame_height
                ],
                &line,
                c, g
            );
        }
    }

    pub fn draw_drag<G: Graphics>(&self, c: &Context, g: &mut G) {

    }
}
