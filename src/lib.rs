extern crate graphics;
extern crate input;

use graphics::{ Context, Graphics };
use input::{ GenericEvent };

mod drawutils;

pub struct Timeline {
    pub frames: u32,
    pub current_frame: u32,
    pub hover_frame: Option<u32>,
    pub start_frame: u32,
    pub bounds: [u32; 4],
    pub settings: TimelineSettings,
    pub hover_goto_start: bool,
    pub hover_goto_end: bool,
}

pub struct TimelineSettings {
    pub left_to_frame: f64,
    pub right_to_frame: f64,
    pub top_to_frame: f64,
    pub frame_size: [f64; 2],
    pub frame_offset_x: f64,
    pub left_to_goto_start: f64,
    pub right_to_goto_end: f64,
    pub lift_hover_frame: f64,
}

pub struct ComputedTimelineSettings {
    pub width_for_frames: f64,
    pub max_visible_frames: u32,
    pub end_frame: u32,
}

impl TimelineSettings {
    pub fn new() -> TimelineSettings {
        TimelineSettings {
            left_to_frame: 20.0,
            right_to_frame: 20.0,
            top_to_frame: 15.0,
            frame_size: [10.0, 10.0],
            frame_offset_x: 5.0,
            left_to_goto_start: 5.0,
            right_to_goto_end: 5.0,
            lift_hover_frame: 10.0,
        }
    }
}

impl ComputedTimelineSettings {
    pub fn new(timeline: &Timeline, settings: &TimelineSettings)
        -> ComputedTimelineSettings
    {
        use std::cmp::min;

        let width_for_frames = timeline.bounds[2] as f64
            - settings.left_to_frame - settings.right_to_frame;
        let max_visible_frames: u32 =
            if width_for_frames < settings.frame_size[0] {
                0
            } else {
                1
                + (
                    (width_for_frames - settings.frame_size[0])
                    / (settings.frame_size[0] + settings.frame_offset_x)
                ) as u32
            };
        let end_frame = min(
            timeline.frames,
            timeline.start_frame + max_visible_frames
        );

        ComputedTimelineSettings {
            width_for_frames: width_for_frames,
            max_visible_frames: max_visible_frames,
            end_frame: end_frame
        }
    }
}

impl Timeline {
    pub fn new_frames_bounds(frames: u32, bounds: [u32; 4]) -> Timeline {
        Timeline {
            frames: frames,
            current_frame: 0,
            hover_frame: None,
            start_frame: 0,
            bounds: bounds,
            settings: TimelineSettings::new(),
            hover_goto_start: false,
            hover_goto_end: false,
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        use input::{ MouseCursorEvent, PressEvent };

        if let Some(pos) = e.mouse_cursor_args() {
            // Detect hover frame.
            {
                let computed_settings = ComputedTimelineSettings::new(self,
                    &self.settings);

                let left_to_frame: f64 = self.settings.left_to_frame;
                let frame_width: f64 = self.settings.frame_size[0];
                let frame_offset_x: f64 = self.settings.frame_offset_x;

                let end_frame: u32 = computed_settings.end_frame;

                let (x, y) = (pos[0], pos[1]);

                let left = self.bounds[0] as f64 + left_to_frame;
                let right = left + end_frame as f64 * (frame_width + frame_offset_x);
                let top = self.bounds[1] as f64;
                let bottom = top + self.bounds[3] as f64;

                if x < left || x > right
                || y < top || y > bottom {
                    self.hover_frame = None;
                } else {
                    let i: u32 = ((x - left) / (frame_width + frame_offset_x)) as u32;
                    self.hover_frame = Some(i);
                }
            }

            // Detect hover over goto start and goto end buttons.
            {
                let left_to_goto_start = self.settings.left_to_goto_start;
                let right_to_goto_end = self.settings.right_to_goto_end;
                let top_to_frame = self.settings.top_to_frame;
                let frame_width = self.settings.frame_size[0];
                let frame_height = self.settings.frame_size[1];

                let outside = |bounds: [f64; 4]| {
                    let (x, y) = (pos[0], pos[1]);
                    (x < bounds[0] || y < bounds[1]
                    || x > bounds[0] + bounds[2] || y > bounds[1] + bounds[3])
                };

                self.hover_goto_start = !outside([
                    self.bounds[0] as f64 + left_to_goto_start,
                    self.bounds[1] as f64 + top_to_frame,
                    frame_width,
                    frame_height
                ]);
                self.hover_goto_end = !outside([
                    self.bounds[0] as f64 + self.bounds[2] as f64
                        - right_to_goto_end - frame_width,
                    self.bounds[1] as f64 + top_to_frame,
                    frame_width,
                    frame_height
                ]);
            }
        }

        if let Some(button) = e.press_args() {
            use input::{ Button, MouseButton };

            if button == Button::Mouse(MouseButton::Left) {
                if self.hover_goto_start {
                    println!("TEST goto start");
                } else if self.hover_goto_end {
                    println!("TEST goto end");
                }
            }
        }
    }

    pub fn draw_timeline<G: Graphics>(&self, c: &Context, g: &mut G) {
        use std::cmp::min;

        let left_to_frame: f64 = self.settings.left_to_frame;
        let right_to_frame: f64 = self.settings.right_to_frame;
        let top_to_frame: f64 = self.settings.top_to_frame;
        let frame_width: f64 = self.settings.frame_size[0];
        let frame_height: f64 = self.settings.frame_size[1];
        let frame_offset_x: f64 = self.settings.frame_offset_x;
        let left_to_goto_start: f64 = self.settings.left_to_goto_start;
        let right_to_goto_end: f64 = self.settings.right_to_goto_end;
        let lift_hover_frame: f64 = self.settings.lift_hover_frame;

        let computed_settings = ComputedTimelineSettings::new(self, &self.settings);

        let slide_offset: f64 = 0.0;

        let width_for_frames = computed_settings.width_for_frames;
        let max_visible_frames: u32 = computed_settings.max_visible_frames;
        let end_frame = computed_settings.end_frame;

        let blue = [0.0, 0.0, 1.0, 1.0];
        let red = [1.0, 0.0, 0.0, 1.0];

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

            let frame_bounds = |i: u32, lift: f64| [
                self.bounds[0] as f64 + left_to_frame
                    + i as f64 * (frame_width + frame_offset_x)
                    - slide_offset,
                self.bounds[1] as f64 + top_to_frame - lift,
                frame_width,
                frame_height
            ];

            let rect = Rectangle::new_border(blue, 0.5);
            for i in 0..end_frame - self.start_frame {
                // Don't draw the hover frame.
                if Some(i) == self.hover_frame { continue; }

                let bounds = frame_bounds(i, 0.0);
                rect.draw(bounds, &c.draw_state, c.transform, g);
            }

            if let Some(i) = self.hover_frame {
                let bounds = frame_bounds(i, lift_hover_frame);
                rect.draw(bounds, &c.draw_state, c.transform, g);
            }
        }

        // Draw navigate to start or end buttons.
        {
            use graphics::Line;
            use graphics::math::*;

            let line = Line::new([0.0, 0.0, 1.0, 1.0], 0.5);

            let at_beginning: bool = self.start_frame == 0;
            drawutils::draw_goto_end(
                at_beginning,
                [
                    self.bounds[0] as f64 + left_to_goto_start + frame_width,
                    self.bounds[1] as f64 + top_to_frame
                ],
                [
                    -frame_width * 0.5,
                    frame_height
                ],
                &Line::new(if self.hover_goto_start { red } else { blue }, 0.5),
                c, g
            );

            let at_end: bool = end_frame >= self.frames;
            drawutils::draw_goto_end(
                at_end,
                [
                    self.bounds[0] as f64 + self.bounds[2] as f64
                        - right_to_goto_end - frame_width,
                    self.bounds[1] as f64 + top_to_frame
                ],
                [
                    frame_width * 0.5,
                    frame_height
                ],
                &Line::new(if self.hover_goto_end { red } else { blue }, 0.5),
                c, g
            );
        }
    }

    pub fn draw_drag<G: Graphics>(&self, c: &Context, g: &mut G) {

    }
}
