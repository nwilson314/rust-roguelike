use crate::prelude::*;

mod camera_follow;
mod collisions;
mod end_turn;
mod movement;
mod player_input;
mod random_move;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(TurnState::AwaitingInput)
            .add_plugin(AwaitingInputPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(MonsterPlugin);
    }
}

struct AwaitingInputPlugin;
impl Plugin for AwaitingInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(TurnState::AwaitingInput)
                .label("awaiting_input")
                .with_system(player_input::get_input)
                .with_system(camera_follow::camera_follow),
        );
    }
}

struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(TurnState::PlayerTurn)
                .label("player_turn")
                .with_system(movement::movement)
                .with_system(collisions::collisions)
                .with_system(end_turn::end_turn),
        );
    }
}

struct MonsterPlugin;
impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(TurnState::MonsterTurn)
                .label("monster_turn")
                .with_system(random_move::random_move)
                .with_system(movement::movement)
                .with_system(end_turn::end_turn),
        );
    }
}
