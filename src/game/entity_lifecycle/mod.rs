mod hide_or_death;
mod spawn;

use crate::game::entity_lifecycle::hide_or_death::despawn_ball;
use crate::game::entity_lifecycle::spawn::spawn_or_reset_ball;
use crate::GameState;
use bevy::prelude::*;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Speed {
    // 每秒多移动的距离
    pub acceleration: f32,
    // 每秒移动的距离
    pub initial_speed: f32,
}

impl Default for Speed {
    fn default() -> Self {
        Self {
            acceleration: 3.0,
            initial_speed: 8.0,
        }
    }
}

pub(super) struct LifecyclePlugin;

impl Plugin for LifecyclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_or_reset_ball)
            .add_systems(OnExit(GameState::InGame), despawn_ball);
    }
}
