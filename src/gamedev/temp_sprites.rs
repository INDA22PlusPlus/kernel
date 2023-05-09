use crate::gamedev::binnary_sprites::get_ovve_outline;
use crate::graph::surface::Surface;
use crate::graph::utils::{ColorCode, u8_buf_to_ColorCode};
use crate::math::vec2::Vec2;

pub fn get_sprite_ovve_outline(
    // func: fn() -> *mut u8,
    buf_u8: *mut u8,
    size: &Vec2<usize>,
    origin: &Vec2<usize>,
    scale: usize,
    ignore_color: ColorCode
) -> Surface {
    // let mut buf_u8 = func();
    let mut buf = u8_buf_to_ColorCode(buf_u8, &size, scale);
    let mut sprite = Surface::from_buffer(buf,
                                             size.y * scale,
                                             size.x * scale,
                                          Some(ignore_color));
    sprite.set_origin(Vec2::<usize>::new(origin.y, origin.x));
    sprite
}