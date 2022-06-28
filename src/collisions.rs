use crate::prelude::*;

pub fn check_enemy_player_collisions(
    mut commands: Commands,
    enemies_query: Query<(&Enemy, Entity)>,
    player_query: Query<&Player, Without<Enemy>>, 
) {
    let player = player_query.single();
    for (enemy, ent) in enemies_query.iter() {
        if player.position == enemy.position {
            commands.entity(ent).despawn_recursive();
        }
    }
}