use crate::prelude::*;

pub fn get_input(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Player)>,
    mut keyboard: ResMut<bevy::prelude::Input<KeyCode>>,
    mb: Res<MapBuilder>,
    mut turn_state: ResMut<State<TurnState>>,
) {
    let (player_entity, mut player) = player_query.single_mut();

    let mut new_pos = player.position;
    let key = keyboard.get_just_pressed().next().cloned();

    if let Some(key) = key {
        match key {
            KeyCode::W | KeyCode::Up => new_pos.y += 1,
            KeyCode::S | KeyCode::Down => new_pos.y -= 1,
            KeyCode::A | KeyCode::Left => new_pos.x -= 1,
            KeyCode::D | KeyCode::Right => new_pos.x += 1,
            _ => (),
        }
        keyboard.reset(key);
        turn_state.set(TurnState::PlayerTurn).unwrap();
    }

    if mb.map.can_enter_tile(new_pos) && new_pos != player.position {
        // let old_pos = player.position;
        player.position = new_pos;
        commands.entity(player_entity).insert(WantsToMove {
            destination: new_pos,
        });
    }
}
