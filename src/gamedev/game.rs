use core::ops::Add;
use crate::gamedev::binary_sprites::get_ovve_outline;
use crate::gamedev::camera::{CameraStationary, CameraType};
use crate::gamedev::entity::BasicEntity;
use crate::gamedev::fighter::Fighter;
use crate::gamedev::player::Player;
use crate::gamedev::scene::{FightingArea, Scene, WalkingArea};
use crate::gamedev::sprite::Sprite;
use crate::gamedev::state::State;
use crate::gamedev::team::Team;
use crate::gamedev::temp_sprites::get_sprite_from_texture;
use crate::graph::font_writer::FontWriter;
use crate::graph::{font_data, planar_writer};
use crate::graph::planar_writer::VgaPlanarWriter;
use crate::graph::utils::ColorCode;
use crate::math::vec2::Vec2;
use crate::mem::alloc::kalloc;

pub struct Game {
    scene: Scene,
    state: State,
    debug_info: bool,
    player: Player,
    camera: CameraType,

    // palette                      // Low prio. Add or use the implemented one?
    // loaded_textures: [*mut u8],
    writer: VgaPlanarWriter,
    font_writer: FontWriter,
}

impl Game {
    pub fn create_test_game() -> Self {
        let mut font_writer = FontWriter::new(font_data::BASIC_FONT);
        font_writer.load_text_color(ColorCode::Green, Some(ColorCode::Black));

        // let mut textures: [*mut u8] = *[];
        // textures[0].add()

        let ovve_outline_texture = get_ovve_outline();
        Game {
            scene: Scene::Test(WalkingArea{}),
            state: State::Walking,
            debug_info: true,
            player: Player{ base: BasicEntity {
                name: "Dima",
                sprite: Sprite::new(
                    ovve_outline_texture,
                    &Vec2::<usize>::new(100, 100),
                    &Vec2::<usize>::new(16, 16),
                    2,
                )
            }, team: Team {
                fighting_team: [None; 6],
                collection: [None; 100],
            }},
            camera: CameraType::Stationary(CameraStationary{}),
            // loaded_textures: ,
            writer: planar_writer::VgaPlanarWriter::new(),
            font_writer,
        }
    }

    pub fn game_loop(&self) {

    }
}