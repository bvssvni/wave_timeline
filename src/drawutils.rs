use graphics::{ self, Context, Line, Graphics };

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

pub fn draw_goto_end<G: Graphics>(
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
