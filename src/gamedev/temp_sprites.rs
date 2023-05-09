use crate::gamedev::binnary_sprites::get_ovve_outline;
use crate::graph::surface::Surface;
use crate::graph::utils::{ColorCode, u8_buf_to_ColorCode};
use crate::math::vec2::Vec2;

// pub fn get_sprite_ovve_outline() {
//     let mut buf_u8: [u8; 256] = get_ovve_outline();
//     // let mut buf = u8_buf_to_ColorCode(buf_u8.as_mut_ptr(), &size, scale);
//
//     ///
//     let mut scale: usize = 1;
//     let mut buf = u8_buf_to_ColorCode(buf_u8.as_mut_ptr(), &size, scale);
//
//     let mut sprite_1x = Surface::from_buffer(buf,
//                                              size.y * scale,
//                                              size.x * scale,
//                                              Some(ColorCode::BrightWhite));
//     sprite_1x.set_origin(Vec2::<usize>::new(100, 100));
// }