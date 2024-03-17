use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Drag(GridPos),
    Release,

    /// TODO_LOW: Press down and release a tile "quickly".
    _Tap(GridPos),

    /// TODO_LOW: Hold down a pushed tile for `self.0` seconds.
    _Hold(f32),
}

/// The sequence of moves input by the player for the current round.
#[derive(Resource, Debug, Default)]
pub struct InputMoves(pub Vec<Move>);

fn debug_log_input_moves(input_moves: Res<InputMoves>) {
    if input_moves.is_changed() {
        println!("input moves changed: {input_moves:?}");
    }
}

pub struct MovesPlugin;

impl Plugin for MovesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputMoves>();

        app.add_systems(Update, debug_log_input_moves);
    }
}
