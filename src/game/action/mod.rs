use crate::game::action::missile_attack::{Missile, MissilePlugin};
use crate::game::action::movement::movement;
use crate::GameState;
use bevy::app::App;
use bevy::prelude::{in_state, Component, IntoScheduleConfigs, Plugin, Update};

pub(super) mod missile_attack;
pub(super) mod movement;

/// 攻击模组
#[derive(Component)]
pub(super) struct AttackAction;

pub(super) struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement.run_if(in_state(GameState::InGame)))
            .add_plugins(MissilePlugin);
    }
}
