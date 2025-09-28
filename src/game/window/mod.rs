mod bg;

use crate::game::window::bg::*;
use crate::GameState;
use bevy::app::App;
use bevy::prelude::*;

#[derive(Component)]
pub(self) struct Star;

#[derive(Component)]
pub(self) struct StarDrift {
    velocity: Vec2,
}

pub(super) struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGame),
            (setup_game_background, spawn_game_main_scene),
        )
        .add_systems(
            OnExit(GameState::InGame),
            (restore_menu_background, hide_spawn_game_main_scene),
        )
        .add_systems(
            Update,
            (wrap_stars_around_camera, drift_stars).run_if(in_state(GameState::InGame)),
        );
    }
}

// fn compute_camera_view_bounds(
//     windows: Query<&Window>,
//     cameras: Query<(&Transform, &Camera, &Projection), With<Camera3d>>,
// ) {
//     let Ok((transform, camera, projection)) = cameras.get_single() else {
//         return;
//     };
//
//     if let Projection::Perspective(persp) = projection {
//         let fov = persp.fov; // 默认 60°
//         let aspect = camera.aspect_ratio().unwrap_or(16.0 / 9.0);
//
//         // 相机在 z=100，看向 z=0 → 距离 d = 100
//         let d = transform.translation.z.abs();
//
//         let height = 2.0 * d * (fov * 0.5).tan();
//         let width = height * aspect;
//
//         info!(
//             "相机在 z=0 平面看到的范围：width = {}, height = {}",
//             width, height
//         );
//     }
// }
