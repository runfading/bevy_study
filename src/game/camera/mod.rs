use crate::game::camera::look_at::camera_follow_ball;
use crate::GameState;
use bevy::app::App;
use bevy::prelude::{in_state, IntoScheduleConfigs, Plugin, Update};
use crate::game::camera::zoom::zoom_camera;

mod look_at;
mod zoom;

pub(super) struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (camera_follow_ball,zoom_camera).run_if(in_state(GameState::InGame)));
    }
}
