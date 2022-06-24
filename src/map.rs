use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES]
        }
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

pub fn render_map(
    mut commands: Commands,
    tile_sheet: Res<FontSpriteSheet>,
    map: Res<Map>
) {
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let idx = map_idx(x, y);
            match map.tiles[idx] {
                TileType::Wall => {
                    commands
                        .spawn_bundle(SpriteSheetBundle {
                            sprite: TextureAtlasSprite {
                                index: to_cp437('#') as usize,
                                color: Color::GREEN,
                                ..TextureAtlasSprite::default()
                            },
                            texture_atlas: tile_sheet.atlas.clone(),
                            transform: Transform::from_xyz((x * TILE_SIZE) as f32, (y * TILE_SIZE) as f32, 0.0),
                            ..Default::default()
                        });
                },
                TileType::Floor => {
                    commands
                        .spawn_bundle(SpriteSheetBundle {
                            sprite: TextureAtlasSprite {
                                index: to_cp437('.') as usize,
                                color: Color::GREEN,
                                ..TextureAtlasSprite::default()
                            },
                            texture_atlas: tile_sheet.atlas.clone(),
                            transform: Transform::from_xyz((x * TILE_SIZE) as f32, (y * TILE_SIZE) as f32, 0.0),
                            ..Default::default()
                        });
                }
            }
        }
    }
}