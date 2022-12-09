#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    Menu,
    Loading,
    Game,
    Running,
    Lost,
    Won,
}
