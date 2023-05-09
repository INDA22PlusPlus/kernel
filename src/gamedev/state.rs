pub enum State {
    Walking,
    Fighting,
    CutScene,
    Inventory,              // Less priority
    Pokemon,
    Menu(MenuOptions),
    GameOver,
    // Settings(Settings)             // Dependent on how menu will be realized

    Test,                   // Debug states
}

pub enum MenuOptions {
    MainMenu,
    Settings(Settings),
    Save(Save),
    Load(Load),

}

pub struct Settings {}

pub struct Save {}

pub struct Load {}