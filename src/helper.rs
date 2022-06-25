use crate::prelude::*;

pub fn get_windowed_tile_size() -> (f32, f32) {
    (SCREEN_WIDTH / (NUM_TILES_WIDTH as f32), SCREEN_HEIGHT / (NUM_TILES_HEIGHT as f32))
}

pub fn convert_pos(x: i32, y: i32) -> (f32, f32) {
    let (tile_width, tile_height) = get_windowed_tile_size();

    let new_x = x as f32 * tile_width - (SCREEN_WIDTH / 2.0) + tile_width / 2.0;
    let new_y = y as f32 * tile_height - (SCREEN_HEIGHT / 2.0) + tile_height / 2.0;

    (new_x, new_y)
}

pub fn convert_movement(x: i32, y: i32) -> (f32, f32) {
    let (tile_width, tile_height) = get_windowed_tile_size();
    (tile_width * x as f32, tile_height * y as f32)
}