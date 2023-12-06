//modified code from Bevy roguelike tutorial - https://maciejglowka.com/blog/bevy-roguelike-tutorial-devlog-part-1/
use bevy::prelude::*;
use rand::prelude::*;

use crate::board::{components::Position, CurrentBoard};
use crate::states::MainState;
use crate::vectors::Vector2Int;

pub mod components;

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_npcs.in_schedule(OnEnter(MainState::Game)));
    }
}

pub fn spawn_npcs(mut commands: Commands, world: &World) {
    let Some(board) = world.get_resource::<CurrentBoard>() else {
        return;
    };
    let mut rng = thread_rng();
    let max_npcs = 6;
    let mut curr_npcs = 1;
    let mut spawn_chance: u32;

    //for testing purposes, this npc should always spawn
    commands.spawn((
        components::Actor::default(),
        components::Piece {
            kind: "NPC".to_string(),
        },
        Position {
            v: Vector2Int::new(3, 5),
        },
        components::Health { max_hp: 3, hp: 3 },
        components::Walk,
    ));

    for x in 0..16 {
        for y in 0..12 {
            if x == 0 && (y == 0 || y == 1) {
                continue;
            }

            //coords of test enemy
            if x == 3 && y == 5 {
                continue;
            }

            if board.walls.contains_key(&Vector2Int { x: (x), y: (y) }) {
                continue;
            };

            if curr_npcs == max_npcs {
                continue;
            }

            spawn_chance = rng.gen_range(1..30);
            if spawn_chance == 1 {
                commands.spawn((
                    components::Actor::default(),
                    components::Piece {
                        kind: "NPC".to_string(),
                    },
                    Position {
                        v: Vector2Int::new(x, y),
                    },
                    components::Health { max_hp: 3, hp: 3 },
                    components::Walk,
                ));
                curr_npcs += 1;
            }
        }
    }
}
