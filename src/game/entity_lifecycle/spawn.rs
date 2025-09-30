use crate::asset_loader::SceneAssets;
use crate::game::action::missile_attack::Missile;
use crate::game::action::AttackAction;
use crate::game::entity_lifecycle::{Ball, Speed};
use bevy::prelude::*;

/// 添加或初始化一个实体
pub(super) fn spawn_or_reset_ball(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Ball>>,
    scene_assets: Res<SceneAssets>,
) {
    if let Ok((ball_entity, mut transform)) = query.single_mut() {
        commands.entity(ball_entity).insert(Visibility::Visible);
        *transform = Transform::from_xyz(0., 0., 0.);
        return;
    }

    commands.spawn((
        Ball,
        SceneRoot(scene_assets.spaceship.clone()),
        Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(1.)),
        Speed::default(),
        AttackAction,
        Missile {
            missile_position: Default::default(),
            missile_speed: 10.0,
        },
    ));
}
