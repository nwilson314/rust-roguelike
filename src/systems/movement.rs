use crate::prelude::*;

pub fn movement(
    mut commands: Commands,
    mut wants_move: Query<(Entity, &mut Transform, &WantsToMove)>,
    mb: Res<MapBuilder>,
) {
    for (ent, mut transform, move_intent) in wants_move.iter_mut() {
        if mb.map.can_enter_tile(move_intent.destination) {
            let (new_x, new_y) = convert_pos(move_intent.destination.x, move_intent.destination.y);
            transform.translation.x = new_x;
            transform.translation.y = new_y;
        }

        commands.entity(ent).remove::<WantsToMove>();
    }
}
