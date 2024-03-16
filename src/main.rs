use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_grid)
        .run();
}

/// The marker component for the parent of all grid tiles.
#[derive(Component)]
struct GridRoot;

fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const TILE_WIDTH: f32 = 100.0;
    const TILE_HEIGHT: f32 = 100.0;
    const TILE_PADDING_X: f32 = 5.0;
    const TILE_PADDING_Y: f32 = 5.0;

    let grid_width = 3;
    let grid_height = 3;

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
        let mut x = 0.0;

        for _ in 0..grid_width {
            let mut y = 0.0;
            for _ in 0..grid_height {
                let rect_mesh = Mesh2dHandle(meshes.add(Rectangle::new(TILE_WIDTH, TILE_HEIGHT)));
                parent.spawn(MaterialMesh2dBundle {
                    mesh: rect_mesh,
                    material: materials.add(tile_color),
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                });

                y += TILE_HEIGHT + TILE_PADDING_Y;
            }
            x += TILE_WIDTH + TILE_PADDING_X;
        }
    });
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
