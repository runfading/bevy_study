use crate::game::action::mario_movement::movement;
use crate::game::action::missile_attack::MissilePlugin;
use crate::GameState;
use bevy::app::App;
use bevy::prelude::{in_state, Component, IntoScheduleConfigs, Plugin, Update};

pub(super) mod mario_movement;
pub(super) mod missile_attack;

/// 攻击模组
#[derive(Component)]
pub(super) struct AttackAction;

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
            acceleration: 10.0,
            initial_speed: 30.0,
        }
    }
}

pub(super) struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement.run_if(in_state(GameState::InGame)))
            .add_plugins(MissilePlugin);
    }
}
