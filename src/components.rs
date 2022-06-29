use crate::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Position {
    pub position: Point
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct MovingRandomly;

#[derive(Component)]
pub struct WantsToMove {
    pub destination: Point,
}

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}
