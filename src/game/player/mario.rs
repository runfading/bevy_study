use crate::assets::{AnimationsResource, ImageAssets};
use crate::game::action::mario_movement::KeyBoardMovementAction;
use crate::game::action::{AttackAction, Speed};
use crate::game::animation;
use crate::game::animation::{AnimationLastIndex, AnimationTimer};
use crate::game::player::{Player, PlayerActonState, PlayerFormState, PlayerSizeState};
use bevy::prelude::*;

/// 添加或初始化一个实体
pub(super) fn spawn_or_reset_ball(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &mut PlayerActonState,
            &mut PlayerSizeState,
            &mut PlayerFormState,
        ),
        With<Player>,
    >,
    scene_assets: Res<ImageAssets>,
    animations: Res<AnimationsResource>,
) {
    let default_action_state = PlayerActonState::default();
    let default_size_state = PlayerSizeState::default();
    let default_form_state = PlayerFormState::default();

    if let Ok((ball_entity, mut transform, mut action_state, mut size_state, mut form_state)) =
        query.single_mut()
    {
        commands.entity(ball_entity).insert(Visibility::Visible);
        *transform = Transform::from_xyz(0., 0., 0.);
        *action_state = default_action_state;
        *size_state = default_size_state;
        *form_state = default_form_state;
        return;
    }

    // 获取初始动画名称
    let animation_name = animation::utils::get_player_animation_name(
        &default_action_state,
        &default_size_state,
        &default_form_state,
    );
    // 取得初始动画
    let mario_animation_data = animations
        .get(animation_name.as_str())
        .expect("找不到mario初始动画");

    commands.spawn((
        Player,
        Sprite::from_atlas_image(
            scene_assets.role.clone(),
            TextureAtlas {
                layout: mario_animation_data.layout.clone(),
                index: mario_animation_data.first_index,
            },
        ),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)).with_scale(Vec3::splat(10.0)),
        Speed::default(),
        AttackAction,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        AnimationLastIndex(mario_animation_data.last_index + 1),
        KeyBoardMovementAction,
        default_action_state,
        default_size_state,
        default_form_state,
    ));
}
