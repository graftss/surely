use _move::{GridPos, MovesPlugin};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use input::InputPlugin;

use crate::input::{InteractState, RectCollision};

mod _move;
mod input;

/// The marker component for the parent of all grid tiles.
#[derive(Component)]
struct GridRoot;

/// The marker component for a grid tile.
/// The components are the x- and y- coordinates of the tile in the grid,
/// where the tile at (0, 0) is at the bottom left.
#[derive(Component, Debug)]
struct GridTile(pub GridPos);

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((InputPlugin, MovesPlugin))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_grid)
        .run();
}

fn setup(mut commands: Commands) {
    // add our marker to the default 2d camera
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const TILE_WIDTH: f32 = 100.0;
    const TILE_HEIGHT: f32 = 100.0;
    const TILE_PADDING_X: f32 = 5.0;
    const TILE_PADDING_Y: f32 = 5.0;

    const TILE_COLLISION_WIDTH: f32 = TILE_WIDTH + TILE_PADDING_X;
    const TILE_COLLISION_HEIGHT: f32 = TILE_HEIGHT + TILE_PADDING_Y;
    const TILE_DELTA_X: f32 = TILE_WIDTH + TILE_PADDING_X;
    const TILE_DELTA_Y: f32 = TILE_HEIGHT + TILE_PADDING_Y;

    const GRID_WIDTH: u32 = 3;
    const GRID_HEIGHT: u32 = 3;

    let min_pos_x = -1.0 * (GRID_WIDTH - 1) as f32 * 0.5 * TILE_DELTA_X;
    let min_pos_y = -1.0 * (GRID_HEIGHT - 1) as f32 * 0.5 * TILE_DELTA_Y;

    let tile_color = Color::hsl(200.0, 0.5, 0.4);

    let grid_root = (
        GridRoot,
        SpatialBundle {
            visibility: Visibility::Visible,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    );

    commands.spawn(grid_root).with_children(|parent| {
        let mut pos_x = min_pos_x;

        for tile_x in 0..GRID_WIDTH {
            let mut pos_y = min_pos_y;

            for tile_y in 0..GRID_HEIGHT {
                let rect_mesh = Mesh2dHandle(meshes.add(Rectangle::new(TILE_WIDTH, TILE_HEIGHT)));

                let mesh_bundle = MaterialMesh2dBundle {
                    mesh: rect_mesh,
                    material: materials.add(tile_color),
                    transform: Transform::from_xyz(pos_x, pos_y, 0.0),
                    ..default()
                };

                let rect_collision =
                    RectCollision::new(TILE_COLLISION_WIDTH, TILE_COLLISION_HEIGHT);

                parent.spawn((
                    GridTile(GridPos::new(tile_x, tile_y)),
                    rect_collision,
                    InteractState::default(),
                    mesh_bundle,
                ));

                pos_y += TILE_DELTA_Y;
            }

            pos_x += TILE_DELTA_X;
        }
    });
}
