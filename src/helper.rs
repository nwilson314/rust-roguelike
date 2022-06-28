use crate::prelude::*;

pub fn get_windowed_tile_size() -> (f32, f32) {
    // Returns the (width, height) of an actual game tile based on the
    // desired number of tiles and the screen's width and height
    (
        SCREEN_WIDTH / (NUM_TILES_WIDTH as f32),
        SCREEN_HEIGHT / (NUM_TILES_HEIGHT as f32),
    )
}

pub fn convert_pos(x: i32, y: i32) -> (f32, f32) {
    // Converts an x and y cooridinate into pixel values factoring in
    // Bevy's camera (0,0) position is in the middle of the screen.
    // Here, a coordinate of (0,0) is in the bottom right
    let (tile_width, tile_height) = get_windowed_tile_size();

    let new_x = x as f32 * tile_width - (SCREEN_WIDTH / 2.0) + tile_width / 2.0;
    let new_y = y as f32 * tile_height - (SCREEN_HEIGHT / 2.0) + tile_height / 2.0;

    (new_x, new_y)
}
