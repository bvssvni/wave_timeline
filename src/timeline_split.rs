//! Timeline split.

#![deny(missing_docs)]

use graphics::*;
use rect::Rect;

const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

struct TimelineSplitSettings {
    rect: [f64; 4],
    margin: f64,
    left: f64,
    right: f64,
    top: f64,
}

struct TimelineSplitLayout {
    left: [f64; 4],
    middle: [f64; 4],
    right: [f64; 4],
}

impl TimelineSplitSettings {
    fn call(&self) -> Option<TimelineSplitLayout> {
        let inside = self.rect.margin(self.margin);
        // Use the same size of goto start and goto end buttons as the height
        // of frames.
        let rest = if inside.h() < self.top {
                0.0
            } else {
                inside.h() - self.top
            };
        let (left, middle, right) = inside.split_left_right_margin(
            rest, rest, self.margin
        );
        Some(TimelineSplitLayout {
            left: left,
            middle: middle,
            right: right
        })
    }
}

fn draw_box<G: Graphics>(color: [f32; 4], rect: [f64; 4], c: &Context, g: &mut G) {
    Rectangle::new_border(color, 0.5)
        .draw(rect, &c.draw_state, c.transform, g);
}

/// Test drawing for timeline split.
pub fn test_draw_timeline<G: Graphics>(rect: [u32; 4], c: &Context, g: &mut G) {
    let top = 20.0;
    let top_factor = 0.3;
    if let Some(layout) =
        (TimelineSplitSettings {
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
