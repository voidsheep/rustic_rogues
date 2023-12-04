//- - - - - - - -
//My Functions
//- - - - - - - -
use bevy::prelude::*;

use super::{GraphicsAssets, TILE_SIZE};

///spawns ui elements
pub fn spawn_ui(mut commands: Commands, ascii: Res<GraphicsAssets>) {
    let mut hearts: [TextureAtlasSprite; 3] = [
        TextureAtlasSprite::new(3),
        TextureAtlasSprite::new(3),
        TextureAtlasSprite::new(3),
    ];
    hearts[0].color = Color::rgb(1.0, 0.0, 0.0);
    hearts[1].color = Color::rgb(1.0, 0.0, 0.0);
    hearts[2].color = Color::rgb(1.0, 0.0, 0.0);

    hearts[0].custom_size = Some(Vec2::splat(TILE_SIZE));
    hearts[1].custom_size = Some(Vec2::splat(TILE_SIZE));
    hearts[2].custom_size = Some(Vec2::splat(TILE_SIZE));

    for i in 0..3 {
        commands
            .spawn(SpriteSheetBundle {
                sprite: hearts[i].clone(),
                texture_atlas: ascii.sprite_texture.clone(),
                transform: Transform {
                    translation: Vec3::new(i as f32 * 32.0, -64.0, 900.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Name::new("Hearts"));
    }
}