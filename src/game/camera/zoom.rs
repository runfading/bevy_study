use bevy::input::mouse::MouseWheel;
use bevy::prelude::{Camera2d, MessageReader, Projection, Query, With};
use std::ops::DerefMut;

pub(super) fn zoom_camera(
    mut mouse_wheel: MessageReader<MouseWheel>,
    mut query: Query<&mut Projection, With<Camera2d>>,
) {
    let mut total_delta = 0.0;

    // 收集所有滚轮事件
    for event in mouse_wheel.read() {
        total_delta += event.y;
    }

    if total_delta != 0.0 {
        for mut projection in query.iter_mut() {
            // 如果是正交投影进行改变
            if let Projection::Orthographic(projection) = projection.deref_mut() {
                if total_delta > 0.0 {
                    projection.scale += 1.;
                } else {
                    projection.scale -= 1.;
                }
            }
        }
    }
}
