use crate::gamedev::binnary_sprites::get_ovve_outline;
use crate::graph::surface::Surface;
use crate::graph::utils::{ColorCode, u8_buf_to_ColorCode};
use crate::math::vec2::Vec2;

pub fn get_sprite_ovve_outline(size: Vec2<usize>, scale: usize) -> Surface {
    let mut buf_u8 = get_ovve_outline();
    let mut buf = u8_buf_to_ColorCode(buf_u8, &size, scale);
    let mut sprite = Surface::from_buffer(buf,
                                             size.y * scale,
                                             size.x * scale,
                                             Some(ColorCode::BrightWhite));
    sprite.set_origin(Vec2::<usize>::new(100, 100));
    sprite
}