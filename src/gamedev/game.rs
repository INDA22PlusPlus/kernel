use crate::gamedev::camera::Camera;
use crate::gamedev::player::Player;
use crate::gamedev::scene::Scene;
use crate::gamedev::state::State;

struct Game {
    scene: Scene,
    state: State,
    debug_info: bool,
    player: Player,
    camera: Camera,
}