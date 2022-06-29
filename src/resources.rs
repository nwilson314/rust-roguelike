use crate::prelude::*;

pub struct FontSpriteSheet {
    pub atlas: Handle<TextureAtlas>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}
