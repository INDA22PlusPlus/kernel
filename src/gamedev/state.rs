#[derive(PartialEq, Debug)]
pub enum State {
    Walking,
    Fighting,
    CutScene,
    Inventory,
    // Less priority
    Pokemon,
    Menu(MenuOptions),
    GameOver,
    // Settings(Settings)             // Dependent on how menu will be realized

    Test,                   // Debug states
}

#[derive(PartialEq, Debug)]
pub enum MenuOptions {
    MainMenu,
    Settings(Settings),
    Save(Save),
    Load(Load),

}

#[derive(PartialEq, Debug)]
pub struct Settings {}

#[derive(PartialEq, Debug)]
pub struct Save {}

#[derive(PartialEq, Debug)]
pub struct Load {}