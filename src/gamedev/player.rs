use crate::gamedev::entity::BasicEntity;
use crate::gamedev::sprite::Sprite;
use crate::gamedev::team::Team;

// Todo: Fix pub
pub struct Player {
    pub base: BasicEntity,
    // inventory: Inventory,            // Low priority
    pub team: Team                          // Pokemon collection equivalent
}