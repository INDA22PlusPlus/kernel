use crate::gamedev::entity::BasicEntity;
use crate::gamedev::sprite::Sprite;
use crate::gamedev::team::Team;

pub struct Player {
    base: BasicEntity,
    // inventory: Inventory,            // Low priority
    team: Team                          // Pokemon collection equivalent
}