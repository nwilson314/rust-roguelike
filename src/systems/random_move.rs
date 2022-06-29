use crate::prelude::*;

pub fn random_move(
    mut commands: Commands,
    mut enemies: Query<(Entity, &mut Position), (With<MovingRandomly>, With<Enemy>)>,
    mb: Res<MapBuilder>,
) {
    let mut rng = RandomNumberGenerator::new();

    for (ent, mut enemy) in enemies.iter_mut() {
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + enemy.position;

        if mb.map.can_enter_tile(destination) {
            enemy.position = destination;
            commands.entity(ent).insert(WantsToMove {
                destination: destination,
            });
        }
    }
}
