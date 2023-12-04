//- - - - - - - -
//My Functions
//- - - - - - - -
use bevy::prelude::*;

use super::{GraphicsAssets, TILE_SIZE};

use crate::player::Player;
use crate::pieces::components::Health;

///spawns ui elements
pub fn spawn_ui(mut commands: Commands, ascii: Res<GraphicsAssets>, health_query: Query<&Health, With<Player>>) {
    let Ok(player_health) = health_query.get_single() else { return;};
    let curr_hearts = player_health.hp;

    //More hearts should be generated if health upgrades are expected
    let mut hearts: [TextureAtlasSprite; 3] = [ 
        TextureAtlasSprite::new(3),
        TextureAtlasSprite::new(3),
        TextureAtlasSprite::new(3),
    ];

    for i in 0..curr_hearts as usize {
        hearts[i].color = Color::rgb(1.0, 0.0, 0.0);
        hearts[i].custom_size = Some(Vec2::splat(TILE_SIZE));
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