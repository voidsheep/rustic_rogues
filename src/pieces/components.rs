//From Bevy roguelike tutorial - https://maciejglowka.com/blog/bevy-roguelike-tutorial-devlog-part-1/
use bevy::prelude::*;

use crate::actions::Action;

#[derive(Component, Default)]
pub struct Actor(pub Vec<Box<dyn Action>>, i32);

#[derive(Component)]
pub struct Piece {
    pub kind: String,
}

#[derive(Component)]
// movement behaviour for non-player pieces
pub struct Walk;

//- - - - - - - -
//My Components
//- - - - - - - -

#[derive(Component)]
pub struct Health {
    pub max_hp: u8,
    pub hp: u8,
}
