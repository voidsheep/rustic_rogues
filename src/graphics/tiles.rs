//From Bevy roguelike tutorial - https://maciejglowka.com/blog/bevy-roguelike-tutorial-devlog-part-1/
use bevy::prelude::*;

use super::{GraphicsAssets, TILE_SIZE, TILE_Z};
use crate::board::components::{Position, Tile, Wall};

pub fn spawn_tile_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position), Added<Tile>>,
    assets: Res<GraphicsAssets>,
) {
    for (entity, position) in query.iter() {
        let mut sprite = TextureAtlasSprite::new(177);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        sprite.color = Color::MIDNIGHT_BLUE;
        let v = super::get_world_position(position, TILE_Z);
        commands.entity(entity).insert(SpriteSheetBundle {
            sprite,
            texture_atlas: assets.sprite_texture.clone(),
            transform: Transform::from_translation(v),
            ..Default::default()
        });
    }
}

//inspired by code above
pub fn spawn_wall_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position), Added<Wall>>,
    assets: Res<GraphicsAssets>,
) {
    for (entity, position) in query.iter() {
        let mut sprite = TextureAtlasSprite::new(219);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        sprite.color = Color::TEAL;
        let v = super::get_world_position(position, 1.0);
        commands.entity(entity).insert(SpriteSheetBundle {
            sprite,
            texture_atlas: assets.sprite_texture.clone(),
            transform: Transform::from_translation(v),
            ..Default::default()
        });
    }
}
