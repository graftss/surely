use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PrimaryWindow,
};

/// The marker component for the parent of all grid tiles.
#[derive(Component)]
struct GridRoot;

/// The marker component for a grid tile.
/// The components are the x- and y- coordinates of the tile in the grid,
/// where the tile at (0, 0) is at the bottom left.
#[derive(Component, Debug)]
struct GridTile(u32, u32);

trait Collision {
    fn meets_point(&self, self_pos: Vec2, point: Vec2) -> bool;
}

/// Rectangular collision bounds.
/// The position of the entity is assumed to be the rectangle's center.
#[derive(Component)]
struct RectCollision {
    width: f32,
    height: f32,
}

impl Collision for RectCollision {
    fn meets_point(&self, self_pos: Vec2, point: Vec2) -> bool {
        let half_width = self.width / 2.0;
        let half_height = self.height / 2.0;

        point.x >= self_pos.x - half_width
            && point.x <= self_pos.x + half_width
            && point.y >= self_pos.y - half_height
            && point.y <= self_pos.y + half_height
    }
}

/// An "on/off switch" that records how long it's been on.
/// Expects to be updated every tick.
#[derive(Default, Debug)]
struct FlagTimer {
    pub on: bool,
    pub time_on: f32,
    pub ticks_on: u32,
}

impl FlagTimer {
    pub fn on(&mut self, time_on: f32) {
        self.on = true;
        self.time_on += time_on;
        self.ticks_on += 1;
    }

    pub fn off(&mut self) {
        self.on = false;
        self.time_on = 0.0;
        self.ticks_on = 0;
    }
}

#[derive(Default, Debug)]
struct MouseInteractState {
    pub hovered: FlagTimer,
    pub held: FlagTimer,
}

impl MouseInteractState {
    pub fn end_interaction(&mut self) {
        self.hovered.off();
        self.held.off();
    }
}

#[derive(Component, Default, Debug)]
struct InteractState {
    mouse: MouseInteractState,
}

/// The world position of the cursor.
///
#[derive(Resource, Default)]
struct CursorWorldPos(Option<Vec2>);

#[derive(Component)]
struct MainCamera;

fn update_cursor_world_pos(
    mut cursor_world_pos: ResMut<CursorWorldPos>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    cursor_world_pos.0 = window
        .cursor_position()
        .and_then(|window_pos| camera.viewport_to_world(camera_transform, window_pos))
        .map(|ray| ray.origin.truncate());
}

fn update_grid_tile_mouse_states(
    mut query: Query<(
        &GridTile,
        &RectCollision,
        &GlobalTransform,
        &mut InteractState,
    )>,
    time: Res<Time>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    cursor_world_pos: Res<CursorWorldPos>,
) {
    if let Some(cursor_pos) = cursor_world_pos.0 {
        for (grid_tile, collision, transform, mut interact_state) in query.iter_mut() {
            let tile_pos = transform.translation().truncate();
            if collision.meets_point(tile_pos, cursor_pos) {
                // mouse is over the grid tile at `tile_pos`
                interact_state.mouse.hovered.on(time.delta_seconds());

                if mouse_input.pressed(MouseButton::Left) {
                    println!("held tile: {:?} {:?}", grid_tile, interact_state);
                    interact_state.mouse.held.on(time.delta_seconds());
                } else {
                    println!("hovered tile: {:?} {:?}", grid_tile, interact_state);
                    interact_state.mouse.held.off();
                }
            } else {
                // mouse is not over the grid tile at `tile_pos`
                interact_state.mouse.end_interaction();
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<CursorWorldPos>()
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_grid)
        .add_systems(
            First,
            (update_cursor_world_pos, update_grid_tile_mouse_states).chain(),
        )
        .run();
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
        let mut pos_x = 0.0;

        for tile_x in 0..grid_width {
            let mut pos_y = 0.0;
            for tile_y in 0..grid_height {
                let rect_mesh = Mesh2dHandle(meshes.add(Rectangle::new(TILE_WIDTH, TILE_HEIGHT)));

                let mesh_bundle = MaterialMesh2dBundle {
                    mesh: rect_mesh,
                    material: materials.add(tile_color),
                    transform: Transform::from_xyz(pos_x, pos_y, 0.0),
                    ..default()
                };

                let rect_collision = RectCollision {
                    width: TILE_WIDTH,
                    height: TILE_HEIGHT,
                };

                parent.spawn((
                    GridTile(tile_x, tile_y),
                    rect_collision,
                    InteractState::default(),
                    mesh_bundle,
                ));

                pos_y += TILE_HEIGHT + TILE_PADDING_Y;
            }
            pos_x += TILE_WIDTH + TILE_PADDING_X;
        }
    });
}

fn setup(mut commands: Commands) {
    // add our marker to the default 2d camera
    commands.spawn((Camera2dBundle::default(), MainCamera));
}
