use glam::Vec2;

use crate::geometry::{Line, Triangle};

/// Get either width or height, depending on which is longer.
pub fn diagonal_distance(from: Vec2, to: Vec2) -> f32 {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    dx.abs().max(dy.abs())
}

/*
    Determine if point is within triangle formed by points p1, p2, p3.
    If so, the point will be on the same side of each of the half planes
    defined by vectors p1p2, p2p3, and p3p1.
*/
pub fn point_is_in_triangle(point: Vec2, triangle: &Triangle) -> bool {
    let Triangle { a, b, c } = *triangle;
    let side1 = side_of_the_plane(point, Line::new(a, b));
    let side2 = side_of_the_plane(point, Line::new(b, c));
    let side3 = side_of_the_plane(point, Line::new(c, a));
    side1 && side2 && side3 || !side1 && !side2 && !side3
}

fn side_of_the_plane(point: Vec2, line: Line) -> bool {
    let Line { a, b } = line;
    ((point.x - b.x) * (a.y - b.y) - (a.x - b.x) * (point.y - b.y)).is_sign_positive()
}
