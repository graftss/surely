use bevy::{prelude::*, window::PrimaryWindow};

use crate::{GridTile, MainCamera};

trait Collision {
    fn meets_point(&self, self_pos: Vec2, point: Vec2) -> bool;
}

/// Rectangular collision bounds.
/// The position of the entity is assumed to be the rectangle's center.
#[derive(Component)]
pub struct RectCollision {
    width: f32,
    height: f32,
}

impl RectCollision {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
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
pub struct InteractState {
    mouse: MouseInteractState,
}

/// The world position of the cursor.
#[derive(Resource, Default)]
struct CursorWorldPos(Option<Vec2>);

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
        &Handle<ColorMaterial>,
        &mut InteractState,
    )>,
    time: Res<Time>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    cursor_world_pos: Res<CursorWorldPos>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Some(cursor_pos) = cursor_world_pos.0 {
        for (_, collision, transform, mat_handle, mut interact_state) in query.iter_mut() {
            let tile_pos = transform.translation().truncate();

            if collision.meets_point(tile_pos, cursor_pos) {
                // mouse is over the grid tile at `tile_pos`
                interact_state.mouse.hovered.on(time.delta_seconds());

                if mouse_input.pressed(MouseButton::Left) {
                    interact_state.mouse.held.on(time.delta_seconds());

                    color_materials
                        .get_mut(mat_handle)
                        .map(|color_material| color_material.color = Color::RED);
                } else {
                    interact_state.mouse.held.off();

                    color_materials
                        .get_mut(mat_handle)
                        .map(|color_material| color_material.color = Color::MIDNIGHT_BLUE);
                }
            } else {
                // mouse is not over the grid tile at `tile_pos`
                interact_state.mouse.end_interaction();

                color_materials
                    .get_mut(mat_handle)
                    .map(|color_material| color_material.color = Color::hsl(200.0, 0.5, 0.4));
            }
        }
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorWorldPos>();

        app.add_systems(
            First,
            (update_cursor_world_pos, update_grid_tile_mouse_states).chain(),
        );
    }
}
