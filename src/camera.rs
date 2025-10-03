use crate::player::Player;
use crate::state::GameState;
use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};

pub struct FollowCameraPlugin;

impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin::default())
            .add_systems(OnEnter(GameState::Loading), setup_camera)
            .add_systems(
                Update,
                camera_follow_player.run_if(in_state(GameState::InGame)),
            );
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Msaa::Off,
        PanCam {
            grab_buttons: vec![],
            ..default()
        },
    ));
}

fn camera_follow_player(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let mut camera_transform = if let Ok(camera_transform) = camera_query.single_mut() {
        camera_transform
    } else {
        return;
    };

    let player_pos = if let Ok(player_transform) = player_query.single() {
        player_transform.translation
    } else {
        return;
    };

    camera_transform.translation = camera_transform
        .translation
        .lerp(vec3(player_pos.x, player_pos.y, 0.), 0.1);
}
