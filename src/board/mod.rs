//From Bevy roguelike tutorial - https://maciejglowka.com/blog/bevy-roguelike-tutorial-devlog-part-1/
use bevy::prelude::*;
use std::collections::HashMap;

use crate::states::MainState;
use crate::vectors::Vector2Int;

pub mod components;
mod systems;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentBoard>()
            .add_system(systems::spawn_map.in_schedule(OnEnter(MainState::Game)));
    }
}

#[derive(Default, Resource)]
pub struct CurrentBoard {
    pub tiles: HashMap<Vector2Int, Entity>,
    pub walls: HashMap<Vector2Int, Entity>,
}
