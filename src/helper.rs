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

/// Get the world position of the cursor in 2D space
pub fn cursor_to_world(
    windows: &Windows,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Vec2 {
    let window = windows.get_primary().unwrap();

    let pos = window.cursor_position().unwrap_or_default();
    // get the size of the window
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);

    // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
    let ndc = (pos / window_size) * 2.0 - Vec2::ONE;

    // matrix for undoing the projection and camera transform
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

    // use it to convert ndc to world-space coordinates
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

    // reduce it to a 2D value
    let world_pos: Vec2 = world_pos.truncate();
    Vec2::new(world_pos.x, world_pos.y)
}
