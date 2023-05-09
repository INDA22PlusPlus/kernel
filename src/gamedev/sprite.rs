use crate::graph::surface::Surface;
use crate::math::vec2::Vec2;

pub struct Sprite {
    surface: Surface,
    logical_size: Vec2<usize>,
    screen_size: Vec2<usize>,
    scale: usize,
}

// impl Sprite {
//     pub fn from_fn(fn)
// }