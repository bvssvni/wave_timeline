/// Represents a rectangle box with something to draw in.
/// Used to compute the layout.
pub struct RectangleBox {
    /// The name of the box.
    pub name: &'static str,
    /// The margin to the children.
    pub margin: f64,
    /// The child.
    pub child: Child,
}

/// A child is inside a rectangle box.
pub enum Child {
    /// Draw something.
    Draw,
    /// Split into left, middle and right sections.
    LeftMiddleRight(Box<LeftMiddleRight>),
}

/// Splits into left, middle and right sections.
/// The middle is stretched.
pub struct LeftMiddleRight {
    /// The position of the left split.
    pub left_split: f64,
    /// The position of the right split.
    pub right_split: f64,
    /// The left rectangle box.
    pub left: RectangleBox,
    /// The middle rectangle box.
    pub middle: RectangleBox,
    /// The right rectangle box.
    pub right: RectangleBox,
}
