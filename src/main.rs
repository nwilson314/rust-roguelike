mod resources;

mod prelude {
    pub use bevy::prelude::*;
    pub use bracket_lib::prelude::*;
    pub const TILE_SIZE: u8 = 8;
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
        .run();
}

#[derive(Component)]
pub struct MainCamera;

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

    commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(MainCamera);
}

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    tile_sheet: Res<FontSpriteSheet>,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: to_cp437('@') as usize,
                ..TextureAtlasSprite::default()
            },
            texture_atlas: tile_sheet.atlas.clone(),
            ..Default::default()
        })
        .insert(Player);
}
