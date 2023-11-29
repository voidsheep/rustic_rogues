use bevy::prelude::*;
use std::collections::VecDeque;

use crate::actions::{models::WalkAction, models::Attack, ActorQueue};
use crate::board::components::Position;
use crate::pieces::components::Actor;
use crate::player::Player;
use crate::states::GameState;
use crate::vectors::Vector2Int;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInputReadyEvent>()
            .add_system(player_position.in_set(OnUpdate(GameState::PlayerInput)));
    }
}

pub struct PlayerInputReadyEvent;

const DIR_KEY_MAPPING: [(KeyCode, Vector2Int); 4] = [
    (KeyCode::W, Vector2Int::UP),
    (KeyCode::S, Vector2Int::DOWN),
    (KeyCode::A, Vector2Int::LEFT),
    (KeyCode::D, Vector2Int::RIGHT),
];

const ACT_KEY_MAPPING: [(KeyCode); 1] = [
    (KeyCode::Q),
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
    //mine
    if keys.pressed(KeyCode::Q){
        let action = Attack{ //need to figure out how to send the enemy
            attacker: entity,
            defender: entity,
            damage: 1,
        };
        actor.0 = Some(Box::new(action));
        queue.0 = VecDeque::from([entity]);
        ev_input.send(PlayerInputReadyEvent);
        info!("Q Pressed");
    }
}
