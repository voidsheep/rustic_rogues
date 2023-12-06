//modified code from Bevy roguelike tutorial - https://maciejglowka.com/blog/bevy-roguelike-tutorial-devlog-part-1/
use bevy::prelude::*;
use rand::prelude::*;
use std::collections::HashMap;

use crate::vectors::Vector2Int;

use super::components::{Position, Tile, Wall};
use super::CurrentBoard;

pub fn spawn_map(mut commands: Commands, mut current: ResMut<CurrentBoard>) {
    current.tiles = HashMap::new();
    current.walls = HashMap::new();

    let mut rng = thread_rng();
    let mut wall_chance: u32;

    //spawn floor
    for i in 0..16 {
        for j in 0..12 {
            let v = Vector2Int::new(i, j);
            let tile = commands.spawn((Position { v }, Tile)).id();
            current.tiles.insert(v, tile);
        }
    }
    //spawn walls
    for k in 0..16 {
        for l in 0..12 {
            if k == 0 && (l == 0 || l == 1) {
                continue;
            }
            wall_chance = rng.gen_range(1..10);

            if wall_chance == 1 {
                let v = Vector2Int::new(k, l);
                let wall = commands.spawn((Position { v }, Wall)).id();
                current.walls.insert(v, wall);
            }
        }
    }
}
