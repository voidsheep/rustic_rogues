use bevy::prelude::*;
use std::collections::VecDeque;

use crate::actions::{models::WalkAction, models::Attack, ActorQueue};
use crate::board::components::Position;
use crate::pieces::components::{Actor, Health};
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
    //mine
    query_entity: Query<Entity, With<Health>>,
    query_entity_pos: Query<&Position, (With<Health>, Without<Player>)>, //will probably remove without player to make more generic 
    query_player_pos: Query<&Position, With<Player>>,
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
      
        queue.0.extend(
            query_entity.iter()
        );
        let Ok(player_position) = query_player_pos.get_single() else { return };

        let Some(entity2) = queue.0.get(0) else {return};

        let Ok(entity2_position) = query_entity_pos.get(*entity2) else {return};
        println!("enemy x pos: {}, enemy y pos: {}", entity2_position.v.x, entity2_position.v.y);

        let distance_x = (player_position.v.x - entity2_position.v.x).abs();
        let distance_y = (player_position.v.y - entity2_position.v.y).abs();

        if (distance_x != 1) && (distance_y != 1){
            println!("x distance: {}, y distance: {}", distance_x, distance_y);
            info!("The attack failed");
            return;
        }
    
        //let entity2 = query_entity.iter() else {return};
        
        let action = Attack{ //need to figure out how to send the enemy
            attacker: entity, //player right now
            defender: *entity2,
            damage: 1,
        };
        actor.0 = Some(Box::new(action));
        queue.0 = VecDeque::from([entity]);
        ev_input.send(PlayerInputReadyEvent);
        info!("Q Pressed");
    }
}
