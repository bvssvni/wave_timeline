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
        let (left, middle, right) = inside.split_left_right_margin(
            rest, rest, self.margin
        );
        Some(TimelineSplitOutput {
            left: left,
            middle: middle,
            right: right
        })
    }
}
