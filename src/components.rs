use crate::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Player {
    pub position: Point,
}

#[derive(Component)]
pub struct Enemy {
    pub position: Point,
}

#[derive(Component)]
pub struct MovingRandomly;

#[derive(Component)]
pub struct WantsToMove {
    pub destination: Point
}