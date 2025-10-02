use bevy::app::App;
use bevy::prelude::*;
use crate::configs::PLAYER_SPEED;
use crate::state::GameState;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;


impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update,(handle_player_input).run_if(in_state(GameState::GameInit)));
    }
}

fn handle_player_input(
    mut player: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut transform = if let Ok(transform) = player.single_mut() {
        transform
    } else {
        return;
    };

    let mut delta = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        delta.y += 1.;
    }

    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        delta.y -= 1.;
    }

    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        delta.x -= 1.;
    }

    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        delta.x += 1.;
    }

    delta = delta.normalize();

    // 向量是否是有限的
    if delta.is_finite() {
        transform.translation += vec3(delta.x, delta.y, 0.) * PLAYER_SPEED;
    }
}