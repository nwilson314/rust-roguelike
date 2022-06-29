use crate::prelude::*;

pub fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<
        (&mut Transform, &OrthographicProjection),
        (With<MainCamera>, Without<Player>),
    >,
) {
    let player_transform = player_query.single();
    let (mut camera, _projection) = camera_query.single_mut();

    // TODO: Update so that the camera only follows the player if it doesn't go "out of bounds"
    camera.translation.x = player_transform.translation.x;
    camera.translation.y = player_transform.translation.y;
}