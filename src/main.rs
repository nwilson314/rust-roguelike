mod components;
mod helper;
mod map;
mod map_builder;
mod player;
mod resources;

mod prelude {
    pub use bevy::prelude::*;
    pub use bracket_lib::prelude::*;
    pub const TILE_SIZE: i32 = 8;
    pub const NUM_TILES_WIDTH: i32 = 80;
    pub const NUM_TILES_HEIGHT: i32 = 50;
    pub const SCREEN_WIDTH: f32 = 1600.0;
    pub const SCREEN_HEIGHT: f32 = 1000.0;
    pub use crate::components::*;
    pub use crate::helper::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
    pub use crate::resources::*;
}

use prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Rust Roguelike".to_string(),
            width: 1600.0,
            height: 1000.0,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system_to_stage(StartupStage::PreStartup, setup.label("setup"))
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_map)
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

    commands.insert_resource(MapBuilder::new());

    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.scale = 1.0 / 2.0;

    commands.spawn_bundle(camera).insert(MainCamera);
}
