use crate::prelude::*;

pub fn spawn_player(mut commands: Commands, tile_sheet: Res<FontSpriteSheet>) {
    let (tile_width, tile_height) = get_windowed_tile_size();
    let (pos_x, pos_y) = convert_pos(0, 0);
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: to_cp437('@') as usize,
                custom_size: Some(Vec2::new(tile_width, tile_height)),
                ..TextureAtlasSprite::default()
            },
            texture_atlas: tile_sheet.atlas.clone(),
            transform: Transform::from_xyz(pos_x, pos_y, 0.0),
            ..Default::default()
        })
        .insert(Player);
}

pub fn get_input(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard: Res<bevy::prelude::Input<KeyCode>>,
) {
    let mut player_transform = player_query.single_mut();
    let mut velo = vec![0; 2];

    if keyboard.just_pressed(KeyCode::W) | keyboard.just_pressed(KeyCode::Up) {
        velo[1] += 1;
    } else if keyboard.just_pressed(KeyCode::S) | keyboard.just_pressed(KeyCode::Down) {
        velo[1] -= 1;
    } else if keyboard.just_pressed(KeyCode::A) | keyboard.just_pressed(KeyCode::Left) {
        velo[0] -= 1;
    } else if keyboard.just_pressed(KeyCode::D) | keyboard.just_pressed(KeyCode::Right) {
        velo[0] += 1;
    }
    let (vel_x, vel_y) = convert_movement(velo[0], velo[1]);
    player_transform.translation.x += vel_x;
    player_transform.translation.y += vel_y;
}
