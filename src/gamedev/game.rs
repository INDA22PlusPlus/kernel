use crate::gamedev::camera::{Camera, CameraType};
use crate::gamedev::player::Player;
use crate::gamedev::scene::Scene;
use crate::gamedev::state::State;

struct Game {
    scene: Scene,
    state: State,
    debug_info: bool,
    player: Player,
    camera: CameraType,
}