use crate::gamedev::temp_sprites::get_sprite_from_texture;
use crate::graph::surface::Surface;
use crate::graph::utils::ColorCode;
use crate::math::vec2::Vec2;

// TODO: fix ignore_color
pub struct Sprite {
    surface: Surface,
    logical_size: Vec2<usize>,
    screen_size: Vec2<usize>,
    scale: usize,
    pos: Vec2<usize>
}

impl Sprite {
    pub fn new(texture: *mut u8, pos: &Vec2<usize>, size: &Vec2<usize>, scale: usize) -> Self {
        Sprite {
            surface: get_sprite_from_texture(texture,
                                             &size,
                                             pos,
                                             scale,
                                             ColorCode::BrightWhite
            ),
            logical_size: *size,
            screen_size: *size * scale,
            scale,
            pos: *pos,
        }
    }
}

// impl Sprite {
//     pub fn from_fn(fn)
// }