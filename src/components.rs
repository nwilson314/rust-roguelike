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
