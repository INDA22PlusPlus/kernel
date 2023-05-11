use core::borrow::Borrow;
use crate::gamedev::sprite::OnScreen::OffScreen;
use crate::gamedev::temp_sprites::get_sprite_from_texture;
use crate::graph::surface::Surface;
use crate::graph::utils::ColorCode;
use crate::math::vec2::Vec2;

pub enum OnScreen {
    OnScreen{ screen_pos: Vec2<usize>, offset: Vec2<usize> },
    OffScreen
}

// TODO: fix ignore_color
pub struct Sprite {
    surface: Surface,
    logical_size: Vec2<usize>,
    screen_size: Vec2<usize>,
    scale: usize,
    on_screen: OnScreen
}

impl Sprite {
    pub fn new(texture: *mut u8, size: &Vec2<usize>, scale: usize, on_screen: OnScreen) -> Self {
        Sprite {
            surface: get_sprite_from_texture(texture,
                                             &size,
                                             pos,
                                             scale,
                                             ColorCode::BrightWhite
            ),
            logical_size: size.borrow(),
            screen_size: size * scale,
            scale,
            on_screen
        }
    }
}

// impl Sprite {
//     pub fn from_fn(fn)
// }