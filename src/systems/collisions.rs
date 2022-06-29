use crate::prelude::*;

pub fn collisions(
    mut commands: Commands,
    enemies_query: Query<(&Position, Entity), With<Enemy>>,
    player_query: Query<&Position, (With<Player>, Without<Enemy>)>,
) {
    let player = player_query.single();
    for (enemy, ent) in enemies_query.iter() {
        if player.position.x == enemy.position.x && player.position.y == enemy.position.y {
            commands.entity(ent).despawn_recursive();
        }
    }
}
