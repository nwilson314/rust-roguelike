use crate::prelude::*;

pub fn spawn_player(mut commands: Commands, tile_sheet: Res<FontSpriteSheet>, mb: Res<MapBuilder>) {
    let (tile_width, tile_height) = get_windowed_tile_size();
    let (pos_x, pos_y) = convert_pos(mb.player_start.x, mb.player_start.y);
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
        .insert(Player {
            position: mb.player_start,
        });
}

pub fn get_input(
    mut player_query: Query<(&mut Transform, &mut Player)>,
    keyboard: Res<bevy::prelude::Input<KeyCode>>,
    mb: Res<MapBuilder>,
    mut camera_query: Query<
        (&mut Transform, &OrthographicProjection),
        (With<MainCamera>, Without<Player>),
    >,
) {
    let (mut player_transform, mut player) = player_query.single_mut();
    let (mut camera, _projection) = camera_query.single_mut();
    let mut new_pos = player.position;
    if keyboard.just_pressed(KeyCode::W) | keyboard.just_pressed(KeyCode::Up) {
        new_pos.y += 1;
    } else if keyboard.just_pressed(KeyCode::S) | keyboard.just_pressed(KeyCode::Down) {
        new_pos.y -= 1;
    } else if keyboard.just_pressed(KeyCode::A) | keyboard.just_pressed(KeyCode::Left) {
        new_pos.x -= 1;
    } else if keyboard.just_pressed(KeyCode::D) | keyboard.just_pressed(KeyCode::Right) {
        new_pos.x += 1;
    }

    if mb.map.can_enter_tile(new_pos) {
        // let old_pos = player.position;
        player.position = new_pos;
        let (new_x, new_y) = convert_pos(new_pos.x, new_pos.y);
        player_transform.translation.x = new_x;
        player_transform.translation.y = new_y;

        // TODO: Update so that the camera only follows the player if it doesn't go "out of bounds"
        camera.translation.x = new_x;
        camera.translation.y = new_y;
    }
}
