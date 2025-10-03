use crate::game::action::Speed;
use crate::game::player::{PlayerActonState, PlayerStateChange};
use bevy::input::ButtonInput;
use bevy::prelude::*;

/// 使用键盘操作移动
#[derive(Component)]
pub(crate) struct KeyBoardMovementAction;

/// 同时具有移动、速度插件
pub(super) fn movement(
    mut query: Query<
        (&mut Transform, &Speed, Option<&mut PlayerActonState>),
        (With<KeyBoardMovementAction>, With<Speed>, With<Transform>),
    >,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_state_change_msg: MessageWriter<PlayerStateChange>,
) {
    let (mut transform, speed, action_state) = if let Ok(entity) = query.single_mut() {
        entity
    } else {
        return;
    };

    let mut direction = Vec3::ZERO;

    // 收集输入方向（用 1.0，不是 initial_speed）
    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    direction = direction.normalize();

    // 计算当前速度
    let current_speed = if keyboard_input.pressed(KeyCode::Space) {
        speed.initial_speed + speed.acceleration // 或者 speed.initial_speed * 1.5 作为倍率
    } else {
        speed.initial_speed
    };

    // 更新行走站立状态
    if let Some(mut action_state) = action_state {
        let new_state = if direction.is_finite() {
            PlayerActonState::Walk
        } else {
            PlayerActonState::Idle
        };

        if *action_state != new_state {
            *action_state = new_state;
            player_state_change_msg.write(PlayerStateChange);
        }
    }

    // 应用移动（仅当方向有效）
    if direction.is_finite() {
        transform.translation += direction * current_speed;
    }
}
