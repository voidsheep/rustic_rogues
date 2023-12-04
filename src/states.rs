//From Bevy roguelike tutorial - https://maciejglowka.com/blog/bevy-roguelike-tutorial-devlog-part-1/
use bevy::prelude::*;

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum MainState {
    #[default]
    LoadAssets,
    Game,
}

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum GameState {
    #[default]
    None,
    PlayerInput,
    TurnUpdate,
}
