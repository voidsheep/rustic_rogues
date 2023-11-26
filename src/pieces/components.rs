//From Bevy roguelike tutorial - https://maciejglowka.com/blog/bevy-roguelike-tutorial-devlog-part-1/
use bevy::prelude::*;

use crate::actions::Action;

#[derive(Component, Default)]
pub struct Actor(pub Option<Box<dyn Action>>);

#[derive(Component)]
pub struct Piece {
    pub kind: String,
}

#[derive(Component)]
// movement behaviour for non-player pieces
pub struct Walk;
//- - - - -

#[derive(Component)]
pub struct Health {
    pub max_hp: i8,
    pub hp: i8,
}

//#[derive(Component)]
