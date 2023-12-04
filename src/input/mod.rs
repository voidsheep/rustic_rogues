use bevy::prelude::*;
use std::collections::VecDeque;

use crate::actions::{models::Attack, models::WalkAction, ActorQueue};
use crate::board::components::Position;
use crate::pieces::components::{Actor, Health};
use crate::player::Player;
use crate::states::GameState;
use crate::vectors::Vector2Int;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInputReadyEvent>()
            .add_system(player_position.in_set(OnUpdate(GameState::PlayerInput)))
            .add_system(player_atk.in_set(OnUpdate(GameState::PlayerInput)));
    }
}
//From Bevy roguelike tutorial - https://maciejglowka.com/blog/bevy-roguelike-tutorial-devlog-part-1/
pub struct PlayerInputReadyEvent;

const DIR_KEY_MAPPING: [(KeyCode, Vector2Int); 4] = [
    (KeyCode::W, Vector2Int::UP),
    (KeyCode::S, Vector2Int::DOWN),
    (KeyCode::A, Vector2Int::LEFT),
    (KeyCode::D, Vector2Int::RIGHT),
];

fn player_position(
    keys: ResMut<Input<KeyCode>>,
    mut player_query: Query<(Entity, &Position, &mut Actor), With<Player>>,
    mut queue: ResMut<ActorQueue>,
    mut ev_input: EventWriter<PlayerInputReadyEvent>,
) {
    let Ok((entity, position, mut actor)) = player_query.get_single_mut() else {
        return;
    };
    for (key, dir) in DIR_KEY_MAPPING {
        if !keys.just_pressed(key) {
            continue;
        }
        let action = WalkAction(entity, position.v + dir);
        actor.0 = Some(Box::new(action));
        queue.0 = VecDeque::from([entity]);
        ev_input.send(PlayerInputReadyEvent);
        info!("WASD Pressed");
    }
}
//- - - - - - - -
//My Functions
//- - - - - - - -
///Detect if an NPC can be attacked. If so, create an attack action and push it
///Only one target can be attacked a turn
fn player_atk(
    keys: ResMut<Input<KeyCode>>,
    mut player_query: Query<(Entity, &mut Actor), With<Player>>,
    mut queue: ResMut<ActorQueue>,
    query_entity: Query<Entity, (With<Health>, Without<Player>)>,
    query_entity_pos: Query<&Position, (With<Health>, Without<Player>)>, //will probably remove without player to make more generic
    query_player_pos: Query<&Position, With<Player>>,
    mut ev_input: EventWriter<PlayerInputReadyEvent>,
) {
    if keys.just_pressed(KeyCode::Q) {
        let Ok((attacker, mut actor)) = player_query.get_single_mut() else {
            return;
        };
        let Ok(player_pos) = query_player_pos.get_single() else {
            info!("error");
            return;
        };

        for defender in &query_entity {
            let Ok(defender_pos) = query_entity_pos.get(defender) else {
                info!("error");
                return;
            };
            let distance_x = (player_pos.v.x - defender_pos.v.x).abs();
            let distance_y = (player_pos.v.y - defender_pos.v.y).abs();

            if (distance_x != 1 || distance_y != 0) && (distance_x != 0 || distance_y != 1) {
                continue;
            }

            let action = Attack {
                attacker: attacker, //player
                defender: defender, //NPC
                damage: 1,
            };
            actor.0 = Some(Box::new(action));
            queue.0 = VecDeque::from([attacker]);
            ev_input.send(PlayerInputReadyEvent);
        }
        info!("Q Pressed");
    }
}
