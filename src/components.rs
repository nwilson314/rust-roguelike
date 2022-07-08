use crate::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Position {
    pub position: Point,
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

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Default for Health {
    fn default() -> Self {
        Self { current: 1, max: 1 }
    }
}

// impl std::fmt::Display for Health {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{}/{}", self.current, self.max)
//     }
// }

#[derive(Clone, PartialEq, Debug)]
pub struct ToolTipPos {
    pub pos: Vec2,
    pub color: Color,
    pub info: ToolTipInfo,
}

impl Default for ToolTipPos {
    fn default() -> Self {
        Self {
            pos: Vec2::new(0.0, 0.0),
            color: Color::WHITE,
            info: ToolTipInfo::default(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ToolTipInfo {
    pub name: Name,
    pub health: Health,
}

impl Default for ToolTipInfo {
    fn default() -> Self {
        Self {
            name: Name::new(""),
            health: Health::default(),
        }
    }
}
