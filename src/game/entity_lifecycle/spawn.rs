use crate::asset_loader::SceneAssets;
use crate::game::entity_lifecycle::{Ball, Speed};
use bevy::prelude::*;

/// 添加或初始化一个实体
pub(super) fn spawn_or_reset_ball(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(Entity, &mut Transform), With<Ball>>,
    gltf: Res<Assets<Gltf>>,
    scene_assets: Res<SceneAssets>,
) {
    if let Ok((ball_entity, mut transform)) = query.single_mut() {
        commands.entity(ball_entity).insert(Visibility::Visible);
        *transform = Transform::from_xyz(0., 0., 0.);
        return;
    }

    commands.spawn((
        Ball,
        // Mesh3d(meshes.add(Sphere::new(2.))),
        // MeshMaterial3d(materials.add(StandardMaterial {
        //     base_color: Color::WHITE,
        //     ..Default::default()
        // })),
        SceneRoot(gltf.get(&scene_assets.spaceship).unwrap().scenes[0].clone()),
        Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(1.)),
        Speed::default(),
    ));

    // MeshMaterial3d(materials.add(StandardMaterial {
    //     base_color: Color::WHITE,
    //     ..Default::default()
    // }));
}
