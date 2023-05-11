use crate::gamedev::entity::BasicEntity;
use crate::gamedev::sprite::Sprite;
use crate::gamedev::team::Team;
use crate::math::vec2::Vec2;

// Todo: Fix pub
pub struct Player {
    pub base: BasicEntity,
    // inventory: Inventory,            // Low priority
    pub team: Team                          // Pokemon collection equivalent
}

impl Player {
    fn step(&mut self) {
        // Temp for testing
        self.base.pos += Vec2::<usize>::new(0, 0);
    }
}