mod components;
mod player;
mod resources;

mod prelude {
    pub use bevy::prelude::*;
    pub use bracket_lib::prelude::*;
    pub const TILE_SIZE: u8 = 8;
    pub use crate::components::*;
    pub use crate::player::*;
    pub use crate::resources::*;
}

use prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Rust Roguelike".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system_to_stage(StartupStage::PreStartup, setup.label("setup"))
        .add_startup_system(spawn_player)
        .add_system(get_input)
        .run();
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("fontfile_transparent.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::splat(TILE_SIZE as f32), 16, 16);
    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(FontSpriteSheet {
        atlas: atlas_handle,
    });

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}
