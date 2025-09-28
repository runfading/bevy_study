use crate::game::entity_lifecycle::{Ball, Speed};
use bevy::input::ButtonInput;
use bevy::prelude::*;

pub(super) fn movement(
    mut query: Query<(&mut Transform, &Speed), With<Ball>>, // 注意：Speed 不需要 mut，除非你改它
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, speed) = if let Ok(entity) = query.single_mut() {
        entity
    } else {
        return;
    };

    let delta = time.delta_secs();
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

    // 归一化方向
    if direction.length_squared() > f32::EPSILON {
        direction = direction.normalize();
    } else {
        return; // 没有输入，不移动
    }

    // 计算当前速度
    let current_speed = if keyboard_input.pressed(KeyCode::Space) {
        speed.initial_speed + speed.acceleration // 或者 speed.initial_speed * 1.5 作为倍率
    } else {
        speed.initial_speed
    };

    // 应用移动
    let move_vec = direction * current_speed * delta;
    transform.translation += move_vec;
    log::info!("小球坐标{:?}", transform);
}
