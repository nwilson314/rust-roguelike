use crate::prelude::*;

pub fn spawn_player(mut commands: Commands, tile_sheet: Res<FontSpriteSheet>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: to_cp437('@') as usize,
                ..TextureAtlasSprite::default()
            },
            texture_atlas: tile_sheet.atlas.clone(),
            ..Default::default()
        })
        .insert(Player);
}

pub fn get_input(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard: Res<bevy::prelude::Input<KeyCode>>,
) {
    let mut player_transform = player_query.single_mut();

    if keyboard.just_pressed(KeyCode::W) | keyboard.just_pressed(KeyCode::Up) {
        player_transform.translation.y += TILE_SIZE as f32;
    } else if keyboard.just_pressed(KeyCode::S) | keyboard.just_pressed(KeyCode::Down) {
        player_transform.translation.y -= TILE_SIZE as f32;
    } else if keyboard.just_pressed(KeyCode::A) | keyboard.just_pressed(KeyCode::Left) {
        player_transform.translation.x -= TILE_SIZE as f32;
    } else if keyboard.just_pressed(KeyCode::D) | keyboard.just_pressed(KeyCode::Right) {
        player_transform.translation.x += TILE_SIZE as f32;
    }
}
