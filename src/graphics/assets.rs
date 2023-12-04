//From Bevy roguelike tutorial - https://maciejglowka.com/blog/bevy-roguelike-tutorial-devlog-part-1/
use bevy::prelude::*;

use super::GraphicsAssets;

const ATLAS_PATH: &str = "ascii.png";

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut asset_list: ResMut<crate::assets::AssetList>,
) {
    let texture = asset_server.load(ATLAS_PATH);
    asset_list.0.push(texture.clone_untyped());
    let atlas = TextureAtlas::from_grid(texture, Vec2::splat(10.), 16, 16, None, None);
    let handle = texture_atlases.add(atlas);
    commands.insert_resource(GraphicsAssets {
        sprite_texture: handle,
    });
}
