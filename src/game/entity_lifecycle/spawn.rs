use crate::game::entity_lifecycle::{Ball, Speed};
use bevy::prelude::*;

/// 添加或初始化一个实体
pub(super) fn spawn_or_reset_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Ball,
        Mesh3d(meshes.add(Sphere::new(2.))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..Default::default()
        })),
        Transform::from_xyz(0., 0., 0.),
        Speed::default(),
    ));

    // MeshMaterial3d(materials.add(StandardMaterial {
    //     base_color: Color::WHITE,
    //     ..Default::default()
    // }));
}
