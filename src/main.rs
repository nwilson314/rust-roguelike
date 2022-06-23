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
        .add_system(get_input)
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

pub fn get_input(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard: Res<bevy::prelude::Input<KeyCode>>, 
) {
    let mut player_transform = player_query.single_mut();

    if keyboard.just_pressed(KeyCode::W) | keyboard.just_pressed(KeyCode::Up) {
        player_transform.translation.y += TILE_SIZE as f32;
    } else if keyboard.just_pressed(KeyCode::S) | keyboard.just_pressed(KeyCode::Down) {
        player_transform.translation.y -= TILE_SIZE as f32;
    } else if keyboard.just_pressed(KeyCode::A) | keyboard.just_pressed(KeyCode::Left) {
        player_transform.translation.x -= TILE_SIZE as f32;
    } else if keyboard.just_pressed(KeyCode::D) | keyboard.just_pressed(KeyCode::Right) {
        player_transform.translation.x += TILE_SIZE as f32;
    }
}
