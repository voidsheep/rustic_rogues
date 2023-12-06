//Modified from Bevy roguelike tutorial - https://maciejglowka.com/blog/bevy-roguelike-tutorial-devlog-part-1/
use bevy::prelude::*;

use crate::board::{components::Position, CurrentBoard};
use crate::pieces::components::Health;
use crate::vectors::Vector2Int;

use super::Action;

pub struct WalkAction(pub Entity, pub Vector2Int);
impl Action for WalkAction {
    fn execute(&self, world: &mut World) -> bool {
        let Some(board) = world.get_resource::<CurrentBoard>() else {
            return false;
        };
        if (board.walls.contains_key(&self.1)) || (!board.tiles.contains_key(&self.1))  {
            return false;
        };

        let Some(mut position) = world.get_mut::<Position>(self.0) else {
            return false;
        };
        position.v = self.1;
        true
    }
}
//- - - - - - - -
//My Functions
//- - - - - - - -
pub struct Attack {
    pub attacker: Entity,
    pub defender: Entity,
    pub damage: u8,
}
impl Action for Attack {
    fn execute(&self, world: &mut World) -> bool {
        let Some(atk_pos) = world.get::<Position>(self.attacker) else {
            return false;
        };
        let Some(def_pos) = world.get::<Position>(self.defender) else {
            return false;
        };
        let distance_x = (atk_pos.v.x - def_pos.v.x).abs();
        let distance_y = (atk_pos.v.y - def_pos.v.y).abs();

        if (distance_x != 1 || distance_y != 0) && (distance_x != 0 || distance_y != 1) {
            return false;
        }
        info!("Hit for {} damage!", self.damage);
        //deal damage
        let Some(mut def_health) = world.get_mut::<Health>(self.defender) else {
            return false;
        };
        def_health.hp = def_health.hp - self.damage;
        if def_health.hp <= 0 {
            world.despawn(self.defender);
        }
        true
    }
}

#[test]
fn test_hit(){
    let mut world = World::new();

 let attacker = world.spawn((
        Position {
            v: Vector2Int::new(0, 0),
        },
        Health { max_hp: 3, hp: 3 },
    )).id();

    let defender = world.spawn((
        Position {
            v: Vector2Int::new(0, 1),
        },
        Health { max_hp: 6, hp: 6 },
    )).id();

    let action = Attack {
        attacker: attacker, //player
        defender: defender, //NPC
        damage: 2,
    };
    action.execute(&mut world);
    
    let Some(npc_health) = world.get::<Health>(defender) else {return;};
    let npc_curr = npc_health.hp;
    
    assert_eq!(npc_curr, 4)
}

#[test]
fn test_miss(){
    let mut world = World::new();

 let attacker = world.spawn((
        Position {
            v: Vector2Int::new(0, 0),
        },
    )).id();

    let defender = world.spawn((
        Position {
            v: Vector2Int::new(5, 5),
        },
    )).id();

    let action = Attack {
        attacker: attacker, //player
        defender: defender, //NPC
        damage: 2,
    };
    assert_eq!(false, action.execute(&mut world)) 
}