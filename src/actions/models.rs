//From Bevy roguelike tutorial - https://maciejglowka.com/blog/bevy-roguelike-tutorial-devlog-part-1/
use bevy::prelude::*;

use crate::board::{components::Position, CurrentBoard};
use crate::vectors::Vector2Int;

use super::Action;

pub struct WalkAction(pub Entity, pub Vector2Int);
impl Action for WalkAction {
    fn execute(&self, world: &mut World) -> bool {
        let Some(board) = world.get_resource::<CurrentBoard>() else {
            return false;
        };
        if !board.tiles.contains_key(&self.1) {
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
pub struct Attack{
    pub attacker: Entity,
    pub defender: Entity,
    pub damage: u8
}
impl Attack for Action {
    //check if attacker is next to defender
    //check if defender can be damaged

}

