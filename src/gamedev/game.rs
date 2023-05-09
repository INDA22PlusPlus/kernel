struct Game {
    scene: Scene
}

enum Scene {
    D1(WalkingArea),                // Most likely not going to be used
    Outside(Outside),
    Meta(Meta)
}

enum Outside {
    BorgGarden(WalkingArea),
    OsquarsBacke(FightingArea),
    MetaRamp(WalkingArea),
    MetaOutisde(GymArea)            // May be combined with meta ramp
}

enum Meta {
    MainArea(WalkingArea),
    CounterArea(FightingArea)
}

struct WalkingArea {}

struct FightingArea {}

struct GymArea {}

// enum Area {
//     WalkingArea,
//     FightingArea,
//     GymArea
// }