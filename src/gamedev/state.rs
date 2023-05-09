enum State {
    Walking,
    Fighting,
    CutScene,
    Inventory,              // Less priority
    Pokemon,
    Menu(MenuOptions),
    // Settings(Settings)             // Dependent on how menu will be realized

    Test,                   // Debug states
}

enum MenuOptions {
    MainMenu,
    Settings(Settings),
    Save(Save),
    Load(Load),

}

struct Settings {}

struct Save {}

struct Load {}