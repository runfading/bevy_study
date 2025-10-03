mod player;
pub(super) mod utils;

use crate::game::animation::player::PlayerAnimationPlugin;
use crate::GameState;
use bevy::prelude::*;

pub(super) struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerAnimationPlugin).add_systems(
            Update,
            tick_animation_timer.run_if(in_state(GameState::InGame)),
        );
    }
}

// 动画定时器
#[derive(Component)]
pub(crate) struct AnimationTimer(pub Timer);

// 动画循环，最后一个index
#[derive(Component)]
pub(crate) struct AnimationLastIndex(pub usize);

/// 触发下一个动画所需要的时间
fn tick_animation_timer(
    time: Res<Time>,
    mut animation_timer_query: Query<&mut AnimationTimer, With<AnimationTimer>>,
) {
    animation_timer_query.iter_mut().for_each(|mut timer| {
        timer.0.tick(time.delta());
    })
}
