use graphics::*;

const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

fn margin(rect: [u32; 4], val: u32) -> Option<[u32; 4]> {
    if rect[2] < 2 * val || rect[3] < 2 * val { None }
    else {
        Some([
            rect[0] + val,
            rect[1] + val,
            rect[2] - 2 * val,
            rect[3] - 2 * val
        ])
    }
}

struct SplitLeftMiddleRight {
    left: u32,
    right: u32,
    margin: u32,
}

fn split_left_middle_right(
    split: &SplitLeftMiddleRight,
    rect: [u32; 4],
) -> Option<([u32; 4], [u32; 4], [u32; 4])> {
    let sub_middle_width = split.left + split.right + 2 * split.margin;
    if rect[2] < sub_middle_width { return None; }
    let left = [rect[0], rect[1], split.left, rect[3]];
    let right = [rect[0] + rect[2] - split.right, rect[1],
                 split.right, rect[3]];
    let middle = [rect[0] + split.left + split.margin, rect[1],
                  rect[2] - sub_middle_width, rect[3]];
    Some((left, middle, right))
}

struct TimelineSettings {
    rect: [u32; 4],
    margin: u32,
    left: u32,
    right: u32,
    top: u32,
}

struct TimelineLayout {
    left: [u32; 4],
    middle: [u32; 4],
    right: [u32; 4],
}

impl TimelineSettings {
    fn call(&self) -> Option<TimelineLayout> {
        if let Some(inside) = margin(self.rect, self.margin) {
            let rest = if inside[3] < self.top {
                    0
                } else {
                    inside[3] - self.top
                };
            if let Some((left, middle, right)) = split_left_middle_right(
                &SplitLeftMiddleRight {
                    left: rest, // self.left,
                    right: rest, // self.right,
                    margin: self.margin
                },
                inside
            ) {
                Some(TimelineLayout {
                    left: left,
                    middle: middle,
                    right: right
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn cap_top(rect: [u32; 4], top: u32) -> Option<[u32; 4]> {
    if rect[3] < top { None }
    else {
        Some([rect[0], rect[1] + top, rect[2], rect[3] - top])
    }
}

fn draw_box<G: Graphics>(color: [f32; 4], rect: [u32; 4], c: &Context, g: &mut G) {
    let rect = [rect[0] as f64, rect[1] as f64, rect[2] as f64, rect[3] as f64];
    Rectangle::new_border(color, 0.5)
        .draw(rect, &c.draw_state, c.transform, g);
}

pub fn draw_timeline<G: Graphics>(rect: [u32; 4], c: &Context, g: &mut G) {
    let top = 20;
    if let Some(layout) =
        (TimelineSettings {
            rect: rect,
            margin: 4,
            left: 40,
            right: 40,
            top: top,
        }).call() {

        draw_box(BLUE, layout.left, c, g);
        draw_box(BLUE, layout.middle, c, g);
        draw_box(BLUE, layout.right, c, g);

        if let Some(left) = cap_top(layout.left, top) {
            draw_box(RED, left, c, g);
        }
        if let Some(middle) = cap_top(layout.middle, top) {
            draw_box(RED, middle, c, g);
        }
        if let Some(right) = cap_top(layout.right, top) {
            draw_box(RED, right, c, g);
        }
    }
}
