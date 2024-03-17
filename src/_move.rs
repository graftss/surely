use bevy::prelude::*;

#[derive(Debug)]
pub struct GridPos {
    pub x: u32,
    pub y: u32,
}

impl GridPos {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
pub enum Move {
    Push(GridPos),
    DragTo(GridPos),
    Release,
    Tap(GridPos),
}

#[derive(Resource, Debug, Default)]
pub struct InputMoves {
    moves: Vec<Move>,
}

pub fn setup_moves_resource(mut commands: Commands) {
    commands.init_resource::<InputMoves>()
}
