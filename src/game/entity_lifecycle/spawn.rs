use crate::assets::{AnimationsResource, ImageAssets};
use crate::game::action::missile_attack::Missile;
use crate::game::action::AttackAction;
use crate::game::entity_lifecycle::{Mario, Speed};
use bevy::prelude::*;

/// 添加或初始化一个实体
pub(super) fn spawn_or_reset_ball(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Mario>>,
    scene_assets: Res<ImageAssets>,
    animations: Res<AnimationsResource>,
) {
    if let Ok((ball_entity, mut transform)) = query.single_mut() {
        commands.entity(ball_entity).insert(Visibility::Visible);
        *transform = Transform::from_xyz(0., 0., 0.);
        return;
    }

    let initial_anim = animations
        .get("right_small_normal_idle")
        .expect("找不到初始动画");

    commands.spawn((
        Mario,
        Sprite::from_atlas_image(
            scene_assets.role.clone(),
            TextureAtlas {
                layout: initial_anim.layout.clone(),
                index: initial_anim.first_index,
            },
        ),
        Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(1.)),
        Speed::default(),
        AttackAction,
        Missile {
            missile_position: Default::default(),
            missile_speed: 10.0,
        },
    ));
}
