use crate::gamedev::dialogue::Dialogue;
use crate::gamedev::path::Path;
use crate::gamedev::player::Player;
use crate::gamedev::scene::GymArea;
use crate::gamedev::sprite::Sprite;
use crate::math::vec2::Vec2;

pub enum Entity {
    Player(Player),
    NPC(NPC),
    Catchable(Catchable),
    Trainer(Trainer),
    GymLeader(GymLeader)
}

pub struct BasicEntity {
    name: str,
    sprite: Sprite,
    pos: Vec2<usize>
}

pub struct NPC {
    base: BasicEntity,
    path: Option<Path>,
    dialogue: Option<Dialogue>,
}

pub struct Catchable {}

pub struct Trainer {}

pub struct GymLeader {}