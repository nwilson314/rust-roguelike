use crate::prelude::*;

pub fn spawn_player(mut commands: Commands, tile_sheet: Res<FontSpriteSheet>, mb: Res<MapBuilder>) {
    let (tile_width, tile_height) = get_windowed_tile_size();
    let (pos_x, pos_y) = convert_pos(mb.player_start.x, mb.player_start.y);
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: to_cp437('@') as usize,
                custom_size: Some(Vec2::new(tile_width, tile_height)),
                ..TextureAtlasSprite::default()
            },
            texture_atlas: tile_sheet.atlas.clone(),
            transform: Transform::from_xyz(pos_x, pos_y, 1.0),
            ..Default::default()
        })
        .insert(Player)
        .insert(Position {
            position: mb.player_start,
        })
        .insert(Health {
            current: 10,
            max: 20,
        });
}

pub fn spawn_monsters(
    mut commands: Commands,
    tile_sheet: Res<FontSpriteSheet>,
    mb: Res<MapBuilder>,
) {
    let mut rng = RandomNumberGenerator::new();
    for room in mb.rooms.iter().skip(1) {
        let center = room.center();
        let (index, color) = match rng.range(0, 4) {
            0 => (to_cp437('E') as usize, Color::RED),
            1 => (to_cp437('O') as usize, Color::ORANGE_RED),
            2 => (to_cp437('o') as usize, Color::ORANGE),
            _ => (to_cp437('g') as usize, Color::YELLOW),
        };

        spawn_monster(&mut commands, &tile_sheet, index, color, center);
    }
}

pub fn spawn_monster(
    commands: &mut Commands,
    tile_sheet: &Res<FontSpriteSheet>,
    index: usize,
    color: Color,
    pos: Point,
) {
    let (tile_width, tile_height) = get_windowed_tile_size();
    let (pos_x, pos_y) = convert_pos(pos.x, pos.y);
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: index,
                custom_size: Some(Vec2::new(tile_width, tile_height)),
                color: color,
                ..TextureAtlasSprite::default()
            },
            texture_atlas: tile_sheet.atlas.clone(),
            transform: Transform::from_xyz(pos_x, pos_y, 1.0),
            ..Default::default()
        })
        .insert(Enemy)
        .insert(Position { position: pos })
        .insert(MovingRandomly);
}
