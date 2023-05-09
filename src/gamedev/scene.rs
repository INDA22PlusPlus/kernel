use crate::gamedev::entity::Entity;
use crate::gamedev::tilemap::TileMap;

pub enum Scene {
    D1(WalkingArea),                // Most likely not going to be used
    Outside(Outside),
    Meta(Meta),

    Test(WalkingArea)               // Debug scene
}

pub enum Outside {
    BorgGarden(WalkingArea),
    OsquarsBacke(FightingArea),
    MetaRamp(WalkingArea),
    MetaOutisde(GymArea)            // May be combined with meta ramp
}

pub enum Meta {
    MainArea(WalkingArea),
    CounterArea(FightingArea)
}

pub struct Area {
    bg: TileMap,
    entities: [Entity]
}

pub struct WalkingArea {}

pub struct FightingArea {}

pub struct GymArea {}

// pub enum Area {
//     WalkingArea,
//     FightingArea,
//     GymArea
// }