// use crate::camera::SCALE;
// use bevy::prelude::*;
//
// // 标记需要像素对齐的实体
// #[derive(Component)]
// pub(crate) struct PixelPerfect;
//
// // 系统：将所有标记为 PixelPerfect 的实体对齐到像素网格
// pub(super) fn snap_to_pixel_grid(mut query: Query<&mut Transform, With<PixelPerfect>>) {
//     for mut transform in &mut query {
//         let pos = transform.translation;
//
//         // 将世界坐标对齐到像素网格（考虑 SCALE）
//         let snapped_x = (pos.x / SCALE).round() * SCALE;
//         let snapped_y = (pos.y / SCALE).round() * SCALE;
//
//         transform.translation.x = snapped_x;
//         transform.translation.y = snapped_y;
//     }
// }
