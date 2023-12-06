//Mine
use bevy::prelude::*;

use super::{GraphicsAssets, TILE_SIZE};

use crate::pieces::components::Health;
use crate::player::Player;

///spawns ui elements
pub fn spawn_ui(
    mut commands: Commands,
    ascii: Res<GraphicsAssets>,
    health_query: Query<&Health, With<Player>>,
) {
    let Ok(player_health) = health_query.get_single() else {
        return;
    };
    let curr_hearts = player_health.hp;

    //More hearts should be generated if health upgrades are expected
    let mut hearts: [TextureAtlasSprite; 3] = [
        TextureAtlasSprite::new(3),
        TextureAtlasSprite::new(3),
        TextureAtlasSprite::new(3),
    ];

    for (i, item) in hearts.iter_mut().enumerate().take(curr_hearts as usize) {
        item.color = Color::rgb(1.0, 0.0, 0.0);
        item.custom_size = Some(Vec2::splat(TILE_SIZE));
        commands
            .spawn(SpriteSheetBundle {
                sprite: item.clone(),
                texture_atlas: ascii.sprite_texture.clone(),
                transform: Transform {
                    translation: Vec3::new(i as f32 * TILE_SIZE, TILE_SIZE * -1.0, 900.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Name::new("Hearts"));
    }
}
