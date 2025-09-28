use crate::game::camera::look_at::camera_follow_ball;
use crate::GameState;
use bevy::app::App;
use bevy::prelude::{in_state, IntoScheduleConfigs, Plugin, Update};

mod look_at;

pub(super) struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_follow_ball.run_if(in_state(GameState::InGame)));
    }
}
