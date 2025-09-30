use bevy::input::mouse::MouseWheel;
use bevy::prelude::{Camera3d, EventReader, Query, Transform, With};

pub(super) fn zoom_camera(
    mut mouse_wheel: EventReader<MouseWheel>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    let mut total_delta = 0.0;

    // 收集所有滚轮事件
    for event in mouse_wheel.read() {
        total_delta += event.y;
    }

    if total_delta != 0.0 {
        for mut transform in query.iter_mut() {
            // 缩放因子，可以根据需要调整
            let zoom_speed = 5.0;
            let zoom_delta = total_delta * zoom_speed;

            // 限制相机的最小和最大距离
            let min_distance = 50.0;
            let max_distance = 200.0;

            // 获取当前相机位置
            let current_position = transform.translation;
            let distance_from_origin = current_position.y;

            // 计算新的距离
            let new_distance =
                (distance_from_origin - zoom_delta).clamp(min_distance, max_distance);

            // 更新相机位置
            transform.translation.y = new_distance;
        }
    }
}
