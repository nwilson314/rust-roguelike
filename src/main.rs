mod resources;

mod prelude {
    pub use bevy::prelude::*;
    pub const TILE_SIZE: u8 = 8;
    pub use crate::resources::*;
}

use prelude::*;

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("fontfile.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::splat(TILE_SIZE as f32), 16, 16);
    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(SpriteSheet {
        atlas: atlas_handle,
    });
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Rust Roguelike".to_string(),
            ..Default::default()
        })
        .add_startup_system_to_stage(StartupStage::PreStartup, setup.label("setup"))
        .add_plugins(DefaultPlugins)
        .run();
}
