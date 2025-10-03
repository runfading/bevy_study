use crate::game::player::Player;
use bevy::prelude::*;

pub(super) fn camera_follow_ball(
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    mut ball_transforms: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let _delta = time.delta_secs();

    let ball_transform = match ball_transforms.single_mut() {
        Ok(ball_transform) => ball_transform,
        Err(err) => {
            log::error!("没有获取到小球的坐标，原因{:?}", err);
            return;
        }
    };

    let mut camera = match camera_query.single_mut() {
        Ok(camera) => camera,
        Err(err) => {
            log::error!("没有获取到相机的坐标，原因{:?}", err);
            return;
        }
    };
    let target_position = ball_transform.translation;
    let camera_position = camera.translation;

    // lerp是平滑移动，当前场景可以去掉，因为小球是每帧移动，相机也是每帧更新
    // let new_x = camera_position.x.lerp(target_position.x, delta);
    // let new_y = camera_position.y.lerp(target_position.y, delta);
    let new_x = target_position.x;
    let new_y = target_position.y;

    camera.translation = Vec3::new(new_x, new_y, camera_position.z);
    *camera = camera.looking_at(target_position, Vec3::Y);
}
