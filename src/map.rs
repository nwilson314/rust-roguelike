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
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn in_bounds(&self, pos: Point) -> bool {
        pos.x >= 0 && pos.x < NUM_TILES_WIDTH && pos.y >= 0 && pos.y < NUM_TILES_HEIGHT
    }

    pub fn can_enter_tile(&self, pos: Point) -> bool {
        self.in_bounds(pos) && self.tiles[map_idx(pos.x, pos.y)] == TileType::Floor
    }

    pub fn try_idx(&self, pos: Point) -> Option<usize> {
        if !self.in_bounds(pos) {
            None
        } else {
            Some(map_idx(pos.x, pos.y))
        }
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * NUM_TILES_WIDTH) + x) as usize
}

pub fn spawn_map(mut commands: Commands, tile_sheet: Res<FontSpriteSheet>, mb: Res<MapBuilder>) {
    let map = &mb.map;
    for y in 0..NUM_TILES_HEIGHT {
        for x in 0..NUM_TILES_WIDTH {
            let idx = map_idx(x, y);
            let (pos_x, pos_y) = convert_pos(x, y);
            let (tile_width, tile_height) = get_windowed_tile_size();
            match map.tiles[idx] {
                TileType::Wall => {
                    commands
                        .spawn_bundle(SpriteSheetBundle {
                            sprite: TextureAtlasSprite {
                                index: to_cp437('#') as usize,
                                color: Color::GRAY,
                                custom_size: Some(Vec2::new(tile_width, tile_height)),
                                ..TextureAtlasSprite::default()
                            },
                            transform: Transform::from_xyz(pos_x, pos_y, 0.0),
                            texture_atlas: tile_sheet.atlas.clone(),
                            ..Default::default()
                        })
                        .insert(Position {
                            position: Point::new(x, y),
                        });
                }
                TileType::Floor => {
                    commands
                        .spawn_bundle(SpriteSheetBundle {
                            sprite: TextureAtlasSprite {
                                index: to_cp437('.') as usize,
                                color: Color::DARK_GRAY,
                                custom_size: Some(Vec2::new(tile_width, tile_height)),
                                ..TextureAtlasSprite::default()
                            },
                            texture_atlas: tile_sheet.atlas.clone(),
                            transform: Transform::from_xyz(pos_x, pos_y, 0.0),
                            ..Default::default()
                        })
                        .insert(Position {
                            position: Point::new(x, y),
                        });
                }
            }
        }
    }
}
