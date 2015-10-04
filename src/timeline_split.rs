//! Timeline split.

#![deny(missing_docs)]

use rect::Rect;

/// The input data for calculating the timeline split layout.
pub struct TimelineSplitInput {
    /// The bounds rectangle of the whole timeline.
    pub rect: [f64; 4],
    /// The margin from edge of bounds and at the splits.
    pub margin: f64,
    /// The size of the left panel.
    pub left: f64,
    /// The size of the right panel.
    pub right: f64,
    /// The height of items in the middle panel.
    pub top: f64,
}

/// The output data for timeline split layout.
pub struct TimelineSplitOutput {
    /// The bounds of the left panel.
    pub left: [f64; 4],
    /// The bounds of the middle panel.
    pub middle: [f64; 4],
    /// The bounds of the right panel.
    pub right: [f64; 4],
}

/// Splits rectangle horizontally into a left, middle and right rectangles.
/// The margin shrinks the middle rectangle.
fn split_left_right_margin(
    rect: [f64; 4],
    left: f64,
    right: f64,
    margin: f64
) -> ([f64; 4], [f64; 4], [f64; 4]) {
    let a_third = 1.0 / 3.0 * rect.w();
    let left = if left > a_third { a_third } else { left };
    let right = if right > a_third { a_third } else { left };
    let middle = rect.w() - left - right;
    let left_rect = [rect.x(), rect.y(), left, rect.h()];
    let right_rect = [rect.x() + left + middle, rect.y(), right, rect.h()];
    let margin = if middle < 2.0 * margin { 0.5 * middle } else { margin };
    (left_rect,
     [rect.x() + left + margin, rect.y(), middle - 2.0 * margin, rect.h()],
     right_rect)
}

impl TimelineSplitInput {
    /// Computes the timeline split output.
    pub fn call(&self) -> Option<TimelineSplitOutput> {
        let inside = self.rect.margin(self.margin);
        // Use the same size of goto start and goto end buttons as the height
        // of frames.
        let rest = if inside.h() < self.top {
                0.0
            } else {
                inside.h() - self.top
            };
        let (left, middle, right) = split_left_right_margin(
            inside, rest, rest, self.margin
        );
        Some(TimelineSplitOutput {
            left: left,
            middle: middle,
            right: right
        })
    }
}
